//! 把 catalog id 改名（连同所有引用），让两套数据**共存**而不是抢优先级。
//!
//! 为什么需要：地图依赖表有序，后面的盖前面的。当别人的 mod 和我们用了**同一个 id**
//! （实测：`CActorUnit ZeratulZealot` 同时被自制 kit 和 StarCoop 定义），
//! 改顺序只是决定谁被牺牲 —— 一边的模型对了，另一边的菜单/技能就没了。
//!
//! **正确做法是把我们这边的 id 改掉**，两边各用各的，谁也不盖谁。
use anyhow::Result;
use std::path::{Path, PathBuf};

/// 一次改名的配对。
pub struct Pair {
    pub from: String,
    pub to: String,
}

/// 在若干目录树里把 `from` 改成 `to`，返回改动的文件数。
///
/// 两类文件分别处理：
/// - XML / Galaxy / Triggers / Objects：替换**带引号的** `"from"` —— 既覆盖 `id="from"`
///   也覆盖 `Link="from"` 这类引用，而且不会误伤 `"fromBlink"`（引号定界天然防前缀误配）。
/// - 本地化 .txt：键是 `Kind/Field/from=`，替换 `from=`。同理不会误伤 `fromBlink=`。
pub fn rename(roots: &[PathBuf], pairs: &[Pair]) -> Result<(usize, usize)> {
    let mut files = 0usize;
    let mut hits = 0usize;
    for root in roots {
        for f in walk(root) {
            let Ok(raw) = std::fs::read(&f) else { continue };
            let Ok(text) = String::from_utf8(raw) else { continue };
            let mut out = text.clone();
            let mut n = 0usize;
            for p in pairs {
                let (o, k) = replace_id(&out, &p.from, &p.to);
                out = o;
                n += k;
            }
            if n > 0 && out != text {
                std::fs::write(&f, out)?;
                files += 1;
                hits += n;
            }
        }
    }
    Ok((files, hits))
}

/// 按**标识符边界**替换：前后不是 [A-Za-z0-9_] 才算一个完整的 id。
///
/// 这一条规则同时覆盖三种真实写法，而且不误伤别的 id：
///
///   id="X"                          前后是引号      → 改
///   AbilCmd="X,Execute"             前引号后逗号     → 改（漏这处，命令卡按钮消失）
///   Terms="Abil.X.TargetOn"         前后都是点号     → 改（漏这处，actor 事件不触发）
///   DefaultButtonFace="ResearchX"   前面是字母       → 不改（那是另一个 id）
///
/// 早先按「带引号的整值」替换，前两种全漏了。
fn replace_id(text: &str, from: &str, to: &str) -> (String, usize) {
    let cs: Vec<char> = text.chars().collect();
    let f: Vec<char> = from.chars().collect();
    let mut out = String::with_capacity(text.len());
    let mut n = 0usize;
    let mut i = 0usize;
    let ident = |c: char| c.is_ascii_alphanumeric() || c == '_';
    while i < cs.len() {
        if i + f.len() <= cs.len() && cs[i..i + f.len()] == f[..] {
            let prev_ok = i == 0 || !ident(cs[i - 1]);
            let next_ok = i + f.len() >= cs.len() || !ident(cs[i + f.len()]);
            if prev_ok && next_ok {
                out.extend(to.chars());
                i += f.len();
                n += 1;
                continue;
            }
        }
        out.push(cs[i]);
        i += 1;
    }
    (out, n)
}

fn walk(root: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(d) = stack.pop() {
        let Ok(rd) = std::fs::read_dir(&d) else { continue };
        for e in rd.flatten() {
            let p = e.path();
            let Ok(ft) = e.file_type() else { continue };
            if ft.is_symlink() {
                continue;
            } else if ft.is_dir() {
                stack.push(p);
            } else {
                out.push(p);
            }
        }
    }
    out.sort();
    out
}