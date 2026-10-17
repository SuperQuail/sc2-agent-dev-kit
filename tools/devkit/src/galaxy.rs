//! 从生成的 `MapScript.galaxy` 里找出单位生成点，以及替换创建调用。
//!
//! **开发时**别手改它（编辑器保存会重新生成）；但**产出交付物**时**必须**改它：
//! 游戏运行时执行的只有它，`Triggers` 是编辑器的源。只改源 = 进游戏毫无变化。
use anyhow::{Context, Result};
use std::path::Path;

/// 一处 `UnitCreate(数量, 单位, 0, 玩家, 点, 朝向)` 调用。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Spawn {
    /// 第 1 个实参，原文（可能是字面量也可能是变量名）
    pub count: String,
    /// 第 2 个实参：单位 id。参数化的调用这里是变量名。
    pub unit: String,
    /// 第 4 个实参：玩家表达式
    pub player: String,
    /// 1-based 行号
    pub line: usize,
    /// 该行原文（已 trim）
    pub text: String,
}

/// 把**创建调用**里的单位字面量 `"from"` 换成 `"to"`，返回新文本与改动处数。
///
/// 只动创建类调用（`CreateUnits*` / `UnitCreate(` / `UnitCargoCreate`），
/// 不碰 `UnitGroup("Marine", ...)` 这种筛选/统计用法 —— 那些改了会出别的错。
///
/// `scope_fn` 给了就只改该函数体内：触发器显示名去掉空格就是函数名
/// （`Intro Cinematic` -> `gt_IntroCinematic_Func`）。
pub fn replace_created_unit(src: &str, from: &str, to: &str, scope_fn: Option<&str>) -> (String, usize) {
    let old = format!("\"{from}\"");
    let new = format!("\"{to}\"");
    let sig = scope_fn.map(|f| format!("bool {f} ("));
    let mut out = String::with_capacity(src.len());
    let (mut n, mut inside, mut depth) = (0usize, scope_fn.is_none(), 0i32);

    for line in src.split_inclusive('\n') {
        if let Some(sig) = &sig {
            if line.contains(sig.as_str()) {
                inside = true;
                depth = 0;
            }
        }
        let creates = line.contains("CreateUnits")
            || line.contains("UnitCreate(")
            || line.contains("UnitCargoCreate");
        if inside && creates && line.contains(&old) {
            out.push_str(&line.replace(&old, &new));
            n += 1;
        } else {
            out.push_str(line);
        }
        if sig.is_some() && inside {
            depth += line.matches('{').count() as i32 - line.matches('}').count() as i32;
            if depth <= 0 && line.contains('}') {
                inside = false;
            }
        }
    }
    (out, n)
}

/// 在某个触发器函数的**动作段开头**插入几行 Galaxy 代码。
///
/// 插在 `if (!runActions) { return true; }` 之后 —— 这是编辑器生成的动作段入口。
/// 产物侧必须这么改：游戏跑的是这个文件，`Triggers` 只是编辑器的源。
///
/// 返回 (新文本, 是否插进去了)。
pub fn inject_into_fn(src: &str, fn_name: &str, code: &[&str]) -> (String, bool) {
    let sig = format!("bool {fn_name} (");
    let lines: Vec<&str> = src.split_inclusive('\n').collect();
    let Some(start) = lines.iter().position(|l| l.contains(sig.as_str())) else {
        return (src.to_string(), false);
    };
    // 守卫长这样：`if (!runActions) {` / `return true;` / `}` —— 插在收尾那个 `}` 之后
    let mut at = None;
    if let Some(g) = lines
        .iter()
        .enumerate()
        .skip(start)
        .find(|(i, l)| *i > start && l.contains("(!runActions)"))
        .map(|(i, _)| i)
    {
        at = lines
            .iter()
            .enumerate()
            .skip(g)
            .find(|(_, l)| l.trim() == "}")
            .map(|(i, _)| i);
    }
    let Some(at) = at else {
        return (src.to_string(), false);
    };
    let mut out = String::with_capacity(src.len());
    for (i, l) in lines.iter().enumerate() {
        out.push_str(l);
        if i == at {
            for c in code {
                out.push_str("    ");
                out.push_str(c);
                out.push('\n');
            }
        }
    }
    (out, true)
}

