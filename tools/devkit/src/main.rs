use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

use devkit::{catalog, edit, galaxy, map, triggers};

#[derive(Parser)]
#[command(name = "sc2cat", version, about = "StarCraft II 开发套件 CLI")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// 在一棵目录树里按 id 找 catalog 行（不假设文件名）
    Find {
        root: PathBuf,
        query: String,
        #[arg(long)]
        r#type: Option<String>,
        #[arg(long, default_value_t = 40)]
        limit: usize,
    },
    /// 列出树里出现过的 catalog 类型及计数
    Types {
        root: PathBuf,
        #[arg(long, default_value_t = 40)]
        limit: usize,
    },
    /// 看一个 .SC2Map / .SC2Mod 目录里有什么
    Layout { root: PathBuf },
    /// 列出地图里的单位生成点（读生成的 MapScript.galaxy）
    Spawns { map: PathBuf },
    /// 在 Triggers 里找某个单位的引用（只列，不改）
    FindUnit { map: PathBuf, unit: String },
    /// 精确替换 Triggers 里的单位引用（只改 Value 文本）
    SetUnit {
        map: PathBuf,
        #[arg(long)]
        from: String,
        #[arg(long)]
        to: String,
        /// 只显示将要改什么，不落盘
        #[arg(long)]
        dry_run: bool,
    },
}

fn rel(p: &std::path::Path, root: &std::path::Path) -> String {
    p.strip_prefix(root).unwrap_or(p).display().to_string()
}

fn main() -> Result<()> {
    match Cli::parse().cmd {
        Cmd::Find { root, query, r#type, limit } => {
            let q = query.to_lowercase();
            let all = catalog::scan(&root)?;
            let mut hits: Vec<_> = all
                .iter()
                .filter(|e| e.id.to_lowercase().contains(&q))
                .filter(|e| r#type.as_ref().is_none_or(|t| &e.kind == t))
                .collect();
            hits.sort_by(|a, b| (&a.kind, &a.id).cmp(&(&b.kind, &b.id)));
            println!("扫到 {} 条 catalog 行，命中 {}", all.len(), hits.len());
            for e in hits.iter().take(limit) {
                println!("  {:<24} {:<38} {}", e.kind, e.id, rel(&e.file, &root));
            }
            if hits.len() > limit {
                println!("  ... 还有 {} 条", hits.len() - limit);
            }
        }
        Cmd::Types { root, limit } => {
            let all = catalog::scan(&root)?;
            let mut counts: Vec<(String, usize)> = Vec::new();
            for e in &all {
                match counts.iter_mut().find(|(k, _)| k == &e.kind) {
                    Some((_, n)) => *n += 1,
                    None => counts.push((e.kind.clone(), 1)),
                }
            }
            counts.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
            println!("共 {} 条，{} 种类型", all.len(), counts.len());
            for (k, n) in counts.iter().take(limit) {
                println!("  {:<28} {}", k, n);
            }
        }
        Cmd::Layout { root } => {
            let l = map::layout(&root)?;
            let show = |t: &str, p: &Option<PathBuf>| match p {
                Some(p) => println!("  {t:<16} {}", rel(p, &root)),
                None => println!("  {t:<16} （没有）"),
            };
            println!("{}", root.display());
            show("Triggers", &l.triggers);
            show("MapScript", &l.mapscript);
            show("ComponentList", &l.component_list);
            println!("  {:<16} {}", "GameData", l.gamedata.len());
            for f in &l.gamedata {
                println!("      {}", f.file_name().unwrap_or_default().to_string_lossy());
            }
        }
        Cmd::Spawns { map: mp } => {
            let l = map::layout(&mp)?;
            let g = l.mapscript.context("这个目录里没有 MapScript.galaxy")?;
            let list = galaxy::spawns(&g)?;
            println!("{} 处 UnitCreate", list.len());
            for s in &list {
                println!("  :{:<6} {:<22} {:<26} {}", s.line, s.unit, s.player, s.count);
            }
        }
        Cmd::FindUnit { map: mp, unit } => {
            let l = map::layout(&mp)?;
            let t = l.triggers.context("这个目录里没有 Triggers")?;
            let xml = std::fs::read_to_string(&t)?;
            let all = triggers::unit_values(&xml)?;
            let hits: Vec<_> = all.iter().filter(|v| v.value == unit).collect();
            println!("共 {} 处 Unit 引用，其中 {} 有 {} 处", all.len(), unit, hits.len());
            for v in hits.iter().take(40) {
                println!("  :{:<6} 字节 {}..{}", v.line, v.start, v.end);
            }
        }
        Cmd::SetUnit { map: mp, from, to, dry_run } => {
            let l = map::layout(&mp)?;
            let t = l.triggers.context("这个目录里没有 Triggers")?;
            let xml = std::fs::read_to_string(&t)?;
            let hits: Vec<_> = triggers::unit_values(&xml)?
                .into_iter()
                .filter(|v| v.value == from)
                .collect();
            if hits.is_empty() {
                anyhow::bail!("Triggers 里没有 <Value>{from}</Value>，文件未改动");
            }
            let edits: Vec<(usize, usize, String)> =
                hits.iter().map(|v| (v.start, v.end, to.clone())).collect();
            let out = edit::splice(&xml, &edits)?;
            if dry_run {
                println!("将替换 {} 处 {from} -> {to}，未写入：", hits.len());
                for v in &hits {
                    println!("  :{:<6} 字节 {}..{}", v.line, v.start, v.end);
                }
            } else {
                std::fs::write(&t, out)?;
                println!("wrote {}: {} 处 {from} -> {to}", t.display(), hits.len());
            }
        }
    }
    Ok(())
}
