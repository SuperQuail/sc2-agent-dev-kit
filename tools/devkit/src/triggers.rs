//! 在 `Triggers`（XML）里定位可以精确替换的单位引用，并算出它属于哪个触发器。
//!
//! 实测形态（一个 Param 元素）：
//!
//! ```text
//! <Element Type="Param" Id="4A9B7C33">
//!     <ParameterDef Type="ParamDef" Library="Ntve" Id="00000691"/>
//!     <Value>Zergling</Value>
//!     <ValueType Type="gamelink"/>
//!     <ValueGameType Type="Unit"/>
//! </Element>
//! ```
//!
//! 只有 `ValueGameType Type="Unit"` 的 Param 里的 `<Value>` 才是单位引用。
//!
//! **元素是平表**：Param 并不嵌在 FunctionCall 里，两者靠 `Id` 相互引用。
//! 所以「这个引用属于哪个触发器」要做一次图遍历：
//! Trigger → Action/FunctionCall → Parameter(Param) → Value。
//!
//! 为什么手写扫描而不用 quick-xml：要的是**字节范围**，解析器只给行列号。
use anyhow::Result;
use std::collections::HashMap;

/// 一处「单位引用」以及 `<Value>` **内容**在原文中的字节范围。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnitValue {
    pub value: String,
    /// `<Value>` 开标签之后的第一个字节
    pub start: usize,
    /// `</Value>` 开标签之前的最后一个字节 + 1
    pub end: usize,
    pub line: usize,
    /// 归属触发器的 Element Id
    pub trigger_id: Option<String>,
    /// 归属触发器的显示名（来自 TriggerStrings），读不到就是 None
    pub trigger_name: Option<String>,
}

/// 从 `TriggerStrings.txt` 解析 `Trigger/Name/<hex>=名字`。
pub fn trigger_names(strings: &str) -> HashMap<String, String> {
    let mut out = HashMap::new();
    for line in strings.lines() {
        let Some((k, v)) = line.split_once('=') else { continue };
        if let Some(id) = k.strip_prefix("Trigger/Name/") {
            out.insert(id.trim().to_uppercase(), v.trim().to_string());
        }
    }
    out
}

/// 一个 `<Element>` 的：类型、Id、字节范围。
struct El {
    kind: String,
    id: String,
    start: usize,
    end: usize,
}

const OPEN: &str = "<Element ";
const CLOSE: &str = "</Element>";

/// 扫出全部 Element。**平表**：每个 `<Element` 之后第一个 `</Element>` 就是它的结束。
fn elements(xml: &str) -> Vec<El> {
    let mut out = Vec::new();
    let mut at = 0usize;
    while let Some(rel) = xml[at..].find(OPEN) {
        let s = at + rel;
        let Some(g) = xml[s..].find('>').map(|g| s + g) else { break };
        let head = &xml[s..g];
        let Some(e) = xml[g..].find(CLOSE).map(|e| g + e) else { break };
        out.push(El {
            kind: attr(head, "Type").unwrap_or_default(),
            id: attr(head, "Id").unwrap_or_default(),
            start: s,
            end: e,

        });
        at = e + CLOSE.len();
    }
    out
}

/// 从标签文本里取 `key="..."`。
pub fn attr(tag: &str, key: &str) -> Option<String> {
    let pat = format!("{key}=\"");
    let i = tag.find(&pat)? + pat.len();
    let j = tag[i..].find('"')? + i;
    Some(tag[i..j].to_string())
}

/// 一个块里所有 `Type="Param" Id="X"` 的引用。
fn refs_of(block: &str, ty: &str) -> Vec<String> {
    let pat = format!("Type=\"{ty}\" Id=\"");
    let mut out = Vec::new();
    let mut at = 0usize;
    while let Some(i) = block[at..].find(&pat) {
        let s = at + i + pat.len();
        let Some(j) = block[s..].find('"') else { break };
        out.push(block[s..s + j].to_string());
        at = s + j;
    }
    out
}

/// 找出全部属于 `Unit` 类型 Param 的 `<Value>`，并标注它属于哪个触发器。
pub fn unit_values(xml: &str) -> Result<Vec<UnitValue>> {
    unit_values_named(xml, &HashMap::new())
}