/// 扫一个 .galaxy 文件里的 UnitCreate 调用。
pub fn spawns(galaxy: &Path) -> Result<Vec<Spawn>> {
    let src = std::fs::read_to_string(galaxy)
        .with_context(|| format!("读不了 {}", galaxy.display()))?;

    let mut out = Vec::new();
    let bytes = src.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        // 注释与字符串字面量里的 UnitCreate 不算调用，整段跳过。
        if let Some(j) = skip_comment(bytes, i) {
            i = j;
            continue;
        }
        if bytes[i] == b'"' {
            i = skip_string(bytes, i);
            continue;
        }
        // 前面必须是标识符边界，否则 libNtve_gf_UnitCreateFacingPoint 会假命中。
        let boundary = i == 0 || !(bytes[i - 1].is_ascii_alphanumeric() || bytes[i - 1] == b'_');
        if boundary && src[i..].starts_with("UnitCreate") {
            let after = i + "UnitCreate".len();
            // 后面也不能还是标识符的一部分（UnitCreated 之类）。
            let ends = bytes
                .get(after)
                .is_some_and(|c| !(c.is_ascii_alphanumeric() || *c == b'_'));
            if ends {
                if let Some(open) = rest_open_paren(bytes, after) {
                    if let Some(call_end) = scan_call(bytes, open) {
                        if let Some(s) = spawn_at(&src, i, open, call_end) {
                            out.push(s);
                        }
                        i = call_end;
                        continue;
                    }
                }
            }
        }
        i += 1;
    }
    Ok(out)
}

/// `from` 起跳空白后紧跟 `(` 就返回它的下标。`UnitCreate` 与 `(` 之间只允许空白。
fn rest_open_paren(bytes: &[u8], from: usize) -> Option<usize> {
    let mut k = from;
    while k < bytes.len() && bytes[k].is_ascii_whitespace() {
        k += 1;
    }
    (bytes.get(k) == Some(&b'(')).then_some(k)
}

/// 从 `(` 开始找到配对的 `)`，返回它之后的下标。字符串和注释里的括号不计。
fn scan_call(bytes: &[u8], open: usize) -> Option<usize> {
    let mut depth = 0usize;
    let mut i = open;
    while i < bytes.len() {
        if let Some(j) = skip_comment(bytes, i) {
            i = j;
            continue;
        }
        match bytes[i] {
            b'"' => i = skip_string(bytes, i),
            b'(' => {
                depth += 1;
                i += 1;
            }
            b')' => {
                depth -= 1;
                i += 1;
                if depth == 0 {
                    return Some(i);
                }
            }
            _ => i += 1,
        }
    }
    None
}

/// `i` 处在注释开头就返回注释之后的下标，否则 None。
fn skip_comment(bytes: &[u8], i: usize) -> Option<usize> {
    if bytes[i..].starts_with(b"//") {
        let mut j = i;
        while j < bytes.len() && bytes[j] != b'\n' {
            j += 1;
        }
        return Some(j);
    }
    if bytes[i..].starts_with(b"/*") {
        let mut j = i + 2;
        // Galaxy 的块注释可嵌套，按嵌套跳到配对的 */。
        let mut depth = 1u32;
        while j < bytes.len() && depth > 0 {
            if bytes[j..].starts_with(b"/*") {
                depth += 1;
                j += 2;
            } else if bytes[j..].starts_with(b"*/") {
                depth -= 1;
                j += 2;
            } else {
                j += 1;
            }
        }
        return Some(j);
    }
    None
}

/// 跳过 `"` 开头的字符串字面量，返回收尾引号之后的下标。Galaxy 用 `\` 转义。
fn skip_string(bytes: &[u8], i: usize) -> usize {
    let mut j = i + 1;
    while j < bytes.len() {
        match bytes[j] {
            b'\\' => j += 2,
            b'"' => return j + 1,
            _ => j += 1,
        }
    }
    bytes.len()
}

/// 按逗号切实参，深度 0 的逗号才是分隔符——括号里的逗号（RegionFromId(29) 那层）不算。
fn split_args(src: &str) -> Vec<String> {
    let bytes = src.as_bytes();
    let mut out = Vec::new();
    let mut start = 0usize;
    let mut depth = 0usize;
    let mut i = 0usize;
    while i < bytes.len() {
        // 注释先剥掉，注释里的逗号/引号都不是语法。
        if let Some(j) = skip_comment(bytes, i) {
            i = j;
            continue;
        }
        match bytes[i] {
            b'"' => i = skip_string(bytes, i),
            b'(' | b'[' => {
                depth += 1;
                i += 1;
            }
            b')' | b']' => {
                depth = depth.saturating_sub(1);
                i += 1;
            }
            b',' if depth == 0 => {
                out.push(src[start..i].trim().to_string());
                start = i + 1;
                i += 1;
            }
            _ => i += 1,
        }
    }
    out.push(src[start..].trim().to_string());
    out
}

