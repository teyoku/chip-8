use crate::{cpu::Cpu, memory::{MEMORY_SIZE, Memory}};

pub const PROGRAM_START: usize = 0x200;

pub struct Chip8 {
    memory: Memory,
    cpu: Cpu,
}

impl Chip8 {
    pub fn new() -> Self {
        Self {
            memory: Memory::new(),
            cpu: Cpu::new(),
        }
    }

    pub fn load_rom(&mut self, data: &[u8]) -> Result<(), &str> {
        if PROGRAM_START + data.len() >= MEMORY_SIZE {
            return Err("Out of memory: program too large");
        }

        for (i, &value) in data.iter().enumerate() {
            let address = (PROGRAM_START + i) as u16;
            self.memory.write_byte(address, value);
        }

        Ok(())
    }

    pub fn run(&mut self) {
        let opcode = self.fetch_opcode();

        self.cpu.execute(opcode, &mut self.memory);
    }

    fn fetch_opcode(&self) -> u16 {
        let pc = self.cpu.pc;
        let high = self.memory.read_byte(pc);
        let low = self.memory.read_byte(pc + 1);

        u16::from_be_bytes([high, low])
    }
}
