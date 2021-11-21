use std::{error::Error, fs::File, io::{Cursor, Read}, path::Path};
use serde_gvas::{serde_gvas_header, types::GvasHeader};

fn main() -> Result<(), Box<dyn Error>> {
    let path = Path::new("SaveData_0.sav");
    let mut file = File::open(path)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;

    let mut cursor = Cursor::new(buf);

    let parsed_header: GvasHeader = serde_gvas_header::from_bytes(&mut cursor)?;
    println!("Header: {:?}", parsed_header);
    Ok(())
}