//! 从某个单位出发的 catalog 引用闭包。
//!
//! 用途：依赖冲突上万处，但和我们有关的只是**从目标单位能走到的那一小撮**。
//!
//! ## 哪些字段算引用
//! **不手写白名单**（那是猜）。改成从数据里学：统计每个属性名的取值中
//! 「有多少是已定义的 catalog id」，命中率高的才算引用属性。
//! 这样它对手上任何一份 mod 都成立，不依赖我记住 SC2 的字段名。
use anyhow::Result;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::path::Path;

use crate::catalog;

const REF_HIT_RATE: f64 = 0.6;
const REF_MIN_SAMPLES: usize = 5;

pub struct Defined {
    pub by_id: BTreeMap<String, (String, std::path::PathBuf, BTreeSet<String>)>,
    pub ref_attrs: BTreeSet<String>,
}

fn bodies(root: &Path) -> Result<Vec<(String, String, std::path::PathBuf, String)>> {
    let mut out = Vec::new();
    for f in catalog::xml_files(root) {
        let text = std::fs::read_to_string(&f).unwrap_or_default();
        for e in catalog::entries_in(&f).unwrap_or_default() {
            let open = format!("<{} id=\"{}\"", e.kind, e.id);
            let Some(a) = text.find(&open) else { continue };
            let tail = &text[a..];
            let close = format!("</{}>", e.kind);
            let body = match tail.find(&close) {
                Some(b) => tail[..b].to_string(),
                None => tail[..(4000).min(tail.len())].to_string(),
            };
            out.push((e.kind, e.id, f.clone(), body));
        }
    }
    Ok(out)
}

/// 只认**真属性**：名字必须在标签名之后（前面是空白）。
///
/// 踩过：不分标签名和属性名时，`<AbilArray Link="stop"/>` 会被记成 `(AbilArray, "stop")`，
/// 于是**每个带 id= 的标签都「命中」**，学出 328 个假引用属性 —— 等于没学。
fn attrs_in(body: &str) -> Vec<(String, String)> {
    let cs: Vec<char> = body.chars().collect();
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < cs.len() {
        if !cs[i].is_ascii_whitespace() || i + 1 >= cs.len() || !cs[i + 1].is_ascii_alphabetic() {
            i += 1;
            continue;
        }
        i += 1;
        let a = i;
        while i < cs.len() && (cs[i].is_ascii_alphanumeric() || cs[i] == '_') {
            i += 1;
        }
        let name: String = cs[a..i].iter().collect();
        while i < cs.len() && cs[i] != '"' {
            i += 1;
        }
        if i >= cs.len() {
            break;
        }
        let b = i + 1;
        let mut j = b;
        while j < cs.len() && cs[j] != '"' {
            j += 1;
        }
        let val: String = cs[b..j].iter().collect();
        out.push((name, val));
        i = j + 1;
    }
    out
}

pub fn scan(root: &Path) -> Result<Defined> {
    let rows = bodies(root)?;
    let ids: BTreeSet<String> = rows.iter().map(|r| r.1.clone()).collect();
    let mut hit: BTreeMap<String, (usize, usize)> = BTreeMap::new();
    for (_, _, _, body) in &rows {
        for (name, val) in attrs_in(body) {
            if val.is_empty() || val.len() >= 64 {
                continue;
            }
            let e = hit.entry(name).or_insert((0, 0));
            e.1 += 1;
            if ids.contains(&val) {
                e.0 += 1;
            }
        }
    }
    let ref_attrs: BTreeSet<String> = hit
        .iter()
        .filter(|(_, (h, t))| *t >= REF_MIN_SAMPLES && (*h as f64) / (*t as f64) >= REF_HIT_RATE)
        .map(|(k, _)| k.clone())
        .collect();
    // **按 id 合并，不覆盖**：同一个 id 在几张表里都存在（`ZeratulZealot` 既是
    // `CUnit` 又是 `CActorUnit`）。早先按 id 单键插入，后者把前者顶掉，
    // 结果从单位出发只能走到单位的链，**actor 那条链整个丢了** —— 而那正是模型所在。
    let mut by_id: BTreeMap<String, (String, std::path::PathBuf, BTreeSet<String>)> = BTreeMap::new();
    for (kind, id, f, body) in rows {
        let mut refs = BTreeSet::new();
        for (name, val) in attrs_in(&body) {
            if !ref_attrs.contains(&name) {
                continue;
            }
            let tok = val.split(',').next().unwrap_or("").trim().to_string();
            if ids.contains(&tok) && tok != id {
                refs.insert(tok);
            }
        }
        match by_id.get_mut(&id) {
            Some((k, _, old)) => {
                if !k.split('+').any(|x| x == kind) {
                    *k = format!("{k}+{kind}");
                }
                old.extend(refs);
            }
            None => {
                by_id.insert(id, (kind, f, refs));
            }
        }
    }
    Ok(Defined { by_id, ref_attrs })
}

pub fn reachable(d: &Defined, starts: &[String]) -> BTreeSet<String> {
    let mut seen = BTreeSet::new();
    let mut q: VecDeque<String> = starts.iter().cloned().collect();
    while let Some(id) = q.pop_front() {
        if !seen.insert(id.clone()) {
            continue;
        }
        if let Some((_, _, refs)) = d.by_id.get(&id) {
            for r in refs {
                if d.by_id.contains_key(r) && !seen.contains(r) {
                    q.push_back(r.clone());
                }
            }
        }
    }
    seen
}