use std::{error::Error, fs::File, io::{Cursor, Read}, path::Path};
use serde_gvas::{header, types::GvasHeader};

fn main() -> Result<(), Box<dyn Error>> {
    let path = Path::new("SaveData_0.sav");
    let mut file = File::open(path)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;

    let mut cursor = Cursor::new(buf);

    let parsed_header: GvasHeader = header::de::from_bytes(&mut cursor)?;
    println!("Header: {:?}", parsed_header);
    Ok(())
}