use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

use devkit::{catalog, doctor, edit, galaxy, locale, map, mpq, triggers};

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
    /// 换掉地图里**预置单位**的类型（改 Objects 的 UnitType 属性，不是触发器）
    SetPlacedUnit {
        map: PathBuf,
        #[arg(long)]
        from: String,
        #[arg(long)]
        to: String,
    },
    /// 交付物自检：这一条命令回答「这个包是不是半成品」
    Doctor { pkg: PathBuf },
    /// 把 Maps/ + Mods/ 目录装成启动器能导入的 zip（补 metadata.json）
    Package {
        /// 含 Maps/ 与 Mods/ 的目录
        pkg: PathBuf,
        /// 输出 zip；不给就同目录同名
        #[arg(long)]
        out: Option<PathBuf>,
        /// 包名（写进 metadata.json）
        #[arg(long)]
        name: String,
        /// 归属资料片，如 wol / hots / lotv
        #[arg(long, default_value = "wol")]
        campaign: String,
        /// 依赖的模组，按 Mods/ 后第一段算；可给多个
        #[arg(long = "mod")]
        mods: Vec<String>,
    },
    /// 看一个 .SC2Map / .SC2Mod 目录里有什么
    Layout { root: PathBuf },
    /// 列出地图里的单位生成点（读生成的 MapScript.galaxy）
    Spawns { map: PathBuf },
    /// 在 Triggers 里找某个单位的引用（只列，不改）
    FindUnit { map: PathBuf, unit: String },
    /// 把展开的地图目录打成单文件 MPQ（交付形态）
    Pack {
        /// 要打包的目录（如 x.SC2Map）
        dir: PathBuf,
        /// 输出文件；不给就同目录同名加 .SC2Map
        #[arg(long)]
        out: Option<PathBuf>,
    },
    /// 把 pack 出来的单文件 MPQ 解回目录 —— 交付形态也要能被验
    Unpack {
        /// 单文件 .SC2Map / .SC2Mod
        file: PathBuf,
        /// 解到哪
        #[arg(long)]
        out: PathBuf,
    },
    /// 往某个触发器函数的动作段里插 Galaxy 代码（改了源不改产物等于没改）
    Inject {
        map: PathBuf,
        /// 函数名，如 gt_Init01Technology_Func（插在动作段的 runActions 守卫之后）
        #[arg(long)]
        r#fn: Option<String>,
        /// 行锚点：插在第一处包含这段文字的行之后（缩进跟随锚点）。与 --fn 二选一
        #[arg(long)]
        after: Option<String>,
        /// 要插入的代码行，可给多个，按顺序插
        #[arg(long = "code", required = true)]
        code: Vec<String>,
    },
    /// 往地图/模组的 DocumentHeader 里追加一条依赖
    AddDep {
        root: PathBuf,
        /// 依赖串，如 file:Mods\kit_lotv_zeratul.SC2Mod
        dep: String,
    },
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
        /// 只改属于这个触发器的引用（按 TriggerStrings 的显示名或 Element Id）
        #[arg(long)]
        in_trigger: Option<String>,
        /// galaxy 函数名（如 gt_IntroCinematic_Func）。**必须与 --in-trigger 一起给**。
        ///
        /// MapScript.galaxy 才是游戏运行时执行的；不给它，galaxy 一个字都不改。
        /// 地图有 TriggerStrings 时能从 --in-trigger 推出来，没有才需要手工给。
        #[arg(long, requires = "in_trigger")]
        galaxy_fn: Option<String>,
    },
    /// 把 CASC 抽出的源形态整成分发形态：语系目录驼峰化、补齐 12 个语系、重写 ComponentList
    Normalize { root: PathBuf },
    /// 按前缀裁掉本地化键（所有语系）。第三方 mod 常把基础战役的文本整包抄进来，会压住战役自己的中文
    PruneLocale {
        root: PathBuf,
        /// 要裁的前缀，可给多个，如 Conversation/ 、ConversationState/ 、Param/
        #[arg(long = "drop-prefix", required = true)]
        drop_prefix: Vec<String>,
    },
    /// 按参考表「影子裁」：只删参考表里也有的键（依赖 mod 抄了基础战役文本时用）
    PruneShadowing {
        root: PathBuf,
        /// 参考文件，如战役自己的 GameStrings.txt
        #[arg(long)]
        reference: PathBuf,
    },
    /// 就地设置本地化字符串（命中已有键就替换；替换不了才追加）
    Localize {
        root: PathBuf,
        #[arg(long, default_value = "zhCN")]
        locale: String,
        /// 形如 Unit/Name/Zealot=狂热者，可给多个
        #[arg(long = "set", required = true)]
        set: Vec<String>,
    },
}

