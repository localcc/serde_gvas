use std::{error::Error, fmt::Display, fs::File, io::Read, path::Path};

use serde::Deserialize;

extern crate serde_gvas;

#[derive(Deserialize, Debug)]
struct FEngineVersion {
    major: u16,
    minor: u16,
    patch: u16,
    change_list: u32,
    branch: String
}

impl Display for FEngineVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}, Change list: {}, Branch: {}", self.major, self.minor, self.patch, self.change_list, self.branch)
    }
}

#[derive(Deserialize, Debug)]
struct FGuid {
    a: u32,
    b: u32,
    c: u32,
    d: u32
}

impl Display for FGuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:x}{:x}{:x}{:x}", self.a, self.b, self.c, self.d)
    }
}

#[derive(Deserialize, Debug)]
struct FCustomVersion {
    key: FGuid,
    version: i32
}

impl Display for FCustomVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.key, self.version)
    }
}

#[derive(Deserialize, Debug)]
struct UnrealHeader {
    file_type_tag: i32,
    save_game_file_version: i32,
    package_file_ue4_version: i32,
    engine_version: FEngineVersion,
    custom_version_format: i32,
    custom_versions: Vec<FCustomVersion>,
    save_game_class_name: String
}

fn print_vec<T>(v: &Vec<T>, f: &mut std::fmt::Formatter<'_>) where T: std::fmt::Display {
    v.iter().for_each(|e| {
        write!(f, "{}, ", e);
    });
}

impl Display for UnrealHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "File version: {}, UE4 Package version: {}, Engine version: {}, Custom version format: {}, Custom versions: \n", 
            self.save_game_file_version, 
            self.package_file_ue4_version, 
            self.engine_version, 
            self.custom_version_format)?;
        print_vec(&self.custom_versions, f);
        write!(f, "\nSave class name: {}", self.save_game_class_name)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = Path::new("fullsave.sav");
    let mut file = File::open(path)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;

    let parsed_header: UnrealHeader = serde_gvas::from_bytes(&buf)?;
    println!("Header: {}", parsed_header);
    Ok(())
}