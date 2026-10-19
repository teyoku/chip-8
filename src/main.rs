pub mod memory;
use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let contents = fs::read("data/BRIX")?;
    println!("Bytes: {contents:?}");

    Ok(())
}
