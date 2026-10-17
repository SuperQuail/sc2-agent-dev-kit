//! 最小 MPQ v1 打包器：把展开的地图目录打成单文件 `.SC2Map`。
//!
//! 为什么是 v1：实测真实地图两种都有——`EditorTest.SC2Map` 是 v1（32 字节头），
//! `Enslavers Redux…SC2Map` 是 v3（208 字节头），**引擎两种都认**，所以写简单的那个。
//!
//! 只支持**不压缩**存储。地图本身已经含 PNG/JPG/TGA 这类压缩过的资源，再套一层收益很小。
//! 校验靠外部工具（mpyq），不能自己读自己。
use anyhow::{bail, Result};
use std::path::{Path, PathBuf};

const SECTOR_SIZE_SHIFT: u16 = 3; // 512 字节扇区
const FLAG_EXISTS: u32 = 0x8000_0000;
const FLAG_SINGLE_UNIT: u32 = 0x0100_0000;

/// MPQ 用的加密表（0x500 项）。标准生成方式，种子固定。
fn crypt_table() -> [u32; 0x500] {
    let mut table = [0u32; 0x500];
    let mut seed: u32 = 0x0010_0001;
    for i in 0..0x100 {
        for j in (0..0x500).step_by(0x100) {
            seed = (seed.wrapping_mul(125).wrapping_add(3)) % 0x2AAAAB;
            let a = (seed & 0xFFFF) << 16;
            seed = (seed.wrapping_mul(125).wrapping_add(3)) % 0x2AAAAB;
            let b = seed & 0xFFFF;
            table[i + j] = a | b;
        }
    }
    table
}

/// 按 MPQ 规则算文件名散列。`kind` 取 0/1/2。
fn hash_string(name: &str, kind: usize, table: &[u32; 0x500]) -> u32 {
    let mut seed1: u32 = 0x7FED_7FED;
    let mut seed2: u32 = 0xEEEE_EEEE;
    // 规范化：统一反斜杠、统一大写（ASCII）。
    let normalized = name.replace('/', "\\").to_uppercase();
    for byte in normalized.bytes() {
        let ch = byte as u32;
        seed1 = table[(kind << 8) + ch as usize] ^ (seed1.wrapping_add(seed2));
        seed2 = ch
            .wrapping_add(seed1)
            .wrapping_add(seed2)
            .wrapping_add(seed2 << 5)
            .wrapping_add(3);
    }
    seed1
}

/// MPQ 的两张表**是加密存储的**——这是格式要求，不是可选项。
///
/// 加解密是同一套运算（对称），所以这里叫 encrypt 但算法与解密一致。
/// 密钥由表名经 TABLE 类型散列得到：`(hash table)` / `(block table)`。
fn encrypt_table(data: &[u8], key: u32, table: &[u32; 0x500]) -> Vec<u8> {
    let mut seed1 = key;
    let mut seed2: u32 = 0xEEEE_EEEE;
    let mut out = data.to_vec();
    for chunk in out.chunks_exact_mut(4) {
        seed2 = seed2.wrapping_add(table[0x400 + (seed1 & 0xFF) as usize]);
        // 明文 = 输入，密文 = 输出。**种子必须用明文更新**——
        // 解密时「明文」恰好是输出，加密时是输入，这正是两者不能共用一次运算的原因。
        let plain = u32::from_le_bytes(chunk.try_into().unwrap());
        let cipher = plain ^ seed1.wrapping_add(seed2);
        chunk.copy_from_slice(&cipher.to_le_bytes());
        seed1 = (!seed1).wrapping_shl(0x15).wrapping_add(0x1111_1111) | (seed1 >> 0x0B);
        seed2 = plain
            .wrapping_add(seed2)
            .wrapping_add(seed2 << 5)
            .wrapping_add(3);
    }
    out
}

/// 收集目录下所有文件，键用**反斜杠**相对路径（MPQ 的写法）。
pub fn collect(root: &Path) -> Result<Vec<(String, PathBuf)>> {
    let mut out: Vec<(String, PathBuf)> = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        for e in std::fs::read_dir(&dir)?.flatten() {
            let p = e.path();
            let ft = e.file_type()?;
            if ft.is_symlink() {
                continue;
            } else if ft.is_dir() {
                stack.push(p);
            } else {
                let rel = p.strip_prefix(root)?.to_string_lossy().replace('/', "\\");
                out.push((rel, p));
            }
        }
    }
    out.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(out)
}

