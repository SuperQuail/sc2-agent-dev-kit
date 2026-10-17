//! 在 `Triggers`（XML）里定位可以精确替换的单位引用。
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
//! 只有 `ValueGameType Type="Unit"` 的 Param 里的 `<Value>` 才是单位引用；其余 `<Value>`
//! 是数字、字符串或别的 gamelink 类型。
//!
//! 为什么手写扫描而不用 quick-xml：这里要的是**字节范围**，解析器只给行列号，
//! 换算回字节偏移要把缩进和实体都算一遍。逐字节走，看到的偏移就是原文偏移。
use anyhow::Result;

/// 一处「单位引用」以及 `<Value>` **内容**在原文中的字节范围。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnitValue {
    pub value: String,
    /// `<Value>` 开标签之后的第一个字节
    pub start: usize,
    /// `</Value>` 开标签之前的最后一个字节 + 1
    pub end: usize,
    pub line: usize,
}

/// 找出全部属于 `Unit` 类型 Param 的 `<Value>`。
///
/// `TriggerData` 里 `<Element>` 是**平表**，不互相嵌套，所以每个 `<Element` 之后
/// 第一个 `</Element>` 就是它的结束。
pub fn unit_values(xml: &str) -> Result<Vec<UnitValue>> {
    let mut out = Vec::new();
    let mut at = 0usize;
    while let Some(open) = xml[at..].find("<Element") {
        let open = at + open;
        let Some(end) = xml[open..].find("</Element>").map(|e| open + e) else { break };
        let block = &xml[open..end];
        at = end + "</Element>".len();

        // 只认 Param，且必须声明 ValueGameType 为 Unit。
        if !tag_has(block, "<Element", "Type", "Param") {
            continue;
        }
        if !block.contains("ValueGameType") || !tag_has(block, "<ValueGameType", "Type", "Unit") {
            continue;
        }
        // 取 <Value> ... </Value> 的内容范围（内容里可能有子元素，用最后一个 </Value> 前的位置）。
        let Some(vopen) = block.find("<Value>") else { continue };
        let content = vopen + "<Value>".len();
        let Some(vclose) = block[content..].find("</Value>").map(|c| content + c) else {
            continue;
        };
        let start = open + content;
        let end = open + vclose;
        out.push(UnitValue {
            value: block[content..vclose].to_string(),
            start,
            end,
            line: line_of(xml, start),
        });
    }
    Ok(out)
}

/// `block` 里第一个以 `tag` 开头的标签，它的 `key` 属性是否等于 `value`。
fn tag_has(block: &str, tag: &str, key: &str, value: &str) -> bool {
    let Some(i) = block.find(tag) else { return false };
    let Some(gt) = block[i..].find('>').map(|g| i + g) else { return false };
    attr(&block[i..gt], key).as_deref() == Some(value)
}

/// 从标签文本里取 `key="..."` 的值。
fn attr(tag: &str, key: &str) -> Option<String> {
    let pat = format!("{key}=\"");
    let i = tag.find(&pat)? + pat.len();
    let j = tag[i..].find('"')? + i;
    Some(tag[i..j].to_string())
}

/// 数换行得到 1-based 行号。
fn line_of(xml: &str, at: usize) -> usize {
    xml[..at].bytes().filter(|&c| c == b'\n').count() + 1
}
