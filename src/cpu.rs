use crate::{chip8::PROGRAM_START_OFFSET, memory::Memory};

const NUM_REGISTERS: usize = 16;

pub struct Cpu {
    pub v: [u8; NUM_REGISTERS], // V0-VF registers
    pub i: u16,                 // Index register
    pub pc: u16,                // Program Counter
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            v: [0; NUM_REGISTERS],
            i: 0,
            pc: PROGRAM_START_OFFSET as u16,
        }
    }

    pub fn execute(&mut self, opcode: u16, memory: &mut Memory) {
        self.pc += 2;

        // ...
    }
}
