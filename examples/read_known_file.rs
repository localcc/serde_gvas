use std::{error::Error, fs::File, io::{Cursor, Read}, path::Path};
use serde::Deserialize;
use serde_gvas::{header, types::{GvasHeader}};

#[derive(Deserialize, Debug)]
struct TestStruct {
    test: i32
}

#[derive(Deserialize, Debug)]
struct UnrealFile {
    u8_test: u8,
    i8_test: i8,
    ushort_test: u16,
    short_test: i16,
    umedium_test: u32,
    medium_test: i32,
    ulong_test: u64,
    long_test: i64,
    f_property: f32,
    d_property: f64,
    str_property: String,
    test_struct: TestStruct
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = Path::new("SaveData_0.sav");
    let mut file = File::open(path)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;

    let mut cursor = Cursor::new(buf);

    let mut header_deserializer = header::de::Deserializer::from_bytes(&mut cursor);
    let header: GvasHeader = GvasHeader::deserialize(&mut header_deserializer)?;
    println!("Header: {:?}", header);

    let file: UnrealFile = serde_gvas::from_bytes(&mut cursor)?;
    println!("File: {:?}", file);

    Ok(())
}