/// 从地图里找 TriggerStrings（enUS 优先），解析出触发器显示名。
fn load_trigger_names(map: &std::path::Path) -> std::collections::HashMap<String, String> {
    for loc in ["enUS", "zhCN", "zhTW"] {
        // 目录名是 `enUS.SC2Data`（一个目录），不是 `enUS/SC2Data`。
        let p = map
            .join(format!("{loc}.SC2Data"))
            .join("LocalizedData")
            .join("TriggerStrings.txt");
        if let Ok(s) = std::fs::read_to_string(&p) {
            let m = triggers::trigger_names(&s);
            if !m.is_empty() {
                return m;
            }
        }
    }
    Default::default()
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
                // 两种写法都认：Unit 与 CUnit —— 写错了会静默 0 命中，所以必须宽松
                .filter(|e| {
                    r#type.as_ref().is_none_or(|t| {
                        e.kind.trim_start_matches('C') == t.trim_start_matches('C')
                    })
                })
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
        Cmd::SetPlacedUnit { map: mp, from, to } => {
            let (n, p) = map::replace_placed_unit(&mp, &from, &to)?;
            println!("wrote {}: {} 处 UnitType=\"{from}\" -> \"{to}\"", p.display(), n);
        }
        Cmd::Doctor { pkg } => {
            // 交付物是 zip，自检也得能吃 zip —— 解到临时目录再查。
            // bsdtar 会把 `D:` 当远程主机，所以 cd 到父目录只传裸文件名。
            let pkg = if pkg.is_file() {
                let dir = std::env::temp_dir().join("sc2cat-doctor");
                let _ = std::fs::remove_dir_all(&dir);
                std::fs::create_dir_all(&dir)?;
                let zip_dir = pkg.parent().unwrap_or_else(|| std::path::Path::new("."));
                let zip_name = pkg.file_name().map(|n| n.to_os_string()).context("输入名不合法")?;
                let st = std::process::Command::new("tar")
                    .current_dir(zip_dir)
                    .arg("-x").arg("-f").arg(&zip_name).arg("-C").arg(&dir)
                    .status()?;
                if !st.success() {
                    anyhow::bail!("解不开 {}（不是 tar/zip？）", pkg.display());
                }
                dir
            } else {
                pkg
            };
            let findings = doctor::check(&pkg)?;
            let mut bad = 0;
            for f in &findings {
                let (mark, line) = match f.level {
                    doctor::Level::Ok => ("  ok  ", format!("{}: {}", f.item, f.detail)),
                    doctor::Level::Warn => (" warn ", format!("{}: {}", f.item, f.detail)),
                    doctor::Level::Bad => {
                        bad += 1;
                        (" FAIL ", format!("{}: {}", f.item, f.detail))
                    }
                };
                println!("{mark}{line}");
            }
            if bad > 0 {
                anyhow::bail!("{bad} 项没通过 —— 这个包还不能交付");
            }
            println!("全部通过");
        }
        Cmd::Package { pkg, out, name, campaign, mods } => {
            // metadata.json 的字段类型是实测出来的：mods 必须是**数组**，
            // 写成字符串会让整个文件解析失败、campaign 也跟着丢。
            let mods_json = mods
                .iter()
                .map(|m| format!("\"{m}\""))
                .collect::<Vec<_>>()
                .join(", ");
            let id = name.replace(' ', "-").to_lowercase();
            let meta = format!(
                "{{\n  \"name\": \"{name}\",\n  \"author\": \"devkit\",\n  \"version\": \"1.0\",\n  \"campaign\": \"{campaign}\",\n  \"miyin\": {{\n    \"format\": 2,\n    \"id\": \"devkit.{id}\",\n    \"mods\": [{mods_json}]\n  }}\n}}\n"
            );
            std::fs::write(pkg.join("metadata.json"), meta)?;
            println!("wrote {}/metadata.json", pkg.display());

            let zip = out.unwrap_or_else(|| pkg.with_extension("zip"));
            // 用平台自带的 tar 归档：Windows 10+ 的 bsdtar 认扩展名，别自己写 zip 格式
            // 显式列出顶层项：用 `.` 会让每个条目带上 `./` 前缀，启动器认不出根下的 Maps/。
            // bsdtar 会把 `D:` 当远程主机（`Cannot connect to D`），所以 cd 到父目录只传裸文件名。
            let zip_dir = zip.parent().unwrap_or_else(|| std::path::Path::new("."));
            let zip_name = zip.file_name().map(|n| n.to_os_string()).context("输出名不合法")?;
            // -C 的路径必须绝对：current_dir 已经换了，相对路径会指错地方。
            let pkg_abs = std::path::absolute(&pkg)?;
            let mut cmd = std::process::Command::new("tar");
            cmd.current_dir(zip_dir);
            cmd.arg("-a").arg("-c").arg("-f").arg(&zip_name).arg("-C").arg(&pkg_abs);
            for top in ["Maps", "Mods", "metadata.json"] {
                if pkg.join(top).exists() {
                    cmd.arg(top);
                }
            }
            let st = cmd.status()?;
            if !st.success() {
                anyhow::bail!("tar 归档失败（退出码 {:?}）", st.code());
            }
            println!(
                "wrote {}: {} 字节",
                zip.display(),
                std::fs::metadata(&zip).map(|m| m.len()).unwrap_or(0)
            );
            println!("接着跑一遍 sc2cat doctor {} 确认", pkg.display());
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
            let names = load_trigger_names(&mp);
            let all = triggers::unit_values_named(&xml, &names)?;
            let hits: Vec<_> = all.iter().filter(|v| v.value == unit).collect();
            println!("共 {} 处 Unit 引用，其中 {} 有 {} 处", all.len(), unit, hits.len());
            for v in hits.iter().take(60) {
                println!(
                    "  :{:<6} {:<28} {}",
                    v.line,
                    v.trigger_name.as_deref().unwrap_or(v.trigger_id.as_deref().unwrap_or("?")),
                    format!("{}..{}", v.start, v.end)
                );
            }
            // 按触发器汇总，便于判断该改哪一组
            let mut by: Vec<(String, usize)> = Vec::new();
            for v in &hits {
                let k = v
                    .trigger_name
                    .clone()
                    .or_else(|| v.trigger_id.clone())
                    .unwrap_or_else(|| "(无归属)".into());
                match by.iter_mut().find(|(n, _)| n == &k) {
                    Some((_, c)) => *c += 1,
                    None => by.push((k, 1)),
                }
            }
            println!("  按触发器汇总：");
            for (n, c) in &by {
                println!("    {:<32} {}", n, c);
            }
        }
        Cmd::Pack { dir, out } => {
            let bytes = mpq::pack_dir(&dir)?;
            let out = out.unwrap_or_else(|| {
                let s = dir.to_string_lossy().to_string();
                PathBuf::from(format!("{}.SC2Map", s.trim_end_matches(['/', '\\'])))
            });
            std::fs::write(&out, &bytes)?;
            println!("wrote {}: {} 字节", out.display(), bytes.len());
        }
        Cmd::Inject { map: mp, r#fn, after, code } => {
            let gal = ["MapScript.galaxy", "mapscript.galaxy"]
                .iter()
                .map(|n| mp.join(n))
                .find(|p| p.is_file())
                .context("这个目录里没有 MapScript.galaxy")?;
            let src = std::fs::read_to_string(&gal)?;
            let refs: Vec<&str> = code.iter().map(String::as_str).collect();
            let (out, ok, where_) = match (&r#fn, &after) {
                (Some(f), None) => {
                    let (o, k) = galaxy::inject_into_fn(&src, f, &refs);
                    (o, k, format!("{f} 的动作段之后"))
                }
                (None, Some(a)) => {
                    let (o, k) = galaxy::inject_after_line(&src, a, &refs);
                    (o, k, format!("含「{a}」的行之后"))
                }
                _ => anyhow::bail!("--fn 与 --after 必须二选一"),
            };
            if !ok {
                anyhow::bail!("{} 里找不到 {}，文件未改动", gal.display(), where_);
            }
            std::fs::write(&gal, out)?;
            println!("wrote {}: {} 行插在 {}", gal.display(), code.len(), where_);
        }
        Cmd::Unpack { file, out } => {
            let entries = mpq::read_entries(&file)?;
            for (name, bytes) in &entries {
                let p = out.join(name.replace('\\', "/"));
                if let Some(parent) = p.parent() {
                    std::fs::create_dir_all(parent)?;
                }
                std::fs::write(&p, bytes)?;
            }
            println!("解开 {} 个文件到 {}", entries.len(), out.display());
        }
        Cmd::AddDep { root, dep } => {
            let p = root.join("DocumentHeader");
            let before = std::fs::read(&p).with_context(|| format!("读不了 {}", p.display()))?;
            let after = map::add_dependency(&before, &dep)?;
            println!("依赖表 {} -> {} 字节", before.len(), after.len());
            std::fs::write(&p, after)?;
            println!("wrote {}: +{}", p.display(), dep);
        }
        Cmd::SetUnit { map: mp, from, to, dry_run, in_trigger, galaxy_fn } => {
            let l = map::layout(&mp)?;
            let t = l.triggers.context("这个目录里没有 Triggers")?;
            let xml = std::fs::read_to_string(&t)?;
            let names = load_trigger_names(&mp);
            let hits: Vec<_> = triggers::unit_values_named(&xml, &names)?
                .into_iter()
                .filter(|v| v.value == from)
                .filter(|v| match &in_trigger {
                    None => true,
                    Some(w) => {
                        v.trigger_name.as_deref() == Some(w.as_str())
                            || v.trigger_id.as_deref() == Some(w.as_str())
                    }
                })
                .collect();
            if hits.is_empty() {
                match &in_trigger {
                    Some(w) => anyhow::bail!("触发器「{w}」里没有 <Value>{from}</Value>，文件未改动"),
                    None => anyhow::bail!("Triggers 里没有 <Value>{from}</Value>，文件未改动"),
                }
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

                // 运行时执行的是 MapScript.galaxy，光改 Triggers 不生效。
                if let Some(gal) = ["MapScript.galaxy", "mapscript.galaxy"]
                    .iter()
                    .map(|n| mp.join(n))
                    .find(|p| p.is_file())
                {
                    let src = std::fs::read_to_string(&gal)?;
                    // 触发器显示名去空格就是函数名： Intro Cinematic -> gt_IntroCinematic_Func
                    let fname = galaxy_fn.clone().or_else(|| {
                        in_trigger
                            .as_ref()
                            .map(|w| names.get(w).cloned().unwrap_or_else(|| w.clone()))
                            .map(|n| format!("gt_{}_Func", n.replace(' ', "")))
                    });
                    if fname.is_none() {
                        anyhow::bail!(
                            "给了 --in-trigger 但推不出 galaxy 函数名（这张图没有 TriggerStrings）——\n\
                             请补 --galaxy-fn gt_Xxx_Func，否则改了源不改产物，进游戏不会有变化"
                        );
                    }
                    let (after, n) = galaxy::replace_created_unit(&src, &from, &to, fname.as_deref());
                    if n == 0 {
                        println!("  {} 里没有可替换的创建调用 —— 进游戏不会有变化", gal.file_name().unwrap().to_string_lossy());
                    } else {
                        std::fs::write(&gal, after)?;
                        println!("wrote {}: {} 处创建调用 {from} -> {to}", gal.display(), n);
                    }
                }
            }
        }
        Cmd::Normalize { root } => println!("{}", locale::normalize(&root)?),
        Cmd::PruneLocale { root, drop_prefix } => println!("{}", locale::prune(&root, &drop_prefix)?),
        Cmd::PruneShadowing { root, reference } => {
            println!("{}", locale::prune_shadowing(&root, &reference)?)
        }
        Cmd::Localize { root, locale: loc, set } => {
            let mut pairs = Vec::new();
            for s in &set {
                let (k, v) = s
                    .split_once('=')
                    .with_context(|| format!("--set 要写成 key=value，收到 {s}"))?;
                pairs.push((k.to_string(), v.to_string()));
            }
            println!("{}", locale::set_keys(&root, &loc, &pairs)?);
        }
    }
    Ok(())
}