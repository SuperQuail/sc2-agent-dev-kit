//! 精确文本拼接：只动指定的字节范围，其余逐字节不变。
//!
//! 为什么不做 DOM 往返：写回会重排缩进与属性顺序，而项目在意编辑器往返安全。
//! 这也是仓库里 `repair_duplicate_trigger_ids.py` 的路子。
use anyhow::{bail, Result};

/// 在 `text` 上按 `(start, end, 新内容)` 替换若干处，返回新文本。
///
/// 区间必须互不重叠且按起点升序；越界或重叠要报错，不能静默错位。
/// 空 `edits` 返回原文。
pub fn splice(text: &str, edits: &[(usize, usize, String)]) -> Result<String> {
    if edits.is_empty() {
        return Ok(text.to_string());
    }
    // 先一次算清结果长度：任何一处非法都在动原文之前就报出来。
    let mut len = text.len();
    let mut prev_end = 0usize;
    for (n, (start, end, new)) in edits.iter().enumerate() {
        if start > end || end > text.len() {
            bail!("第 {} 处区间越界：{start}..{end}，原文只有 {} 字节", n + 1, text.len());
        }
        if !text.is_char_boundary(*start) || !text.is_char_boundary(*end) {
            bail!("第 {} 处区间落在字符中间：{start}..{end}", n + 1);
        }
        if n > 0 && *start < prev_end {
            bail!("第 {} 处区间与前一处重叠：{start} < {prev_end}", n + 1);
        }
        len = len - (end - start) + new.len();
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
    fn 空_edits 返回原文() {
        let s = "abc\n中文\n";
        assert_eq!(splice(s, &[]).unwrap(), s);
    }

    #[test]
    fn 单个替换() {
        let s = "hello world";
        let out = splice(s, &[(6, 11, "Rust".to_string())]).unwrap();
        assert_eq!(out, "hello Rust");
    }

    #[test]
    fn 多个替换未涉及的字节不变() {
        let s = "<Value>Zealot</Value>\n<Value>Zealot</Value>\n";
        let edits = vec![
            (7, 13, "ZeratulZealot".to_string()),
            (30, 36, "Zergling".to_string()),
        ];
        let out = splice(s, &edits).unwrap();
        assert_eq!(out, "<Value>ZeratulZealot</Value>\n<Value>Zergling</Value>\n");
        // 逐字节核对：只有那两段变了
        assert_eq!(out.len(), s.len() - 6 + 13 - 6 + 8);
    }

    #[test]
    fn 替换后长度变化正确() {
        let s = "abcdef";
        assert_eq!(splice(s, &[(1, 3, "X".to_string())]).unwrap().len(), 6 - 2 + 1);
        assert_eq!(splice(s, &[(1, 3, "LONGER".to_string())]).unwrap(), "aLONGERdef");
        // 删除：新内容为空
        assert_eq!(splice(s, &[(1, 3, String::new())]).unwrap(), "adef");
        // 插入：零宽区间
        assert_eq!(splice(s, &[(3, 3, "-".to_string())]).unwrap(), "abc-def");
    }

    #[test]
    fn 重叠区间报错() {
        let s = "abcdef";
        assert!(splice(s, &[(1, 4, "x".to_string()), (3, 5, "y".to_string())]).is_err());
        // 同一个点插两次也算重叠
        assert!(splice(s, &[(3, 3, "x".to_string()), (3, 3, "y".to_string())]).is_err());
    }

    #[test]
    fn 越界与倒序区间报错() {
        let s = "abcdef";
        assert!(splice(s, &[(0, 7, "x".to_string())]).is_err());
        assert!(splice(s, &[(4, 2, "x".to_string())]).is_err());
        assert!(splice(s, &[(3, 5, "x".to_string()), (1, 2, "y".to_string())]).is_ok());
    }

    #[test]
    fn 切在字符中间报错而不是_panic() {
        let s = "中文";
        assert!(splice(s, &[(1, 2, "x".to_string())]).is_err());
    }
}
