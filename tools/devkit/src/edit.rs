//! 精确文本拼接：只动指定的字节范围，其余逐字节不变。
//!
//! 为什么不做 DOM 往返：写回会重排缩进与属性顺序，而项目在意编辑器往返安全。
//! 这也是仓库里 `repair_duplicate_trigger_ids.py` 的路子。
use anyhow::{bail, Result};

/// 在 `text` 上按 `(start, end, 新内容)` 替换若干处，返回新文本。
///
/// 区间必须按起点升序且**互不重叠**（后一处起点 >= 前一处终点）；
/// 越界、落在字符中间、或倒序重叠都报错，不能静默错位。空 `edits` 返回原文。
pub fn splice(text: &str, edits: &[(usize, usize, String)]) -> Result<String> {
    if edits.is_empty() {
        return Ok(text.to_string());
    }
    let mut len = text.len();
    let mut prev_end = 0usize;
    for (n, (start, end, new)) in edits.iter().enumerate() {
        if start > end || *end > text.len() {
            bail!("第 {} 处区间越界：{start}..{end}，原文 {} 字节", n + 1, text.len());
        }
        if !text.is_char_boundary(*start) || !text.is_char_boundary(*end) {
            bail!("第 {} 处区间落在字符中间：{start}..{end}", n + 1);
        }
        if n > 0 && *start < prev_end {
            bail!("第 {} 处与前一处重叠或倒序：起点 {start} < 前一处终点 {prev_end}", n + 1);
        }
        len = len - (*end - *start) + new.len();
        prev_end = *end;
    }

    let mut out = String::with_capacity(len);
    let mut at = 0usize;
    for (start, end, new) in edits {
        out.push_str(&text[at..*start]);
        out.push_str(new);
        at = *end;
    }
    out.push_str(&text[at..]);
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 单个替换() {
        assert_eq!(splice("abcdef", &[(2, 4, "XY".into())]).unwrap(), "abXYef");
    }

    #[test]
    fn 多个替换_未涉及的字节不变() {
        let s = "<Value>Zealot</Value>\n<Value>Zergling</Value>\n";
        let a = s.find("Zealot").unwrap();
        let b = s.find("Zergling").unwrap();
        let out = splice(s, &[(a, a + 6, "ZeratulZealot".into()), (b, b + 8, "Roach".into())]).unwrap();
        assert_eq!(out, "<Value>ZeratulZealot</Value>\n<Value>Roach</Value>\n");
        // 换行与其余标点一字未动
        assert_eq!(out.matches('\n').count(), 2);
    }

    #[test]
    fn 空_edits_返回原文() {
        let s = "abc\n中文\n";
        assert_eq!(splice(s, &[]).unwrap(), s);
    }

    #[test]
    fn 越界报错() {
        assert!(splice("abc", &[(0, 99, "x".into())]).is_err());
    }

    #[test]
    fn 倒序或重叠报错() {
        // 倒序
        assert!(splice("abcdef", &[(3, 5, "x".into()), (1, 2, "y".into())]).is_err());
        // 真重叠
        assert!(splice("abcdef", &[(1, 4, "x".into()), (3, 5, "y".into())]).is_err());
    }

    #[test]
    fn 零宽相邻是允许的() {
        // 同一点插两次不产生错位，顺序确定
        assert_eq!(splice("ab", &[(1, 1, "X".into()), (1, 1, "Y".into())]).unwrap(), "aXYb");
    }

    #[test]
    fn 落在字符中间报错() {
        let s = "中"; // 3 字节
        assert!(splice(s, &[(1, 2, "x".into())]).is_err());
    }
}
