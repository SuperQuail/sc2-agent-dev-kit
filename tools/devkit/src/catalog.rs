//! 扫描一棵目录树里的 GameData catalog 行。
//!
//! 刻意**按内容判断**而不是按文件名：实测第三方 mod 会把 CUnit 放进
//! `commanders/commanderzeratul_zealot.xml` 这类随意命名的文件里，
//! 根元素仍然是 `<Catalog>`。所以要遍历所有 *.xml 看根元素。
use anyhow::{Context, Result};
use quick_xml::events::Event;
use quick_xml::Reader;
use std::path::{Path, PathBuf};

/// 一条 catalog 行：类型 + id + 它在哪个文件里。
#[derive(Debug, Clone)]
pub struct Entry {
    pub kind: String,
    pub id: String,
    pub file: PathBuf,
}

/// 递归找所有 .xml（不跟符号链接）。
pub fn xml_files(root: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let Ok(rd) = std::fs::read_dir(&dir) else { continue };
        for e in rd.flatten() {
            let p = e.path();
            let Ok(ft) = e.file_type() else { continue };
            if ft.is_symlink() {
                continue;
            } else if ft.is_dir() {
                stack.push(p);
            } else if p.extension().is_some_and(|x| x.eq_ignore_ascii_case("xml")) {
                out.push(p);
            }
        }
    }
    out.sort();
    out
}

/// 从一个 GameData XML 里取出顶层 catalog 行。非 `<Catalog>` 根的文件返回空。
pub fn entries_in(file: &Path) -> Result<Vec<Entry>> {
    let bytes = std::fs::read(file).with_context(|| format!("读不了 {}", file.display()))?;
    let mut reader = Reader::from_reader(bytes.as_slice());
    reader.config_mut().trim_text(true);

    let mut out = Vec::new();
    let mut depth = 0usize;
    let mut buf = Vec::new();

    // depth 是「已经进来的 Start 层数」：0=根，1=顶层 catalog 行。
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                let at = depth;
                depth += 1;
                if at == 0 {
                    // 根元素不是 Catalog 就不是 GameData（Triggers 的根是 TriggerData）。
                    if e.name().as_ref() != "Catalog" {
                        return Ok(out);
                    }
                } else if at == 1 {
                    if let Some(ent) = row(&e, file) {
                        out.push(ent);
                    }
                }
            }
            // 自闭合的行（<CUnit id="X"/>）也同样是 catalog 行，必须收。
            Ok(Event::Empty(e)) => {
                if depth == 1 {
                    if let Some(ent) = row(&e, file) {
                        out.push(ent);
                    }
                } else if depth == 0 {
                    // <Catalog/> 或非 Catalog 的根，都没有内容
                    return Ok(out);
                }
            }
            Ok(Event::End(_)) => {
                if depth == 0 {
                    break;
                }
                depth -= 1;
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(e).with_context(|| format!("解析 {} 失败", file.display())),
            _ => {},
        }
        buf.clear();
    }
    Ok(out)
}

/// 一个元素能不能算 catalog 行：看它有没有 id。
fn row(e: &quick_xml::events::BytesStart<'_>, file: &Path) -> Option<Entry> {
    let mut id = None;
    for a in e.attributes().flatten() {
        if a.key.as_ref() == "id" {
            id = Some(a.value.to_string());
        }
    }
    id.map(|id| Entry { kind: e.name().as_ref().to_string(), id, file: file.to_path_buf() })
}
/// 扫整棵树，返回全部 catalog 行。
pub fn scan(root: &Path) -> Result<Vec<Entry>> {
    let mut all = Vec::new();
    for f in xml_files(root) {
        all.extend(entries_in(&f)?);
    }
    Ok(all)
}
#[cfg(test)]
mod tests {
    use super::*;

    fn write(dir: &Path, name: &str, body: &str) -> PathBuf {
        let p = dir.join(name);
        std::fs::create_dir_all(p.parent().unwrap()).unwrap();
        std::fs::write(&p, body).unwrap();
        p
    }

    /// 临时目录：文件名带进程 id，测试之间不打架。
    fn tmp(tag: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("sc2cat-test-{}-{}", std::process::id(), tag));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).unwrap();
        d
    }

    #[test]
    fn 顶层行带类型与_id() {
        let d = tmp("basic");
        write(&d, "UnitData.xml", r#"<Catalog>
    <CUnit id="Marine"><LifeMax value="45"/></CUnit>
    <CUnit id="Zealot"/>
</Catalog>"#);
        let es = scan(&d).unwrap();
        assert_eq!(es.len(), 2);
        assert_eq!((es[0].kind.as_str(), es[0].id.as_str()), ("CUnit", "Marine"));
        assert_eq!(es[1].id, "Zealot");
        // 嵌套的子元素不能被当成顶层行
        assert!(es.iter().all(|e| e.kind == "CUnit"));
    }

    #[test]
    fn 非_catalog_根返回空() {
        let d = tmp("trigger");
        write(&d, "Triggers", r#"<TriggerData><Item id="X"/></TriggerData>"#);
        write(&d, "Triggers.xml", r#"<TriggerData><Item Type="Trigger" id="Y"/></TriggerData>"#);
        assert!(scan(&d).unwrap().is_empty());
    }

    #[test]
    fn 空_catalog_与注释不产生假命中() {
        let d = tmp("empty");
        write(&d, "A.xml", "<Catalog/>");
        write(&d, "B.xml", "<Catalog>\n<!-- <CUnit id=\"Ghost\"/> -->\n</Catalog>");
        assert!(scan(&d).unwrap().is_empty());
    }

    #[test]
    fn 按内容而非文件名_随意命名的文件也能扫到() {
        let d = tmp("arbitrary");
        // 这是实测第三方 mod 的形态：CUnit 不在 UnitData.xml 里
        write(&d, "commanders/commanderzeratul_zealot.xml",
            r#"<Catalog><CUnit id="ZeratulZealot"><LifeMax value="100"/></CUnit></Catalog>"#);
        let es = scan(&d).unwrap();
        assert_eq!(es.len(), 1);
        assert_eq!(es[0].id, "ZeratulZealot");
        assert!(es[0].file.ends_with("commanderzeratul_zealot.xml"));
    }

    #[test]
    fn 没有_id_的顶层元素被跳过() {
        let d = tmp("noid");
        write(&d, "A.xml", r#"<Catalog><const id="K" path="CUnit.X"/><CUnit/><CEditorComment id="C"/></Catalog>"#);
        let es = scan(&d).unwrap();
        assert_eq!(es.len(), 2);
        assert_eq!(es[0].kind, "const");
        assert_eq!(es[1].kind, "CEditorComment");
    }
}