pub fn unit_values_named(xml: &str, names: &HashMap<String, String>) -> Result<Vec<UnitValue>> {
    let els = elements(xml);
    let by_id: HashMap<&str, &El> = els.iter().map(|e| (e.id.as_str(), e)).collect();

    // Param id -> 持有它的 FunctionCall id（由 FunctionCall 块里的 Parameter 引用得出）
    let mut param_owner: HashMap<String, String> = HashMap::new();
    // FunctionCall id -> 它的直接子 FunctionCall（做传递闭包用）
    let mut call_children: HashMap<String, Vec<String>> = HashMap::new();
    // Trigger id -> 它的直接 FunctionCall
    let mut trigger_roots: Vec<(String, String)> = Vec::new();

    for e in &els {
        let block = &xml[e.start..e.end];
        match e.kind.as_str() {
            "FunctionCall" => {
                for p in refs_of(block, "Param") {
                    param_owner.insert(p, e.id.clone());
                }
                call_children.insert(e.id.clone(), refs_of(block, "FunctionCall"));
            }
            "Trigger" => {
                for c in refs_of(block, "FunctionCall") {
                    trigger_roots.push((e.id.clone(), c));
                }
            }
            _ => {}
        }
    }

    // FunctionCall id -> 归属触发器（从每个触发器的根出发做可达性）
    let mut call_trigger: HashMap<String, String> = HashMap::new();
    for (tid, root) in &trigger_roots {
        let mut stack = vec![root.clone()];
        while let Some(c) = stack.pop() {
            if call_trigger.insert(c.clone(), tid.clone()).is_some() {
                continue; // 已有归属，避免环
            }
            if let Some(kids) = call_children.get(&c) {
                stack.extend(kids.iter().cloned());
            }
        }
    }

    let mut out = Vec::new();
    for e in &els {
        if e.kind != "Param" {
            continue;
        }
        let block = &xml[e.start..e.end];
        if !block.contains("ValueGameType") {
            continue;
        }
        let Some(vg) = block.find("<ValueGameType") else { continue };
        let Some(gt) = block[vg..].find('>').map(|g| vg + g) else { continue };
        if attr(&block[vg..gt], "Type").as_deref() != Some("Unit") {
            continue;
        }
        let Some(vo) = block.find("<Value>") else { continue };
        let content = vo + "<Value>".len();
        let Some(vc) = block[content..].find("</Value>").map(|c| content + c) else { continue };
        let start = e.start + content;
        let trigger_id = param_owner
            .get(&e.id)
            .and_then(|c| call_trigger.get(c))
            .cloned();
        let trigger_name = trigger_id.as_ref().and_then(|t| names.get(t).cloned());
        out.push(UnitValue {
            value: block[content..vc].to_string(),
            start,
            end: e.start + vc,
            line: xml[..start].bytes().filter(|&c| c == b'\n').count() + 1,
            trigger_id,
            trigger_name,
        });
    }
    let _ = by_id;
    Ok(out)
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;

    /// 最小触发器图：一个 Trigger 指向一个 FunctionCall，后者带一个 Unit 参数。
    fn fixture() -> String {
        [
            "<TriggerData>",
            "  <Element Type=\"Trigger\" Id=\"AAAA0001\">",
            "    <Action Type=\"FunctionCall\" Id=\"CCCC0001\"/>",
            "  </Element>",
            "  <Element Type=\"FunctionCall\" Id=\"CCCC0001\">",
            "    <FunctionDef Type=\"FunctionDef\" Library=\"Ntve\" Id=\"F20A0011\"/>",
            "    <Parameter Type=\"Param\" Id=\"PPPP0001\"/>",
            "    <Parameter Type=\"Param\" Id=\"PPPP0002\"/>",
            "  </Element>",
            "  <Element Type=\"Param\" Id=\"PPPP0001\">",
            "    <Value>Marine</Value>",
            "    <ValueType Type=\"gamelink\"/>",
            "    <ValueGameType Type=\"Unit\"/>",
            "  </Element>",
            "  <Element Type=\"Param\" Id=\"PPPP0002\">",
            "    <Value>5</Value>",
            "    <ValueType Type=\"int\"/>",
            "  </Element>",
            "</TriggerData>",
        ]
        .join("\n")
    }

    #[test]
    fn 找到单位引用并定出归属触发器() {
        let xml = fixture();
        let vs = unit_values(&xml).unwrap();
        assert_eq!(vs.len(), 1, "只有 Unit 类型的 Param 才算");
        assert_eq!(vs[0].value, "Marine");
        assert_eq!(vs[0].trigger_id.as_deref(), Some("AAAA0001"));
        // 字节范围切出来正好是 value
        assert_eq!(&xml[vs[0].start..vs[0].end], "Marine");
    }

    #[test]
    fn 触发器显示名来自_TriggerStrings() {
        let mut names = HashMap::new();
        names.insert("AAAA0001".to_string(), "Intro Cinematic".to_string());
        let vs = unit_values_named(&fixture(), &names).unwrap();
        assert_eq!(vs[0].trigger_name.as_deref(), Some("Intro Cinematic"));
    }

    #[test]
    fn 没有归属引用时_trigger_id_为空() {
        let xml = [
            "<TriggerData>",
            "  <Element Type=\"Param\" Id=\"PPPP0001\">",
            "    <Value>Zealot</Value>",
            "    <ValueGameType Type=\"Unit\"/>\n  </Element>",
            "</TriggerData>",
        ]
        .join("\n");
        let vs = unit_values(&xml).unwrap();
        assert_eq!(vs.len(), 1);
        assert!(vs[0].trigger_id.is_none());
    }

    #[test]
    fn 自闭合的_value_不产生引用() {
        let xml = "<TriggerData><Element Type=\"Param\" Id=\"P1\"><ValueGameType Type=\"Unit\"/></Element></TriggerData>";
        assert!(unit_values(xml).unwrap().is_empty());
    }

    #[test]
    fn 多行时行号按_value_起始行给() {
        let xml = "<TriggerData>\n\n\n<Element Type=\"Param\" Id=\"P1\">\n<Value>Marine</Value>\n<ValueGameType Type=\"Unit\"/>\n</Element>\n</TriggerData>";
        let vs = unit_values(xml).unwrap();
        assert_eq!(vs[0].line, 5);
    }

    #[test]
    fn 解析_TriggerStrings() {
        let s = "Category/Name/AA=x\nTrigger/Name/CC2D6739=Intro Cinematic\nTrigger/Name/4a874697=Lower\n";
        let n = trigger_names(s);
        assert_eq!(n.get("CC2D6739").map(String::as_str), Some("Intro Cinematic"));
        // 键统一大写
        assert_eq!(n.get("4A874697").map(String::as_str), Some("Lower"));
    }
}
