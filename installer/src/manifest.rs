//! 发行清单结构，与 tools/release.py 写出的格式一一对应。

use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize)]
pub struct Artifact {
    pub file: Option<String>,
    pub sha256: Option<String>,
    #[serde(default)]
    pub uncompressed_bytes: u64,
    #[serde(default)]
    pub file_count: usize,
    #[serde(default)]
    pub required: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SkillEntry {
    pub name: String,
    pub source: String,
    #[serde(default)]
    pub group: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FileSets {
    #[serde(default)]
    pub core: HashMap<String, String>,
    #[serde(default)]
    pub data: HashMap<String, String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Artifacts {
    pub core: Artifact,
    #[serde(default)]
    pub data: Option<Artifact>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Manifest {
    pub version: String,
    #[serde(default)]
    pub layout_revision: u32,
    pub artifacts: Artifacts,
    pub files: FileSets,
    #[serde(default)]
    pub skills: Vec<SkillEntry>,
}
