//! 在 `Triggers`（XML）里定位可以精确替换的单位引用。
//!
//! 实测形态：
//! ````text
//! <Element Type="Param" Id="4A9B7C33">
//!     <ParameterDef Type="ParamDef" Library="Ntve" Id="00000691"/>
//!     <Value>Zergling</Value>
//!     <ValueType Type="gamelink"/>
//!     <ValueGameType Type="Unit"/>
//! </Element>
//! ````
//! 只有 `ValueGameType` 为 `Unit` 的那个 `<Value>` 才是单位引用；其余 `<Value>` 是数字、
//! 字符串或别的 gamelink 类型。只改这类 `<Value>` 里的文本，其余一字不动。
//!
//! 为什么手写扫描而不用 quick-xml：这里要的是**字节范围**，而 XML 解析器只给行号列号，
//! 换算回字节偏移要把缩进和实体都算一遍，既慢又容易错。手写扫描逐字节走，看到的偏移量
//! 就是原文偏移量，不需要任何回推。
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

/// 栈上的一项：元素名 + 它的字节起点 + 若是 Value 则记下它是不是单位引用。
struct Frame {
    name: String,
    start: usize,
    /// Value 专用：已经确认过 `ValueGameType Type="Unit"`
    unit: bool,
}

/// 找出全部 `ValueGameType = Unit` 的 `<Value>`。
pub fn unit_values(xml: &str) -> Result<Vec<UnitValue>> {
    let s = xml.as_bytes();
    let mut out: Vec<UnitValue> = Vec::new();
    // 内容里可能还有子元素，所以靠栈配平，不能只 search 最近的 `</Value>`。
    let mut stack: Vec<Frame> = Vec::new();
    let (mut nl, mut ln) = (0usize, 1usize);
    let mut i = 0usize;

    while i < s.len() {
        if s[i] != b'<' {
            i += 1;
            continue;
        }
        // 找本标签的 '>'；扫不下去说明文件尾部是个残标签，直接停。
        let Some(rel) = s[i + 1..].iter().position(|&c| c == b'>') else { break };
        let gt = i + 1 + rel;
        let raw = &s[i + 1..gt];
        let closing = raw.first() == Some(&b'/');
        // 结尾是 '`/`' 说明自闭合。属性里的 '`/`' 都在引号内，不会出现在结尾。
        let empty = raw.last() == Some(&b'/');
        let body = if closing { &xml[i + 2..gt] } else { &xml[i + 1..gt] };
        let (name, attrs) = split_name(body);

        if closing {
            // 找同名开标签；找不到就当这段是文本，忽略。
            if let Some(at) = stack.iter().rposition(|f| f.name == name) {
                for f in stack.drain(at..).rev() {
                    if f.name == "Value" && f.unit {
                        let line = line_at(xml, s, f.start, &mut nl, &mut ln);
                        out.push(UnitValue {
                            value: xml[f.start..i].to_string(),
                            start: f.start,
                            end: i,
                            line,
                        });
                    }
                }
            }
        } else if !empty {
            stack.push(Frame { name: name.to_string(), start: gt + 1, unit: false });
            if name == "ValueGameType" && attr(attrs, "Type").as_deref() == Some("Unit") {
                // 自闭合的 ValueGameType 与它的 Value 同属一个 Param；中间可能有兄弟元素，
                // 所以往上找最近的那个 Value。
                for f in stack.iter_mut().rev() {
                    if f.name == "Value" {
                        f.unit = true;
                        break;
                    }
                }
            }
        }
        i = gt + 1;
    }
    // drain 是从内到外，同一层不会嵌套两个 Value，顺序天然是文档序。
    out.reverse();
    Ok(out)
}

/// 数到 `at` 为止的换行数，得到 1-based 行号。惰性推进，整体只过一遍。
fn line_at(xml: &str, s: &[u8], at: usize, nl: &mut usize, ln: &mut usize) -> usize {
    let _ = xml;
    while *nl < at && *nl < s.len() {
        if s[*nl] == b'\n' {
            *ln += 1;
        }
        *nl += 1;
    }
    *ln
}

/// 拆开 "`名字 属性...`"；名字到第一个空白为止。
fn split_name(body: &str) -> (&str, &str) {
    let end = body.find(char::is_whitespace).unwrap_or(body.len());
    (&body[..end], &body[end..])
}

