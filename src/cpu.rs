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

        let c = ((opcode & 0xF000) >> 12) as u8; // Instruction category
        let x = (opcode & 0x0F00 >> 8) as usize; // Vx register index
        let y = (opcode & 0x00F0 >> 4) as usize; // Vy register index
        let n = (opcode & 0x000F) as u8; // 4-bit const

        let nn = (opcode & 0x00FF) as u8; // 8-bit const
        let nnn = opcode & 0x0FFF; // 12-bit memory address

        match (c, x, y, n) {
            // 1NNN - goto NNN
            (0x1, _, _, _) => {
                self.pc = nnn;
            }
            // 6XNN - write NN to VX
            (0x6, _, _, _) => {
                self.v[x] = nn;
            }
            // 7XNN - add NN to VX
            (0x7, _, _, _) => {
                self.v[x] = self.v[x].wrapping_add(nn);
            }
            // ANNN - load NNN into I
            (0xA, _, _, _) => {
                self.i = nnn;
            }
            _ => {
                panic!("Unknown opcode: {opcode:04X}");
            }
        }
    }
}
