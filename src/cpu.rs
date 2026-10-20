use rand::RngExt;

use crate::{chip8::PROGRAM_START, memory::Memory, window::Window};

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
            pc: PROGRAM_START as u16,
            stack: [0; STACK_SIZE],
            sp: 0,
        }
    }

    pub fn execute(&mut self, opcode: u16, memory: &mut Memory, window: &mut Window) {
        let c = ((opcode & 0xF000) >> 12) as u8; // Instruction category
        let x = ((opcode & 0x0F00) >> 8) as usize; // Vx register index
        let y = ((opcode & 0x00F0) >> 4) as usize; // Vy register index
        let n = (opcode & 0x000F) as u8; // 4-bit const

        let nn = (opcode & 0x00FF) as u8; // 8-bit const
        let nnn = opcode & 0x0FFF; // 12-bit memory address

        let pc_change = match (c, x, y, n) {
            // 00E0 - clear the display
            (0x00, 0x00, 0x0e, 0x00) => {
                window.clear();
                ProgramCounter::Next
            }
            // 00EE - return from subroutine
            (0x00, 0x00, 0x0e, 0x0e) => {
                self.sp -= 1;
                ProgramCounter::Jump(self.stack[self.sp as usize])
            }
            // 1NNN - goto NNN
            (0x1, _, _, _) => ProgramCounter::Jump(nnn),
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
            // 8XY2 - Set Vx OR Vy to Vx
            (0x8, _, _, 0x2) => {
                self.v[x] &= self.v[y];
                ProgramCounter::Next
            }
            // 8XY3 - Set Vx XOR Vy to Vx
            (0x8, _, _, 0x3) => {
                self.v[x] ^= self.v[y];
                ProgramCounter::Next
            }
            // 8XY4 - add Vy to Vx
            (0x8, _, _, 0x4) => {
                let vx = self.v[x] as u16;
                let vy = self.v[y] as u16;
                let result = vx + vy;
                self.v[x] = result as u8;
                self.v[0xF] = if result > 0xFF { 1 } else { 0 };

                ProgramCounter::Next
            }
            // 8XY5 - sub Vy from Vx
            (0x8, _, _, 0x5) => {
                self.v[0xF] = if self.v[x] > self.v[y] { 1 } else { 0 };
                self.v[x] = self.v[x].wrapping_sub(self.v[y]);
                ProgramCounter::Next
            }
            // 8XY6 - SHR Vx {, Vy}
            (0x8, _, _, 0x6) => {
                self.v[0xF] = self.v[x] & 1;
                self.v[x] >>= 1;
                ProgramCounter::Next
            }
            // 8XY7 - SUBN Vx, Vy
            (0x8, _, _, 0x7) => {
                self.v[0xF] = if self.v[y] > self.v[x] { 1 } else { 0 };
                self.v[x] = self.v[y].wrapping_sub(self.v[x]);
                ProgramCounter::Next
            }
            // 8XYE - SHL Vx {, Vy}
            (0x8, _, _, 0xE) => {
                self.v[0xF] = (self.v[x] & 0b10000000) >> 7;
                self.v[x] <<= 1;
                ProgramCounter::Next
            }
            // 9XY0 - SNE Vx, Vy
            (0x9, _, _, 0x0) => ProgramCounter::skip_if(self.v[x] != self.v[y]),
            // ANNN - load NNN into I
            (0xA, _, _, _) => {
                self.i = nnn;
                ProgramCounter::Next
            }
            // BNNN - goto NNN + V0
            (0xB, _, _, _) => ProgramCounter::Jump(nnn + (self.v[0] as u16)),
            // CXNN - random number
            (0xC, _, _, _) => {
                let mut rng = rand::rng();
                self.v[x] = rng.random::<u8>() & nn;
                ProgramCounter::Next
            }
            // DXYN - draw N-byte sprite at (Vx, Vy), setting VF on collision
            (0xD, _, _, _) => {
                let sprite: Vec<u8> = (0..n)
                    .map(|row| memory.read_byte(self.i + row as u16))
                    .collect();
                self.v[0xF] = window.draw(self.v[x] as usize, self.v[y] as usize, &sprite);
                ProgramCounter::Next
            }
            // EX9E - SKP Vx
            (0xE, _, 0x9, 0xE) => ProgramCounter::skip_if(window.is_chip8_key_down(self.v[x])),
            _ => ProgramCounter::Next,
        };

        match pc_change {
            ProgramCounter::Next => self.pc += OPCODE_SIZE,
            ProgramCounter::Skip => self.pc += 2 * OPCODE_SIZE,
            ProgramCounter::Jump(addr) => self.pc = addr,
        }
    }
}