/// 从一个开标签的属性区里取某个属性的值（不做实体解码）。
fn attr(attrs: &str, key: &str) -> Option<String> {
    let b = attrs.as_bytes();
    let mut i = 0usize;
    while i < b.len() {
        if !(b[i].is_ascii_alphanumeric() || b[i] == b'-' || b[i] == b'_' || b[i] == b':') {
            i += 1;
            continue;
        }
        let ks = i;
        while i < b.len() && (b[i].is_ascii_alphanumeric() || b[i] == b'-' || b[i] == b'_' || b[i] == b':') {
            i += 1;
        }
        let k = &attrs[ks..i];
        while i < b.len() && b[i].is_ascii_whitespace() {
            i += 1;
        }
        if i >= b.len() || b[i] != b'=' {
            continue;
        }
        i += 1;
        while i < b.len() && b[i].is_ascii_whitespace() {
            i += 1;
        }
        if i >= b.len() || (b[i] != b'"' && b[i] != b'\'') {
            continue;
        }
        let quote = b[i];
        i += 1;
        let vs = i;
        while i < b.len() && b[i] != quote {
            i += 1;
        }
        if k == key {
            return Some(attrs[vs..i].to_string());
        }
        i += 1;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 实测的 Param 形态。
    fn param(unit: &str, gametype: &str) -> String {
        format!(
            "    <Element Type=\"Param\" Id=\"4A9B7C33\">\n        <ParameterDef Type=\"ParamDef\" Library=\"Ntve\" Id=\"00000691\"/>\n        <Value>{unit}</Value>\n        <ValueType Type=\"gamelink\"/>\n        <ValueGameType Type=\"{gametype}\"/>\n    </Element>\n"
        )
    }

    #[test]
    fn 单个单位引用_字节范围切出来正好是_value() {
        let xml = param("Zergling", "Unit");
        let vs = unit_values(&xml).unwrap();
        assert_eq!(vs.len(), 1);
        assert_eq!(vs[0].value, "Zergling");
        assert_eq!(&xml[vs[0].start..vs[0].end], "Zergling");
    }

    #[test]
    fn 多个引用按文档序返回() {
        let xml = format!("<TriggerData>\n{}{}{}</TriggerData>\n", param("Zealot", "Unit"), param("Zergling", "Unit"), param("Marine", "Unit"));
        let vs = unit_values(&xml).unwrap();
        let names: Vec<&str> = vs.iter().map(|v| v.value.as_str()).collect();
        assert_eq!(names, ["Zealot", "Zergling", "Marine"]);
        for v in &vs {
            assert_eq!(&xml[v.start..v.end], v.value);
        }
    }

    #[test]
    fn 非_unit 的_ValueGameType 要排除() {
        let xml = format!(
            "<TriggerData>{}{}{}{}</TriggerData>",
            param("Zealot", "Unit"),
            param("Stimpack", "Behavior"),
            param("SomeActor", "Actor"),
            // 没有 ValueGameType 的普通 Value（数字、字符串）也不是单位引用
            "    <Element Type=\"Param\"><ValueType Type=\"int\"/><Value>7</Value></Element>"
        );
        let vs = unit_values(&xml).unwrap();
        let names: Vec<&str> = vs.iter().map(|v| v.value.as_str()).collect();
        assert_eq!(names, ["Zealot"]);
    }

    #[test]
    fn 值里有子元素也能正确切出范围() {
        let xml = "<R><Element Type=\"Param\"><Value>Ze<B>a</B>lot</Value><ValueGameType Type=\"Unit\"/></Element></R>";
        let vs = unit_values(xml).unwrap();
        assert_eq!(vs.len(), 1);
        assert_eq!(vs[0].value, "Ze<B>a</B>lot");
        assert_eq!(&xml[vs[0].start..vs[0].end], vs[0].value);
    }

    #[test]
    fn 行号按_value 起始行给() {
        let xml = format!("<R>\n{}</R>", param("Zealot", "Unit"));
        let vs = unit_values(&xml).unwrap();
        // 第 1 行是 <R>，<Value> 在第 4 行
        assert_eq!(vs[0].line, 4);
    }

    #[test]
    fn 自闭合的_value 不产生引用() {
        let xml = "<R><Element><Value/><ValueGameType Type=\"Unit\"/></Element></R>";
        assert!(unit_values(xml).unwrap().is_empty());
    }
}
