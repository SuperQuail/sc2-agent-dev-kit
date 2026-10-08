//! 安装与预演的任务入口。两者都在后台线程运行。

use crate::harness::Harness;
use crate::install::{
    copy_dir, diff_files, extract_zip, install_skill, read_state, Msg, PlanInfo, Summary, KIT_DIR,
    STATE_FILE,
};
use crate::manifest::Manifest;
use crate::source;
use std::path::PathBuf;
use std::sync::mpsc::Sender;

/// 把消息送回 UI 并唤醒界面。
///
/// 持有 egui Context 的弱引用式回调：引擎不依赖 egui，UI 也不必轮询。
pub struct Reporter {
    tx: Sender<Msg>,
    wake: std::sync::Arc<dyn Fn() + Send + Sync>,
}

impl Reporter {
    pub fn new(tx: Sender<Msg>, wake: std::sync::Arc<dyn Fn() + Send + Sync>) -> Self {
        Self { tx, wake }
    }
    pub fn send(&self, m: Msg) {
        let _ = self.tx.send(m);
        (self.wake)();
    }
    pub fn log(&self, s: impl Into<String>) {
        self.send(Msg::Log(s.into()));
    }
}

/// 只做差异计算，不写任何东西。
pub fn run_plan(from: String, root: PathBuf, proxy: Option<String>, rep: Reporter) {
    let result = (|| -> Result<PlanInfo, String> {
        rep.send(Msg::Phase("解析来源".into()));
        let mut say = |s: String| rep.send(Msg::Log(s));
        // 预演不写任何文件，所以只取清单，不把 20 MB 的包也拖下来。
        let r = source::resolve(&from, proxy.as_deref(), false, &mut say)?;
        let kit = root.join(KIT_DIR);
        rep.send(Msg::Phase("核对文件".into()));
        let core = diff_files(&kit, &r.manifest.files.core);
        let data = diff_files(&kit, &r.manifest.files.data);
        let data_pack = r.manifest.artifacts.data.clone();
        let data_identical = data.total > 0 && data.missing == 0 && data.stale == 0;
        let state = read_state(&root);
        let skip_data = data_identical
            && data_pack
                .as_ref()
                .and_then(|d| d.sha256.clone())
                .zip(state.as_ref().and_then(|s| {
                    s.get("artifacts")?.get("data")?.as_str().map(str::to_string)
                }))
                .map(|(a, b)| a == b)
                .unwrap_or(false);
        Ok(PlanInfo {
            version: r.manifest.version.clone(),
            layout_revision: r.manifest.layout_revision,
            skills: r.manifest.skills.len(),
            core,
            data,
            data_mb: data_pack.as_ref().map(|d| d.uncompressed_bytes as f64 / 1048576.0).unwrap_or(0.0),
            skip_data,
            kit_path: kit,
        })
    })();
    match result {
        Ok(p) => rep.send(Msg::PlanReady(p)),
        Err(e) => rep.send(Msg::Done(Err(e))),
    }
}

