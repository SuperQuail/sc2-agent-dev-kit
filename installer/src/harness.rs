//! 各 harness 的技能目录。
//!
//! 路径是在真机上逐个列目录确认过的，不是猜的。四个都遵循扁平约定
//! <skills-dir>/<name>/SKILL.md，而套件把 19 个子技能嵌在 skills/galaxy/ 与
//! skills/sc2data/ 下，所以安装时要拍平。

use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct Harness {
    pub id: &'static str,
    pub name: &'static str,
    pub config_root: PathBuf,
    pub skills_dir: PathBuf,
    pub installed: bool,
    pub existing_skills: usize,
}

fn home() -> PathBuf {
    dirs::home_dir().unwrap_or_else(|| PathBuf::from("."))
}

fn count_dirs(p: &std::path::Path) -> usize {
    std::fs::read_dir(p)
        .map(|rd| rd.filter_map(Result::ok).filter(|e| e.path().is_dir()).count())
        .unwrap_or(0)
}

pub fn detect_all() -> Vec<Harness> {
    let h = home();
    let specs: [(&'static str, &'static str, PathBuf, PathBuf); 4] = [
        ("dsh", "DeepSeek Harness", h.join(".dsh"), h.join(".dsh/skills")),
        ("claude", "Claude Code", h.join(".claude"), h.join(".claude/skills")),
        ("codex", "Codex", h.join(".codex"), h.join(".codex/skills")),
        (
            "opencode",
            "opencode",
            h.join(".config/opencode"),
            h.join(".config/opencode/skills"),
        ),
    ];
    specs
        .into_iter()
        .map(|(id, name, config_root, skills_dir)| {
            // 判定依据是配置根目录存在；skills/ 子目录按需创建。
            let installed = config_root.is_dir();
            let existing_skills = if skills_dir.is_dir() { count_dirs(&skills_dir) } else { 0 };
            Harness { id, name, config_root, skills_dir, installed, existing_skills }
        })
        .collect()
}
