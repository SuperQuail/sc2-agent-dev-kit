//! 本地化：把 CASC 抽出的**源形态**整成**分发形态**，以及就地改字符串。
//!
//! 两个坑换来的：
//! 1. CASC 里的目录是全小写（`zhcn.sc2data`），而所有能跑的成品图都是驼峰
//!    且带齐 12 个语系 —— 只声明 1 个语系的图，文本组件不生效，画面全是裸键。
//! 2. Blizzard 的本地化文件带 UTF-8 BOM，手写的不带，游戏读不到。
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

/// 标准和成品图一致的语系集合（顺序也照抄）。
const LOCALES: [&str; 12] = [
    "enUS", "koKR", "esES", "deDE", "frFR", "ruRU", "zhCN", "zhTW", "ptBR", "plPL", "esMX", "itIT",
];

const BOM: &[u8] = b"\xef\xbb\xbf";

/// 把一个展开的地图/模组目录整成可分发的形态：
///
/// - 语系目录改成驼峰（`zhcn.sc2data` -> `zhCN.SC2Data`）
/// - 补齐 12 个语系（缺的用已有的第一份内容填，`shortcut:` 见下）
/// - `ComponentList` 按实际存在的语系重写 `text` 条目
///
/// shortcut: 补齐的语系是**内容复制**，不是真翻译；要出多语言就得逐份替换。
pub fn normalize(root: &Path) -> Result<String> {
    // 1) 收集现有语系内容：<locale> -> { 文件名 -> 字节 }
    let mut found: Vec<(String, PathBuf)> = Vec::new();
    for e in std::fs::read_dir(root)? {
        let p = e?.path();
        if !p.is_dir() {
            continue;
        }
        let name = p.file_name().unwrap().to_string_lossy().to_string();
        let lower = name.to_ascii_lowercase();
        if !lower.ends_with(".sc2data") {
            continue;
        }
        let loc = name[..name.len() - ".SC2Data".len()].to_string();
        // `base.sc2data` 是地图本地 GameData，不是语系
        if loc.eq_ignore_ascii_case("base") {
            continue;
        }
        found.push((loc, p));
    }
    if found.is_empty() {
        anyhow::bail!("没有找到任何 <locale>.SC2Data 目录");
    }

    // 2) 选一份当模板：优先 zhCN，其次 enUS，再其次第一个
    let pick = |want: &str| found.iter().find(|(l, _)| l.eq_ignore_ascii_case(want)).map(|(_, p)| p.clone());
    let template = pick("zhCN")
        .or_else(|| pick("enUS"))
        .or_else(|| found.first().map(|(_, p)| p.clone()))
        .unwrap();
    let mut files: Vec<(String, Vec<u8>)> = Vec::new();
    collect_files(&template, &template, &mut files)?;

    // 3) 全部重建成驼峰 + 12 语系
    for (_, p) in &found {
        let _ = std::fs::remove_dir_all(p);
    }
    for loc in LOCALES {
        let dst = root.join(format!("{loc}.SC2Data"));
        for (rel, bytes) in &files {
            let out = dst.join(rel);
            if let Some(parent) = out.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::write(&out, bytes)?;
        }
    }

    // 4) ComponentList 重写 text 条目
    let cl = root.join("componentlist.sc2components");
    let cl = if cl.is_file() { cl } else { root.join("ComponentList.SC2Components") };
    let xml = std::fs::read_to_string(&cl).context("读不了 ComponentList")?;
    let mut out = String::new();
    let mut done = false;
    for line in xml.lines() {
        if line.contains("Type=\"text\"") {
            if !done {
                for loc in LOCALES {
                    out.push_str(&format!("    <DataComponent Type=\"text\" Locale=\"{loc}\">GameText</DataComponent>\n"));
                }
                done = true;
            }
            continue;
        }
        out.push_str(line);
        out.push('\n');
    }
    if !done {
        // 原本一条 text 都没有：插在末尾之前
        out = out.replace(
            "</Components>",
            &format!(
                "{}\n</Components>",
                LOCALES
                    .iter()
                    .map(|l| format!("    <DataComponent Type=\"text\" Locale=\"{l}\">GameText</DataComponent>"))
                    .collect::<Vec<_>>()
                    .join("\n")
            ),
        );
    }
    std::fs::write(&cl, out)?;

    Ok(format!(
        "归一化 {} 个语系为 12 个驼峰目录，ComponentList 已重写（模板来自 {}）",
        found.len(),
        template.file_name().unwrap().to_string_lossy()
    ))
}

fn collect_files(base: &Path, dir: &Path, out: &mut Vec<(String, Vec<u8>)>) -> Result<()> {
    for e in std::fs::read_dir(dir)? {
        let p = e?.path();
        if p.is_dir() {
            collect_files(base, &p, out)?;
        } else {
            let rel = p.strip_prefix(base)?.to_path_buf();
            out.push((rel.display().to_string(), std::fs::read(&p)?));
        }
    }
    Ok(())
}

/// 就地设置若干本地化键：命中已有键就**替换**（不是追加 —— 追加时先出现的旧值会赢），
/// 没有就追加。写回时保留原文件的 BOM。
pub fn set_keys(root: &Path, locale: &str, pairs: &[(String, String)]) -> Result<String> {
    let dir = root.join(format!("{locale}.SC2Data")).join("LocalizedData");
    std::fs::create_dir_all(&dir)?;
    let file = dir.join("ObjectStrings.txt");
    let raw = std::fs::read(&file).unwrap_or_default();
    let bom = raw.starts_with(BOM);
    let text = String::from_utf8_lossy(raw.strip_prefix(BOM).unwrap_or(&raw)).to_string();

    let mut lines: Vec<String> = text.lines().map(str::to_string).collect();
    let mut replaced = 0usize;
    let mut added = 0usize;
    for (k, v) in pairs {
        match lines.iter_mut().find(|l| l.split('=').next() == Some(k.as_str())) {
            Some(l) => {
                *l = format!("{k}={v}");
                replaced += 1;
            }
            None => {
                lines.push(format!("{k}={v}"));
                added += 1;
            }
        }
    }
    let mut out: Vec<u8> = Vec::new();
    if bom {
        out.extend_from_slice(BOM);
    }
    out.extend_from_slice(lines.join("\n").as_bytes());
    out.push(b'\n');
    std::fs::write(&file, out)?;
    Ok(format!("{}: 替换 {} 条, 新增 {} 条", file.display(), replaced, added))
}
