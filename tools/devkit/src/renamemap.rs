//! 改名映射表：**落在工作目录里的一个 TSV**，不是内存状态。
//!
//! 为什么必须是文件：改名要跨很多次会话、很多个依赖。谁被改过、为了躲谁、改到哪一步，
//! 全都得能查、能审、能重跑。内存里做完就丢，等于每次都从零推一遍链路。
//!
//! 为什么是 TSV 而不是 SQLite：这张表要能 §git diff§、能人眼读、能 grep，
//! 而且 §sc2cat§ 目前零额外依赖（clap / quick-xml / anyhow），TSV 原生读写。
//! 真到了需要索引和联表的规模（比如整个引用图），再上 SQLite 不迟。
//!
//! 列：§old_id  new_id  table  dependency  status§
//!   dependency —— 这处改名是为了躲谁（审计线索）
//!   status     —— §done§ / §todo§，§apply§ 只动 §todo§，§check§ 只查 §done§
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

/// 工作目录里的默认文件名。
pub const MAP_FILE: &str = "sc2cat-rename-map.tsv";

/// 在若干目录树里数一个 id 出现几次（**按标识符边界**）。
///
/// 边界规则和 `rename::rename` 一致：前后不是 `[A-Za-z0-9_]` 才算一个完整的 id。
/// 这样 `ResearchZeratulZealotBlinkHeal` 里的 `ZeratulZealotBlinkHeal` 不会被误计。
pub fn count_id(roots: &[PathBuf], id: &str) -> usize {
    let needle: Vec<char> = id.chars().collect();
    let ident = |c: char| c.is_ascii_alphanumeric() || c == '_';
    let mut n = 0usize;
    for root in roots {
        for f in walk(root) {
            let Ok(raw) = std::fs::read(&f) else { continue };
            let Ok(text) = String::from_utf8(raw) else { continue };
            let cs: Vec<char> = text.chars().collect();
            let mut i = 0usize;
            while i + needle.len() <= cs.len() {
                if cs[i..i + needle.len()] == needle[..] {
                    let prev_ok = i == 0 || !ident(cs[i - 1]);
                    let next_ok = i + needle.len() >= cs.len() || !ident(cs[i + needle.len()]);
                    if prev_ok && next_ok {
                        n += 1;
                        i += needle.len();
                        continue;
                    }
                }
                i += 1;
            }
        }
    }
    n
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

/// 一次守恒检查的结论。
pub struct Verdict {
    pub old_id: String,
    pub before: usize,
    pub after_old: usize,
    pub after_new: usize,
}

impl Verdict {
    /// 判据：旧 id **一个都不剩**，且新 id 的数量**不少于**原来的。
    ///
    /// 为什么用守恒而不是「悬空引用」：改名后旧 id 往往**仍然被别人定义**着
    /// （实测 `ZeratulZealotWhirlwind` 被 StarCoop 定义），悬空检查报不出来。
    /// 守恒判据**不依赖闭包是否完整**。
    pub fn ok(&self) -> bool {
        self.after_old == 0 && self.after_new >= self.before
    }
}

/// 对映射表里每个 `done` 行跑守恒检查。`baseline` 是改动前的树。",
pub fn check(map: &Map, baseline: &[PathBuf], current: &[PathBuf]) -> Vec<Verdict> {
    map.done()
        .into_iter()
        .map(|r| Verdict {
            old_id: r.old_id.clone(),
            before: count_id(baseline, &r.old_id),
            after_old: count_id(current, &r.old_id),
            after_new: count_id(current, &r.new_id),
        })
        .collect()
}

#[derive(Debug, Clone)]
pub struct Row {
    pub old_id: String,
    pub new_id: String,
    pub table: String,
    pub dependency: String,
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct Map {
    pub path: PathBuf,
    pub rows: Vec<Row>,
}

impl Map {
    /// 读；文件不存在就返回空表（不是错误 —— 第一次用就是要新建）。
    pub fn load(path: &Path) -> Result<Map> {
        let mut rows = Vec::new();
        if path.is_file() {
            let text = std::fs::read_to_string(path)
                .with_context(|| format!("读不了 {}", path.display()))?;
            for line in text.lines() {
                let l = line.trim_end();
                if l.is_empty() || l.starts_with('#') {
                    continue;
                }
                let f: Vec<&str> = l.split('\t').collect();
                if f.len() < 2 || f[0] == "old_id" {
                    continue;
                }
                rows.push(Row {
                    old_id: f[0].to_string(),
                    new_id: f[1].to_string(),
                    table: f.get(2).unwrap_or(&"").to_string(),
                    dependency: f.get(3).unwrap_or(&"").to_string(),
                    status: f.get(4).unwrap_or(&"todo").to_string(),
                });
            }
        }
        Ok(Map { path: path.to_path_buf(), rows })
    }

    pub fn save(&self) -> Result<()> {
        let mut out = String::from(
            "# sc2cat 改名映射表 —— 一行一个 id。加新依赖 = 加行 + apply + check，不用重推链路。\n\
             # dependency 列是审计线索（这处改名是为了躲谁）；status 列 done/todo 可批量重跑。\n\
             old_id\tnew_id\ttable\tdependency\tstatus\n",
        );
        for r in &self.rows {
            out.push_str(&format!(
                "{}\t{}\t{}\t{}\t{}\n",
                r.old_id, r.new_id, r.table, r.dependency, r.status
            ));
        }
        std::fs::write(&self.path, out)
            .with_context(|| format!("写不了 {}", self.path.display()))?;
        Ok(())
    }

    /// 加一行（同 old_id 就更新），返回是否新增。
    pub fn upsert(&mut self, row: Row) -> bool {
        match self.rows.iter_mut().find(|r| r.old_id == row.old_id) {
            Some(r) => {
                *r = row;
                false
            }
            None => {
                self.rows.push(row);
                true
            }
        }
    }

    /// 待改的行（§apply§ 用）。
    pub fn todo(&self) -> Vec<&Row> {
        self.rows.iter().filter(|r| r.status != "done").collect()
    }

    /// 已改的行（§check§ 用）。
    pub fn done(&self) -> Vec<&Row> {
        self.rows.iter().filter(|r| r.status == "done").collect()
    }
}