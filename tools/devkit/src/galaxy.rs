//! 从生成的 `MapScript.galaxy` 里找出单位生成点。
//!
//! **只读**。这个文件是编辑器从 `Triggers` 生成的，改它会被覆盖——
//! 规则见 skills/sc2-map-triggers。它可读性远好于编译后的触发器图，所以用它来*找*。
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
}
