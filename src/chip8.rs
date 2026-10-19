use crate::{cpu::Cpu, memory::Memory};

pub const PROGRAM_START_OFFSET: usize = 0x200;

pub struct Chip8 {
    memory: Memory,
    cpu: Cpu
}

impl Chip8 {
    pub fn new() -> Self {
        Self {
            memory: Memory::new(),
            cpu: Cpu::new(),
        }
    }

    pub fn load_rom(&mut self, data: &[u8]) {
        for (i, &value) in data.iter().enumerate() {
            let address = (PROGRAM_START_OFFSET + i) as u16;
            self.memory.write_byte(address, value);
        }
    }

    fn fetch_opcode(&self) -> u16 {
        let pc = self.cpu.pc;
        let high = self.memory.read_byte(pc);
        let low = self.memory.read_byte(pc + 1);

        u16::from_be_bytes([high, low])
    }
}
