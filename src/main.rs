pub mod chip8;
pub mod cpu;
pub mod memory;
pub mod window;

use std::{env, error::Error, fs};

use crate::chip8::Chip8;

fn main() -> Result<(), Box<dyn Error>> {
    let rom_path = env::args_os()
        .nth(1)
        .unwrap_or_else(|| "data/IBM Logo".into());
    let data = fs::read(rom_path)?;

    let mut chip8 = Chip8::new()?;
    chip8.load_rom(&data)?;

    while chip8.is_open() {
        chip8.run()?;
    }

    Ok(())
}
