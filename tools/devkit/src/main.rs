use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod catalog;

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
        /// 要扫的根目录（.SC2Mod / .SC2Map / GameData 都行）
        root: PathBuf,
        /// id 的子串，大小写不敏感
        query: String,
        /// 只看某个 catalog 类型
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
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
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
                println!("  {:<24} {:<38} {}", e.kind, e.id, short(&e.file, &root));
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
    }
    Ok(())
}

fn short(p: &std::path::Path, root: &std::path::Path) -> String {
    p.strip_prefix(root).unwrap_or(p).display().to_string()
}
