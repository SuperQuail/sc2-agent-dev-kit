//! 读 `.SC2Mod` / `.SC2Map` 目录：里面有什么、关键文件在哪。
//!
//! 不解析内容——那是 `catalog` / `triggers` / `galaxy` 的事。
use anyhow::Result;
use std::path::{Path, PathBuf};

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