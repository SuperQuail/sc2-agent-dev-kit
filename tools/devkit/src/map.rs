//! 读 `.SC2Mod` / `.SC2Map` 目录：里面有什么、关键文件在哪。
//!
//! 不解析内容——那是 `catalog` / `triggers` / `galaxy` 的事。
use anyhow::{bail, Context, Result};
use std::path::{Path, PathBuf};

/// 往 `DocumentHeader` 里追加一条依赖。
///
/// 实测格式（偏移 44 起）：
///
/// ```text
/// 04 00 00 00                         u32 依赖条数
/// "file:Mods\kit_liberty_story.SC2Mod\0"   NUL 结尾的 UTF-8
/// "file:Mods\3疯批帝国之翼.SC2Mod\0"
/// ```
///
/// 所以加一条 = 计数 +1、在最后一条之后插入。
/// 用「第一个 `file:` 的位置」来定位计数，不硬编码偏移。
pub fn add_dependency(header: &[u8], dep: &str) -> Result<Vec<u8>> {
    // 依赖区固定在偏移 44：u32 条数，随后是 N 个 NUL 结尾的 UTF-8 串。
    //
    // **不能靠「第一个 file:」定位** —— 依赖串有两种写法：
    //   file:Mods\X.SC2Mod
    //   bnet:Liberty (Mod)/0.0/999,file:Mods/Liberty.SC2Mod
    // 后者里的 file: 前面是 bnet 名字，往前 4 字节不是条数。
    let count_at = 44usize;
    if header.len() < 48 {
        bail!("DocumentHeader 太短");
    }
    let first = 48usize;
    let count = u32::from_le_bytes(header[count_at..first].try_into().unwrap()) as usize;
    if count > 256 {
        bail!("依赖条数异常：{count}");
    }

    // 依次跨过 count 条 NUL 结尾串，得到插入点。
    let mut at = first;
    for _ in 0..count {
        let end = header[at..].iter().position(|&c| c == 0).context("依赖串未以 NUL 结尾")? + at;
        at = end + 1;
    }

    let mut out = Vec::with_capacity(header.len() + dep.len() + 1);
    out.extend_from_slice(&header[..count_at]);
    out.extend_from_slice(&((count + 1) as u32).to_le_bytes());
    out.extend_from_slice(&header[first..at]);
    out.extend_from_slice(dep.as_bytes());
    out.push(0);
    out.extend_from_slice(&header[at..]);
    Ok(out)
}

/// 一个模块目录（.SC2Mod 或 .SC2Map）里值得关心的东西。
#[derive(Debug, Clone)]
pub struct Layout {
    pub root: PathBuf,
    /// 触发器图（XML）。生成物是 MapScript.galaxy，那个不能改。
    pub triggers: Option<PathBuf>,
    /// 生成的 Galaxy，**只读**：用来找生成点，改它会被编辑器覆盖。
    pub mapscript: Option<PathBuf>,
    /// 本地的 GameData 覆盖（`Base.SC2Data/GameData/*.xml`）。
    pub gamedata: Vec<PathBuf>,
    /// `ComponentList.SC2Components`——它声明了依赖。
    pub component_list: Option<PathBuf>,
}

pub fn layout(root: &Path) -> Result<Layout> {
    if !root.is_dir() {
        anyhow::bail!("不是目录：{}", root.display());
    }
    let at = |rel: &str| {
        let p = root.join(rel);
        p.is_file().then_some(p)
    };
    let mut gamedata: Vec<PathBuf> = Vec::new();
    let gd = root.join("Base.SC2Data/GameData");
    if gd.is_dir() {
        for e in std::fs::read_dir(&gd)?.flatten() {
            let p = e.path();
            if p.extension().is_some_and(|x| x.eq_ignore_ascii_case("xml")) {
                gamedata.push(p);
            }
        }
        gamedata.sort();
    }

    Ok(Layout {
        root: root.to_path_buf(),
        triggers: at("Triggers"),
        mapscript: at("MapScript.galaxy"),
        gamedata,
        component_list: at("ComponentList.SC2Components"),
    })
}