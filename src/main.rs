pub mod chip8;
pub mod cpu;
pub mod window;
pub mod memory;

use std::{error::Error, fs};

use crate::chip8::Chip8;

fn main() -> Result<(), Box<dyn Error>> {
    let data = fs::read("data/BRIX")?;

    let mut chip8 = Chip8::new();
    chip8.load_rom(&data)?;

    Ok(())
}
