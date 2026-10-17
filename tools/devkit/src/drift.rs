//! 依赖漂移报告：我们交付的这份拷贝，相对**原始依赖**改了哪些文件。
//!
//! 为什么需要它：从别人做好的 mod 出发做迁移，很容易在「只改一点」的过程中
//! 顺手丢掉或改坏不属于自己的东西，而且**改完才发现**。
//! 交付前跑一遍，任何差异都必须能一行行解释清楚。
//!
//! **判据**：`丢了 0 个` 是硬要求；`改了 N 个` 必须逐条能说明为什么。
use anyhow::{Context, Result};
use std::collections::BTreeMap;
use std::path::Path;

pub struct Diff {
    pub lost: Vec<String>,
    pub added: Vec<String>,
    pub changed: Vec<(String, u64, u64)>,
}

impl Diff {
    pub fn clean(&self) -> bool {
        self.lost.is_empty() && self.added.is_empty() && self.changed.is_empty()
    }
}

/// 逐文件比对两棵树（按相对路径 + 大小 + 内容）。
pub fn compare(original: &Path, ours: &Path) -> Result<Diff> {
    let a = collect(original)?;
    let b = collect(ours)?;
    let mut lost = Vec::new();
    let mut added = Vec::new();
    let mut changed = Vec::new();
    for (k, pa) in &a {
        match b.get(k) {
            None => lost.push(k.clone()),
            Some(pb) => {
                let (sa, sb) = (pa.metadata()?.len(), pb.metadata()?.len());
                if sa != sb || std::fs::read(pa)? != std::fs::read(pb)? {
                    changed.push((k.clone(), sa, sb));
                }
            }
        }
    }
    for k in b.keys() {
        if !a.contains_key(k) {
            added.push(k.clone());
        }
    }
    Ok(Diff { lost, added, changed })
}

fn collect(root: &Path) -> Result<BTreeMap<String, std::path::PathBuf>> {
    if !root.is_dir() {
        anyhow::bail!("不是目录：{}", root.display());
    }
    let mut out = BTreeMap::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        for e in std::fs::read_dir(&dir)?.flatten() {
            let p = e.path();
            if p.is_dir() {
                stack.push(p);
            } else {
                let rel = p
                    .strip_prefix(root)
                    .with_context(|| format!("路径异常：{}", p.display()))?
                    .display()
                    .to_string()
                    .replace('\\', "/");
                out.insert(rel, p);
            }
        }
    }
    Ok(out)
}