/// 打成 MPQ v1。返回整个归档的字节。
pub fn build(files: &[(String, Vec<u8>)]) -> Result<Vec<u8>> {
    if files.is_empty() {
        bail!("没有要打包的文件");
    }
    let table = crypt_table();

    // 哈希表项数取 2 的幂，且不小于文件数的两倍——留出空位，避免线性探测绕圈。
    let mut hash_entries = 4usize;
    while hash_entries < files.len() * 2 {
        hash_entries *= 2;
    }
    let block_entries = files.len();

    // 文件数据区紧跟在 32 字节头之后。
    let mut data = Vec::new();
    let mut blocks: Vec<(u32, u32, u32, u32)> = Vec::new();
    let mut hash_table = vec![(0u32, 0u32, 0u16, 0u16, 0xFFFF_FFFFu32); hash_entries];

    for (i, (name, bytes)) in files.iter().enumerate() {
        let offset = 32 + data.len() as u32;
        data.extend_from_slice(bytes);
        blocks.push((offset, bytes.len() as u32, bytes.len() as u32, FLAG_EXISTS | FLAG_SINGLE_UNIT));

        // MPQ 的类型索引：0 = 落槽、1 = HASH_A、2 = HASH_B。
        let a = hash_string(name, 1, &table);
        let b = hash_string(name, 2, &table);
        let mut slot = (hash_string(name, 0, &table) as usize) % hash_entries;
        loop {
            if hash_table[slot].4 == 0xFFFF_FFFF {
                hash_table[slot] = (a, b, 0, 0, i as u32);
                break;
            }
            slot = (slot + 1) % hash_entries;
        }
    }

    let hash_off = 32 + data.len() as u32;

    let mut hash_bytes = Vec::with_capacity(hash_entries * 16);
    for (a, b, locale, platform, index) in &hash_table {
        hash_bytes.extend_from_slice(&a.to_le_bytes());
        hash_bytes.extend_from_slice(&b.to_le_bytes());
        hash_bytes.extend_from_slice(&locale.to_le_bytes());
        hash_bytes.extend_from_slice(&platform.to_le_bytes());
        hash_bytes.extend_from_slice(&index.to_le_bytes());
    }
    let hash_key = hash_string("(hash table)", 3, &table);
    data.extend_from_slice(&encrypt_table(&hash_bytes, hash_key, &table));

    let block_off = hash_off + (hash_entries * 16) as u32;
    let mut block_bytes = Vec::with_capacity(block_entries * 16);
    for (o, c, s, f) in &blocks {
        block_bytes.extend_from_slice(&o.to_le_bytes());
        block_bytes.extend_from_slice(&c.to_le_bytes());
        block_bytes.extend_from_slice(&s.to_le_bytes());
        block_bytes.extend_from_slice(&f.to_le_bytes());
    }
    let block_key = hash_string("(block table)", 3, &table);
    data.extend_from_slice(&encrypt_table(&block_bytes, block_key, &table));

    let archive_size = 32 + data.len() as u32;
    let mut out = Vec::with_capacity(archive_size as usize);
    out.extend_from_slice(b"MPQ\x1a");
    out.extend_from_slice(&32u32.to_le_bytes());
    out.extend_from_slice(&archive_size.to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes()); // format_version 0 = v1
    out.extend_from_slice(&SECTOR_SIZE_SHIFT.to_le_bytes());
    out.extend_from_slice(&hash_off.to_le_bytes());
    out.extend_from_slice(&block_off.to_le_bytes());
    out.extend_from_slice(&(hash_entries as u32).to_le_bytes());
    out.extend_from_slice(&(block_entries as u32).to_le_bytes());
    out.extend_from_slice(&data);
    Ok(out)
}

