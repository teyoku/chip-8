use crate::memory::Memory;

const LOAD_ROM_OFFSET: usize = 0x200;

pub struct Chip8 {
    memory: Memory,
}

impl Chip8 {
    pub fn new() -> Self {
        Self {
            memory: Memory::new(),
        }
    }

    pub fn load_rom(&mut self, data: &[u8]) {
        for (i, &value) in data.iter().enumerate() {
            let address = (LOAD_ROM_OFFSET + i) as u16;
            self.memory.write_byte(address, value);
        }
    }
}
