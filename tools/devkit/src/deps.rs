//! 依赖闭包解析 + 冲突检测。
//!
//! 为什么需要：地图依赖表**有序**，后面的盖前面的。给地图加一个看似无关的 mod
//! 会把整条依赖链拉进来，其中某个 mod 重定义了我们自制 mod 依赖的 id ——
//! 文件一个字节没改，单位外观/技能/菜单却变了。
//!
//! **静态比文件内容查不出这种事**：它不是内容差异，是列表顺序 + 继承链。
//! 这个模块把整条闭包摊开，列出同一个 id 被谁定义了几次、谁赢。
use anyhow::{Context, Result};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use crate::catalog;
use crate::map;

/// 闭包里的一个依赖点。
#[derive(Debug, Clone)]
pub struct Node {
    pub dep: String,
    pub root: PathBuf,
}

/// 从依赖串里取出 file: 后面的路径。
pub fn path_of(dep: &str) -> Option<String> {
    let at = dep.find("file:")?;
    Some(dep[at + 5..].trim_matches('"').replace('\\', "/"))
}

/// 在若干搜索根下找这个依赖对应哪个目录（大小写不敏感）。
fn locate(name: &str, search: &[PathBuf]) -> Option<PathBuf> {
    for base in search {
        let direct = base.join(name);
        if direct.is_dir() {
            return Some(direct);
        }
        let mut cur = base.clone();
        let mut ok = true;
        for seg in name.split('/') {
            let mut found = None;
            if let Ok(rd) = std::fs::read_dir(&cur) {
                for e in rd.flatten() {
                    if e.file_name().to_string_lossy().to_ascii_lowercase()
                        == seg.to_ascii_lowercase()
                    {
                        found = Some(e.path());
                        break;
                    }
                }
            }
            match found {
                Some(p) => cur = p,
                None => {
                    ok = false;
                    break;
                }
            }
        }
        if ok && cur.is_dir() {
            return Some(cur);
        }
    }
    None
}

/// 展开依赖闭包，**保持优先级顺序**（先出现的优先级低）。
pub fn closure(start: &Path, search: &[PathBuf]) -> Result<(Vec<Node>, Vec<String>)> {
    let mut seen: Vec<String> = Vec::new();
    let mut out: Vec<Node> = Vec::new();
    let mut missing: Vec<String> = Vec::new();
    let mut queue: Vec<PathBuf> = vec![start.to_path_buf()];
    while let Some(dir) = queue.pop() {
        let hdr = ["DocumentHeader", "documentheader"]
            .iter()
            .map(|n| dir.join(n))
            .find(|p| p.is_file());
        let Some(hdr) = hdr else { continue };
        let deps = map::read_dependencies(&std::fs::read(&hdr)?)?;
        for d in deps {
            if seen.iter().any(|x| x == &d) {
                continue;
            }
            seen.push(d.clone());
            match path_of(&d).and_then(|p| locate(&p, search)) {
                Some(root) => {
                    out.push(Node { dep: d.clone(), root: root.clone() });
                    queue.push(root);
                }
                None => missing.push(d.clone()),
            }
        }
    }
    Ok((out, missing))
}

/// 每个 (表, id) 被哪些闭环节点定义（按出现顺序，最后的最优先）。
pub type Owners = BTreeMap<(String, String), Vec<usize>>;

/// 扫整条闭包，统计每个 (表, id) 被哪些节点定义。
pub fn owners(nodes: &[Node]) -> Result<Owners> {
    let mut out: Owners = BTreeMap::new();
    for (i, n) in nodes.iter().enumerate() {
        for f in catalog::xml_files(&n.root) {
            for e in catalog::entries_in(&f).with_context(|| format!("解析 {}", f.display()))? {
                out.entry((e.kind, e.id)).or_default().push(i);
            }
        }
    }
    Ok(out)
}