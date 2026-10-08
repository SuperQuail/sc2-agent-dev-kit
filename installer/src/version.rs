//! 版本号解析与比较。
//!
//! 被自更新和「找最新发布」共用，所以单独放一个模块。
//!
//! 后缀有两种写法都要认：
//!   * `0.1.0-a1` —— Cargo.toml 的写法（semver 要求连字符）
//!   * `0.1.0a1`  —— tools/sc2_version.py 的写法，也出现在 git tag 上
//! 两者视为同一个版本。

/// 拆成（数字段, 预发布后缀）。
pub fn parse(version: &str) -> (Vec<u64>, String) {
    let v = version.trim().trim_start_matches(['v', 'V']);
    let (core, mut pre) = match v.split_once('-') {
        Some((c, p)) => (c, p.to_string()),
        None => (v, String::new()),
    };
    let last = core.rsplit('.').next().unwrap_or("").to_string();
    let mut nums = Vec::new();
    for part in core.split('.') {
        let digits: String = part.chars().take_while(|c| c.is_ascii_digit()).collect();
        let rest: String = part.chars().skip_while(|c| c.is_ascii_digit()).collect();
        nums.push(digits.parse().unwrap_or(0));
        // 后缀直接粘在最后一段上时（0.1.0a1），把它摘出来
        if !rest.is_empty() && pre.is_empty() && part == last {
            pre = rest;
        }
    }
    (nums, pre)
}

/// candidate 是否比 current 新。
pub fn is_newer(candidate: &str, current: &str) -> bool {
    let (ca, cp) = parse(candidate);
    let (cu, cup) = parse(current);
    if ca != cu {
        return ca > cu;
    }
    match (cp.is_empty(), cup.is_empty()) {
        (true, true) => false,
        // 候选是预发布、当前是正式版：不新
        (false, true) => false,
        // 候选是正式版、当前是预发布：新
        (true, false) => true,
        (false, false) => cp > cup,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_both_suffix_styles() {
        assert_eq!(parse("0.1.0-a1"), (vec![0, 1, 0], "a1".to_string()));
        assert_eq!(parse("0.1.0a1"), (vec![0, 1, 0], "a1".to_string()));
        assert_eq!(parse("v1.2.3"), (vec![1, 2, 3], String::new()));
    }

    #[test]
    fn compares_numeric_parts_first() {
        assert!(is_newer("0.2.0", "0.1.9"));
        assert!(!is_newer("0.1.0", "0.2.0"));
        assert!(!is_newer("0.1.0", "0.1.0"));
    }

    #[test]
    fn compares_prerelease_suffixes() {
        assert!(is_newer("0.1.0-a2", "0.1.0-a1"));
        assert!(!is_newer("0.1.0-a1", "0.1.0-a2"));
        assert!(is_newer("0.1.0", "0.1.0-a1"));
        assert!(!is_newer("0.1.0-a1", "0.1.0"));
    }

    #[test]
    fn suffix_style_does_not_change_ordering() {
        assert!(!is_newer("0.1.0a1", "0.1.0-a1"));
        assert!(!is_newer("0.1.0-a1", "0.1.0a1"));
    }
}