/// 组装一处结果：调用起点 `at`、实参列表左括号 `open`、配对右括号之后 `call_end`。
fn spawn_at(src: &str, at: usize, open: usize, call_end: usize) -> Option<Spawn> {
    let args = split_args(&src[open + 1..call_end - 1]);
    // 实参不够就不是我们认的这个签名，跳过更省事。
    let (count, unit, player) = match (args.first(), args.get(1), args.get(3)) {
        (Some(a), Some(b), Some(c)) => (a.clone(), b.clone(), c.clone()),
        _ => return None,
    };
    let line = src[..at].bytes().filter(|c| *c == b'\n').count() + 1;
    let start = src[..at].rfind('\n').map_or(0, |p| p + 1);
    let end = src[at..].find('\n').map_or(src.len(), |p| at + p);
    Some(Spawn { count, unit, player, line, text: src[start..end].trim().to_string() })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tmp(name: &str, body: &str) -> std::path::PathBuf {
        let d = std::env::temp_dir().join(format!("sc2cat-galaxy-{}-{}", std::process::id(), name));
        std::fs::create_dir_all(&d).unwrap();
        let p = d.join("MapScript.galaxy");
        std::fs::write(&p, body).unwrap();
        p
    }

    #[test]
    fn 单行调用() {
        let p = tmp("one", "    UnitCreate(8, \"Zergling\", 0, gv_pLAYER_01, RegionRandomPoint(RegionFromId(29)), 30.0);\n");
        let s = spawns(&p).unwrap();
        assert_eq!(s.len(), 1);
        assert_eq!(s[0].count, "8");
        assert_eq!(s[0].unit, "\"Zergling\"");
        assert_eq!(s[0].player, "gv_pLAYER_01");
        assert_eq!(s[0].line, 1);
        assert!(s[0].text.starts_with("UnitCreate(8"));
    }

    #[test]
    fn 插在动作段开头且不越界() {
        let src = r#"bool gt_Init01Technology_Func (bool testConds, bool runActions) {
    // Automatic Variable Declarations
    // Actions
    if (!runActions) {
        return true;
    }

    libCamp_gf_SetTechRestrictionsCallback(TriggerGetCurrent());
    return true;
}

bool gt_Other_Func (bool a, bool b) {
    UnitCreate(1, "Marine", 0, gv_P, Point(0,0), 0.0);
}
"#;
        let (out, ok) = inject_into_fn(src, "gt_Init01Technology_Func", &["TechTreeUpgradeAddLevel(gv_p1_USER, \"X\", 1);"]);
        assert!(ok);
        let lines: Vec<&str> = out.lines().collect();
        let at = lines.iter().position(|l| l.contains("TechTreeUpgradeAddLevel")).unwrap();
        // 必须插在动作段里（runActions 守卫之后），且不能跑到别的函数去
        assert!(lines[..at].iter().any(|l| l.contains("runActions")));
        assert!(lines[at..].iter().any(|l| l.contains("SetTechRestrictionsCallback")));
        assert_eq!(out.matches("gt_Other_Func").count(), 1);
    }

    #[test]
    fn 函数不存在时不改动() {
        let src = "bool gt_A_Func (bool t, bool r) {\n    return true;\n}\n";
        let (out, ok) = inject_into_fn(src, "gt_NotThere_Func", &["x();"]);
        assert!(!ok);
        assert_eq!(out, src, "失败时必须原样返回");
    }

    #[test]
    fn 嵌套括号不被当成实参分隔() {
        let p = tmp("nest", "UnitCreate(2, \"Roach\", 0, gv_P, RegionRandomPoint(RegionFromId(29)), 30.0);\n");
        let s = spawns(&p).unwrap();
        // 切成 6 段才对；按裸逗号切会把这行切碎
        assert_eq!(s.len(), 1);
        assert_eq!(s[0].count, "2");
        assert_eq!(s[0].unit, "\"Roach\"");
        assert_eq!(s[0].player, "gv_P");
        assert_eq!(split_args("RegionRandomPoint(RegionFromId(29)), 30.0").len(), 2);
    }

    #[test]
    fn 跨多行调用() {
        let src = "void f () {\n    UnitCreate(lp_qty, lp_unitType, 0, lp_player,\n        UnitGetPosition(lp_nydus),\n        libNtve_gf_RandomAngle());\n}\n";
        let p = tmp("multi", src);
        let s = spawns(&p).unwrap();
        assert_eq!(s.len(), 1);
        assert_eq!(s[0].count, "lp_qty");
        assert_eq!(s[0].unit, "lp_unitType");
        assert_eq!(s[0].player, "lp_player");
        assert_eq!(s[0].line, 2);
        assert_eq!(s[0].text, "UnitCreate(lp_qty, lp_unitType, 0, lp_player,");
    }

    #[test]
    fn 注释里的_unitcreate_不算调用() {
        let src = "// UnitCreate(1, \"Ghost\", 0, gv_P, Point(0,0), 0.0);\n/* UnitCreate(1, \"Ghost\", 0, gv_P, Point(0,0), 0.0); */\nUnitCreate(1, \"Marine\", 0, gv_P, Point(0,0), 0.0);\n";
        let p = tmp("comment", src);
        let s = spawns(&p).unwrap();
        assert_eq!(s.len(), 1);
        assert_eq!(s[0].unit, "\"Marine\"");
        assert_eq!(s[0].line, 3);
    }

    #[test]
    fn 更长的标识符不会被误认() {
        let src = "    UnitCreated = 1;\n    libNtve_gf_UnitCreateFacingPoint(1, \"ZeratulZealot\", 0, gv_P, PointFromName(\"P\"), UnitGetPosition(gv_c));\n";
        let p = tmp("prefix", src);
        assert!(spawns(&p).unwrap().is_empty());
    }

    #[test]
    fn 字符串里的逗号与括号不参与切分() {
        let p = tmp("str", "UnitCreate(1, \"ZeratulStalker\", 0, gv_P, PointFromName((\"Victory Stalker \" + IntToString(lv_i))), UnitGetPosition(gv_c));\n");
        let s = spawns(&p).unwrap();
        assert_eq!(s.len(), 1);
        assert_eq!(s[0].unit, "\"ZeratulStalker\"");
        assert_eq!(s[0].player, "gv_P");
    }

    #[test]
    fn 文件不存在返回错误() {
        assert!(spawns(std::path::Path::new("这个文件不存在.galaxy")).is_err());
    }

    #[test]
    fn 只替换创建调用不动筛选用法() {
        let src = r#"void x () {
    libNtve_gf_CreateUnitsAtPoint2(1, "Marine", 0, gv_P, PointFromId(1));
    if ((UnitGroupCount(UnitGroup("Marine", gv_P, RegionEntireMap(), UnitFilter(0,0,0,0), 0), c_unitCountAll) > 0)) {
    }
    UnitCargoCreate(gv_ship, "Marine", 5);
}
"#;
        let (out, n) = replace_created_unit(src, "Marine", "ZeratulZealot", None);
        assert_eq!(n, 2, "创建调用两处");
        assert!(out.contains(r#"CreateUnitsAtPoint2(1, "ZeratulZealot""#));
        assert!(out.contains(r#"UnitCargoCreate(gv_ship, "ZeratulZealot", 5)"#));
        assert!(out.contains(r#"UnitGroup("Marine""#), "筛选用法必须原样保留");
    }

    #[test]
    fn 限定触发器时只改函数体内() {
        let src = r#"bool gt_IntroCinematic_Func (bool a, bool b) {
    libNtve_gf_CreateUnitsAtPoint2(1, "Marine", 0, gv_P, PointFromId(1));
}

bool gt_Other_Func (bool a, bool b) {
    libNtve_gf_CreateUnitsAtPoint2(1, "Marine", 0, gv_P, PointFromId(2));
}
"#;
        let (out, n) = replace_created_unit(src, "Marine", "ZeratulZealot", Some("gt_IntroCinematic_Func"));
        assert_eq!(n, 1);
        assert_eq!(out.matches(r#""ZeratulZealot""#).count(), 1);
        assert_eq!(out.matches(r#""Marine""#).count(), 1, "另一个函数不动");
    }
}

