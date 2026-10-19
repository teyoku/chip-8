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

        let x = (opcode & 0x0F00 >> 8) as usize;
        let y = (opcode & 0x00F0 >> 4) as usize;
        let nn = (opcode & 0x00FF) as u8;
        let nnn = opcode & 0x0FFF;

        match opcode & 0xF000 {
            0x1000 => {
                // 1NNN - jump to NNN
                self.pc = nnn;
            }
            0x6000 => {
                // 6XNN - write NN to VX
                self.v[x] = nn;
            }
            0x7000 => {
                // 7XNN - add NN to VX
                self.v[x] = self.v[x].wrapping_add(nn);
            }
            _ => {
                panic!("Unknown opcode: {opcode:04X}");
            }
        }
    }
}
