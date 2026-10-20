use crate::{chip8::PROGRAM_START_OFFSET, memory::Memory};

const NUM_REGISTERS: usize = 16;
const STACK_SIZE: usize = 16;
const OPCODE_SIZE: u16 = 2;

enum ProgramCounter {
    Next,
    Skip,
    Jump(u16),
}

impl ProgramCounter {
    fn skip_if(condition: bool) -> ProgramCounter {
        if condition {
            ProgramCounter::Skip
        } else {
            ProgramCounter::Next
        }
    }
}

pub struct Cpu {
    pub v: [u8; NUM_REGISTERS], // V0-VF registers
    pub i: u16,                 // Index register
    pub pc: u16,                // Program Counter
    pub stack: [u16; STACK_SIZE],
    pub sp: u16,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            v: [0; NUM_REGISTERS],
            i: 0,
            pc: PROGRAM_START_OFFSET as u16,
            stack: [0; STACK_SIZE],
            sp: 0,
        }
    }

    pub fn execute(&mut self, opcode: u16, memory: &mut Memory) {
        let c = ((opcode & 0xF000) >> 12) as u8; // Instruction category
        let x = (opcode & 0x0F00 >> 8) as usize; // Vx register index
        let y = (opcode & 0x00F0 >> 4) as usize; // Vy register index
        let n = (opcode & 0x000F) as u8; // 4-bit const

        let nn = (opcode & 0x00FF) as u8; // 8-bit const
        let nnn = opcode & 0x0FFF; // 12-bit memory address

        let pc_change = match (c, x, y, n) {
            // 1NNN - goto NNN
            (0x1, _, _, _) => {
                self.pc = nnn;
                ProgramCounter::Next
            }
            // 2NNN - call NNN
            (0x2, _, _, _) => {
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                ProgramCounter::Jump(nnn)
            }
            // 3XNN - skip if Vx == NN
            (0x3, _, _, _) => ProgramCounter::skip_if(self.v[x] == nn),
            // 4XNN - skip if Vx != NN
            (0x4, _, _, _) => ProgramCounter::skip_if(self.v[x] != nn),
            // 5XY0 - skip if Vx == Vy
            (0x5, _, _, 0x0) => ProgramCounter::skip_if(self.v[x] == self.v[y]),
            // 6XNN - write NN to VX
            (0x6, _, _, _) => {
                self.v[x] = nn;
                ProgramCounter::Next
            }
            // 7XNN - add NN to VX
            (0x7, _, _, _) => {
                self.v[x] = self.v[x].wrapping_add(nn);
                ProgramCounter::Next
            }
            // 8XY0 - Set Vy to Vx
            (0x8, _, _, 0x0) => {
                self.v[x] = self.v[y];
                ProgramCounter::Next
            }
            // 8XY1 - Set Vx OR Vy to Vx
            (0x8, _, _, 0x1) => {
                self.v[x] |= self.v[y];
                ProgramCounter::Next
            }
            // 8XY0 - Set Vx OR Vy to Vx
            (0x8, _, _, 0x2) => {
                self.v[x] &= self.v[y];
                ProgramCounter::Next
            }
            // ANNN - load NNN into I
            (0xA, _, _, _) => {
                self.i = nnn;
                ProgramCounter::Next
            }
            _ => {
                panic!("Unknown opcode: {opcode:04X}");
            }
        };

        match pc_change {
            ProgramCounter::Next => self.pc += OPCODE_SIZE,
            ProgramCounter::Skip => self.pc += 2 * OPCODE_SIZE,
            ProgramCounter::Jump(addr) => self.pc = addr,
        }
    }
}