/// 真正执行安装。整段跑在后台线程，绝不阻塞 UI。
pub fn run_install(
    from: String,
    root: PathBuf,
    proxy: Option<String>,
    selected: Vec<String>,
    banner: bool,
    rep: Reporter,
) {
    let out = (|| -> Result<Summary, String> {
        rep.send(Msg::Phase("解析来源".into()));
        let mut say = |s: String| rep.send(Msg::Log(s));
        let r = source::resolve(&from, proxy.as_deref(), true, &mut say)?;
        let manifest: &Manifest = &r.manifest;
        let kit = root.join(KIT_DIR);
        std::fs::create_dir_all(&kit).map_err(|e| format!("建不了套件目录：{e}"))?;

        let targets: Vec<Harness> = crate::harness::detect_all()
            .into_iter()
            .filter(|h| h.installed && selected.contains(&h.id.to_string()))
            .collect();
        if targets.is_empty() {
            return Err("没有选中任何 harness，或选中的都没检测到".into());
        }
        rep.log(format!(
            "目标：{}",
            targets.iter().map(|h| h.id).collect::<Vec<_>>().join(", ")
        ));

        // 先判断数据包能否跳过，再算总步数，进度条才能是确定的而非转圈。
        let data_before = diff_files(&kit, &manifest.files.data);
        let data_pack = manifest.artifacts.data.clone();
        let state = read_state(&root);
        let skip_data = data_before.total > 0
            && data_before.missing == 0
            && data_before.stale == 0
            && data_pack
                .as_ref()
                .and_then(|d| d.sha256.clone())
                .zip(state.as_ref().and_then(|s| {
                    s.get("artifacts")?.get("data")?.as_str().map(str::to_string)
                }))
                .map(|(a, b)| a == b)
                .unwrap_or(false);

        let core_files = manifest.files.core.len();
        let data_files = if skip_data { 0 } else { manifest.files.data.len() };
        let skill_steps = manifest.skills.len() * targets.len();
        let total_steps = core_files.max(1) + data_files + skill_steps.max(1) + 2;
        rep.send(Msg::Total(total_steps));
        let mut done = 0usize;
        let mut bump = |label: &str, done: &mut usize| {
            *done += 1;
            rep.send(Msg::Step { done: *done, total: total_steps, label: label.to_string() });
        };

        rep.send(Msg::Phase("解压套件本体".into()));
        let stage = root.join(format!(".stage-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&stage);
        std::fs::create_dir_all(&stage).map_err(|e| format!("建暂存目录失败：{e}"))?;
        let core_zip = r.dir.join(
            manifest
                .artifacts
                .core
                .file
                .clone()
                .ok_or("清单里没有 core 包名")?,
        );
        extract_zip(&core_zip, &stage, &mut |name| bump(name, &mut done))?;
        for e in std::fs::read_dir(&stage).map_err(|e| e.to_string())?.filter_map(Result::ok) {
            let to = kit.join(e.file_name());
            if to.exists() {
                let _ = std::fs::remove_dir_all(&to);
                let _ = std::fs::remove_file(&to);
            }
            if e.path().is_dir() {
                copy_dir(&e.path(), &to)?;
            } else {
                std::fs::copy(e.path(), &to).map_err(|e| e.to_string())?;
            }
        }
        let _ = std::fs::remove_dir_all(&stage);

        if let Some(dp) = &data_pack {
            if let Some(file) = &dp.file {
                if !skip_data {
                    rep.send(Msg::Phase(format!(
                        "解压游戏数据（{:.1} MB）",
                        dp.uncompressed_bytes as f64 / 1048576.0
                    )));
                    let ds = root.join(format!(".stage-data-{}", std::process::id()));
                    let _ = std::fs::remove_dir_all(&ds);
                    std::fs::create_dir_all(&ds).map_err(|e| e.to_string())?;
                    extract_zip(&r.dir.join(file), &ds, &mut |name| bump(name, &mut done))?;
                    copy_dir(&ds, &kit)?;
                    let _ = std::fs::remove_dir_all(&ds);
                } else {
                    rep.send(Msg::Log(format!(
                        "数据包哈希一致，整体跳过（{:.1} MB 未重写）",
                        dp.uncompressed_bytes as f64 / 1048576.0
                    )));
                    for _ in 0..data_files.min(1) {
                        bump("跳过数据包", &mut done);
                    }
                }
            }
        }

        rep.send(Msg::Phase("交付技能".into()));
        let mut linked = Vec::new();
        for t in &targets {
            std::fs::create_dir_all(&t.skills_dir).map_err(|e| e.to_string())?;
            let mut n = 0usize;
            for s in &manifest.skills {
                let src = kit.join(&s.source);
                if src.is_dir() {
                    install_skill(&src, &t.skills_dir, &s.name, &kit, banner)?;
                    n += 1;
                }
                bump(&format!("{} → {}", s.name, t.id), &mut done);
            }
            rep.log(format!("已链接 {n} 个技能 -> {}", t.skills_dir.display()));
            linked.push((t.id.to_string(), t.skills_dir.clone(), n));
        }

        rep.send(Msg::Phase("写入安装记录".into()));
        let record = serde_json::json!({
            "version": manifest.version,
            "layoutRevision": manifest.layout_revision,
            "kit": kit.display().to_string(),
            "installedUtc": "",
            "artifacts": {
                "core": manifest.artifacts.core.sha256,
                "data": data_pack.as_ref().and_then(|d| d.sha256.clone()),
            },
            "harnesses": linked.iter().map(|(id, dir, n)| serde_json::json!({
                "id": id, "skillsDir": dir.display().to_string(), "skills": n
            })).collect::<Vec<_>>(),
        });
        std::fs::write(
            root.join(STATE_FILE),
            serde_json::to_string_pretty(&record).unwrap_or_default(),
        )
        .map_err(|e| format!("写安装记录失败：{e}"))?;
        bump("完成", &mut done);

        Ok(Summary {
            version: manifest.version.clone(),
            kit,
            linked,
            data_skipped: skip_data,
        })
    })();
    rep.send(Msg::Done(out));
}