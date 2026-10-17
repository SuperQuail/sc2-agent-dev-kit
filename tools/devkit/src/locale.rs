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

/// 按前缀裁掉某些本地化键，覆盖所有语系目录。
///
/// 为什么要这个：第三方 mod 常常是「在某个战役基础上做的」，作者导出时会把**整个基础战役的
/// 文本**一起抄进自己包里。那份文本是**英文**，而且覆盖的是战役自己的键 —— 打进包后，
/// 玩家会看到「战役原文变英文」，而地图/战役自己的中文被压住。
///
/// 裁掉 `Conversation/` / `ConversationState/` / `Param/` 这类**战役专属前缀**即可：
/// mod 自己的 `Button/` / `Effect/` / `Unit/` / `UserData/` 都留着。
/// 从 `reference` 文件里读出全部键，把这些键从 `root` 下所有语系里删掉。
///
/// 这是「按影子裁」：第三方 mod 若抄了基础战役的整份文本，那些键和战役自己的冲突。
/// **只删战役也定义过的键** —— mod 自己独有的键（对话字幕、`StringExternal` 取的参数）全留着。
/// 按前缀粗暴裁会误删后者，这是实测踩过的。
pub fn prune_shadowing(root: &Path, reference: &Path) -> Result<String> {
    let raw = std::fs::read(reference).with_context(|| format!("读不了 {}", reference.display()))?;
    let body = String::from_utf8_lossy(raw.strip_prefix(BOM).unwrap_or(&raw)).to_string();
    let keys: std::collections::HashSet<String> = body
        .lines()
        .filter(|l| l.contains('='))
        .map(|l| l.split('=').next().unwrap_or("").to_string())
        .collect();

    let mut n = 0usize;
    let mut files = 0usize;
    for loc in std::fs::read_dir(root)? {
        let loc = loc?.path();
        let name = loc.file_name().unwrap_or_default().to_string_lossy().to_ascii_lowercase();
        if !loc.is_dir() || !name.ends_with(".sc2data") || name == "base.sc2data" {
            continue;
        }
        let data = loc.join("LocalizedData");
        if !data.is_dir() {
            continue;
        }
        for txt in std::fs::read_dir(&data)?.flatten().map(|e| e.path()) {
            if !txt.is_file() {
                continue;
            }
            let raw = std::fs::read(&txt)?;
            let bom = raw.starts_with(BOM);
            let body = String::from_utf8_lossy(raw.strip_prefix(BOM).unwrap_or(&raw)).to_string();
            let mut kept: Vec<&str> = Vec::new();
            let mut dropped = 0usize;
            for line in body.lines() {
                let k = line.split('=').next().unwrap_or("");
                if line.contains('=') && keys.contains(k) {
                    dropped += 1;
                } else {
                    kept.push(line);
                }
            }
            if dropped > 0 {
                let mut out = Vec::new();
                if bom {
                    out.extend_from_slice(BOM);
                }
                out.extend_from_slice(kept.join("\n").as_bytes());
                out.push(b'\n');
                std::fs::write(&txt, out)?;
                n += dropped;
                files += 1;
            }
        }
    }
    Ok(format!("按 {} 的影子裁掉 {} 个键（{} 个文件）", reference.display(), n, files))
}

/// 按前缀裁（粗暴版）。要精确请用 `prune_shadowing`。
pub fn prune(root: &Path, drop_prefixes: &[String]) -> Result<String> {
    let mut dropped = 0usize;
    let mut files = 0usize;
    let mut touched = Vec::new();
    for e in std::fs::read_dir(root)? {
        let p = e?.path();
        if !p.is_dir() || !p.to_string_lossy().to_ascii_lowercase().contains(".sc2data") {
            continue;
        }
        for f in std::fs::read_dir(&p)?.flatten().map(|e| e.path()) {
            if !f.is_dir() {
                continue;
            }
            for txt in std::fs::read_dir(&f)?.flatten().map(|e| e.path()) {
                if !txt.is_file() {
                    continue;
                }
                let raw = std::fs::read(&txt)?;
                let bom = raw.starts_with(BOM);
                let body = String::from_utf8_lossy(raw.strip_prefix(BOM).unwrap_or(&raw)).to_string();
                let mut kept = Vec::new();
                let mut n = 0usize;
                for line in body.lines() {
                    let k = line.split('=').next().unwrap_or("");
                    // 只裁「有 = 的键行」；注释/空行保留
                    if line.contains('=') && drop_prefixes.iter().any(|p| k.starts_with(p.as_str())) {
                        n += 1;
                    } else {
                        kept.push(line);
                    }
                }
                if n > 0 {
                    let mut out = Vec::new();
                    if bom {
                        out.extend_from_slice(BOM);
                    }
                    out.extend_from_slice(kept.join("\n").as_bytes());
                    out.push(b'\n');
                    std::fs::write(&txt, out)?;
                    dropped += n;
                    files += 1;
                    touched.push(format!("{}/{}: -{}", p.file_name().unwrap().to_string_lossy(), txt.file_name().unwrap().to_string_lossy(), n));
                }
            }
        }
    }
    Ok(format!("裁掉 {} 个键（{} 个文件）：{}", dropped, files, touched.join(", ")))
}

/// 就地设置若干本地化键：命中已有键就**替换**（不是追加 —— 追加时先出现的旧值会赢），
/// 没有就追加。写回时保留原文件的 BOM。
///
/// **两张表都写**：`Unit/Name/*` 与 `Button/Tooltip/*` 在 `GameStrings`，编辑器导出的对象名
/// 却常在 `ObjectStrings` —— 分不清该进哪张时，两边都放一份最省事（重复无害）。
pub fn set_keys(root: &Path, locale: &str, pairs: &[(String, String)]) -> Result<String> {
    let dir = root.join(format!("{locale}.SC2Data")).join("LocalizedData");
    std::fs::create_dir_all(&dir)?;
    let mut notes = Vec::new();
    for name in ["ObjectStrings.txt", "GameStrings.txt"] {
        let (replaced, added) = set_keys_in(&dir.join(name), pairs)?;
        notes.push(format!("{name}: 替换 {replaced}, 新增 {added}"));
    }
    Ok(format!("{locale} → {}", notes.join(" | ")))
}

fn set_keys_in(file: &Path, pairs: &[(String, String)]) -> Result<(usize, usize)> {
    let raw = std::fs::read(file).unwrap_or_default();
    let bom = raw.starts_with(BOM);
    let text = String::from_utf8_lossy(raw.strip_prefix(BOM).unwrap_or(&raw)).to_string();

    let mut lines: Vec<String> = text.lines().map(str::to_string).collect();
    let (mut replaced, mut added) = (0usize, 0usize);
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
    std::fs::write(file, out)?;
    Ok((replaced, added))
}
