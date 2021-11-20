use std::fmt::Debug;

use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct FEngineVersion {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
    pub change_list: u32,
    pub branch: String
}

#[derive(Deserialize, PartialEq, Eq)]
pub struct FGuid {
    pub a: u32,
    pub b: u32,
    pub c: u32,
    pub d: u32
}

impl FGuid {
    pub fn new(a: u32, b: u32, c: u32, d: u32) -> Self {
        FGuid { a, b, c, d }
    }
}

impl Debug for FGuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FGuid").field("value", &format!("{:x}{:x}{:x}{:x}", self.a, self.b, self.c, self.d)).finish()
    }
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct FCustomVersion {
    pub key: FGuid,
    pub version: i32
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct GvasHeader {
    pub file_type_tag: i32,
    pub save_game_file_version: i32,
    pub package_file_ue4_version: i32,
    pub engine_version: FEngineVersion,
    pub custom_version_format: i32,
    pub custom_versions: Vec<FCustomVersion>,
    pub save_game_class_name: String
}