/// 打一个目录。会额外写入 `(listfile)`，否则多数读取器列不出内容。
pub fn pack_dir(root: &Path) -> Result<Vec<u8>> {
    let mut files: Vec<(String, Vec<u8>)> = Vec::new();
    for (rel, path) in collect(root)? {
        files.push((rel, std::fs::read(&path)?));
    }
    let listfile = files.iter().map(|(n, _)| n.as_str()).collect::<Vec<_>>().join("\r\n");
    files.push(("(listfile)".to_string(), listfile.into_bytes()));
    build(&files)
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    fn 生成的归档有正确的_v1_头() {
        let a = build(&[("a.txt".into(), b"hello".to_vec())]).unwrap();
        assert_eq!(&a[0..4], b"MPQ\x1a");
        assert_eq!(u32::from_le_bytes(a[4..8].try_into().unwrap()), 32);
        assert_eq!(u32::from_le_bytes(a[8..12].try_into().unwrap()) as usize, a.len());
        assert_eq!(u16::from_le_bytes(a[12..14].try_into().unwrap()), 0);
        assert_eq!(u16::from_le_bytes(a[14..16].try_into().unwrap()), SECTOR_SIZE_SHIFT);
        assert_eq!(u32::from_le_bytes(a[28..32].try_into().unwrap()), 1);
    }

    /// 钉死标准值：这些数是从 mpyq（独立实现）对同一算法算出来的，
    /// 用来防止「把自己写错的实现当成标准」。
    #[test]
    fn 散列值与_mpyq_一致() {
        let t = crypt_table();
        assert_eq!(hash_string("BankList.xml", 1, &t), 0xAE51_2863);
        assert_eq!(hash_string("BankList.xml", 2, &t), 0x65B7_9F97);
        assert_eq!(hash_string("(listfile)", 1, &t), 0xFD65_7910);
        assert_eq!(hash_string("(listfile)", 2, &t), 0x4E9B_98A7);
    }

    #[test]
    fn 同样的名字散列稳定且大小写与斜杠无关() {
        let t = crypt_table();
        assert_eq!(hash_string("Ab\\Cd", 0, &t), hash_string("ab/cd", 0, &t));
        assert_eq!(hash_string("Ab\\Cd", 1, &t), hash_string("ab/cd", 1, &t));
        assert_ne!(hash_string("x", 0, &t), hash_string("x", 1, &t));
    }

    #[test]
    fn 多个文件各有块表项() {
        let files: Vec<(String, Vec<u8>)> = (0..5)
            .map(|i| (format!("f{i}.txt"), vec![b'x'; 100 + i]))
            .collect();
        let a = build(&files).unwrap();
        let block_off = u32::from_le_bytes(a[20..24].try_into().unwrap()) as usize;
        let block_n = u32::from_le_bytes(a[28..32].try_into().unwrap()) as usize;
        assert_eq!(block_n, 5);
        // 块表是加密存的：读回来要先用同一个对称运算解开。
        let table = crypt_table();
        let key = hash_string("(block table)", 3, &table);
        let plain = encrypt_table(&a[block_off..block_off + 5 * 16], key, &table);
        let flags = u32::from_le_bytes(plain[12..16].try_into().unwrap());
        assert_eq!(flags & 0x8000_0000, 0x8000_0000);
        let off0 = u32::from_le_bytes(plain[0..4].try_into().unwrap());
        assert_eq!(off0, 32, "第一个文件紧跟在 32 字节头之后");
    }

    /// 加密的输出必须能被「解密版本」还原。
    /// 解密版本 = 同一个循环，但种子用**输出**更新（见 mpyq 的 _decrypt）。
    #[test]
    fn 加密后可被解密还原() {
        let table = crypt_table();
        let plain: Vec<u8> = (0u8..64).collect();
        let key = 0x1234_5678u32;
        let cipher = encrypt_table(&plain, key, &table);
        assert_ne!(cipher, plain, "必须真的变了");

        // 解密：种子用还原出的明文更新，明文 = 输出。
        let mut seed1 = key;
        let mut seed2: u32 = 0xEEEE_EEEE;
        let mut back = cipher.clone();
        for chunk in back.chunks_exact_mut(4) {
            seed2 = seed2.wrapping_add(table[0x400 + (seed1 & 0xFF) as usize]);
            let v = u32::from_le_bytes(chunk.try_into().unwrap());
            let p = v ^ seed1.wrapping_add(seed2);
            chunk.copy_from_slice(&p.to_le_bytes());
            seed1 = (!seed1).wrapping_shl(0x15).wrapping_add(0x1111_1111) | (seed1 >> 0x0B);
            seed2 = p.wrapping_add(seed2).wrapping_add(seed2 << 5).wrapping_add(3);
        }
        assert_eq!(back, plain);
    }

    #[test]
    fn 空输入报错() {
        assert!(build(&[]).is_err());
    }
}