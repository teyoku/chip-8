use rand::RngExt;

use crate::{chip8::PROGRAM_START, memory::Memory, window::Window};

const NUM_REGISTERS: usize = 16;
const STACK_SIZE: usize = 16;
const OPCODE_SIZE: u16 = 2;

enum ProgramCounter {
    Next,
    Skip,
    Jump(u16),
    Hold,
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
    pub delay_timer: u8,
    pub sound_timer: u8,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            v: [0; NUM_REGISTERS],
            i: 0,
            pc: PROGRAM_START as u16,
            stack: [0; STACK_SIZE],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    pub(crate) fn tick_timers(&mut self, ticks: u64) {
        let ticks = ticks.min(u8::MAX as u64) as u8;
        self.delay_timer = self.delay_timer.saturating_sub(ticks);
        self.sound_timer = self.sound_timer.saturating_sub(ticks);
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
            // EXA1 - SKNP Vx
            (0xE, _, 0xA, 0x1) => ProgramCounter::skip_if(!window.is_chip8_key_down(self.v[x])),
            // FX07 - LD Vx, delay timer
            (0xF, _, 0x0, 0x7) => {
                self.v[x] = self.delay_timer;
                ProgramCounter::Next
            }
            // FX0A - wait for a key press, then store it in Vx
            (0xF, _, 0x0, 0xA) => {
                if let Some(key) = window.pressed_chip8_key() {
                    self.v[x] = key;
                    ProgramCounter::Next
                } else {
                    ProgramCounter::Hold
                }
            }
            // FX15 - LD delay timer, Vx
            (0xF, _, 0x1, 0x5) => {
                self.delay_timer = self.v[x];
                ProgramCounter::Next
            }
            // FX18 - LD sound timer, Vx
            (0xF, _, 0x1, 0x8) => {
                self.sound_timer = self.v[x];
                ProgramCounter::Next
            }
            // FX1E - ADD I, Vx
            (0xF, _, 0x1, 0xE) => {
                self.i += self.v[x] as u16;
                self.v[0xF] = if self.i > 0x0F00 { 1 } else { 0 };
                ProgramCounter::Next
            }
            // FX29 - LD, F, Vx
            (0xF, _, 0x2, 0x9) => {
                self.i = (self.v[x] as u16) * 5;
                ProgramCounter::Next
            }
            // FX33 - LD, B, Vx
            (0xF, _, 0x3, 0x3) => {
                memory.write_byte(self.i, self.v[x] / 100);
                memory.write_byte(self.i + 1, (self.v[x] % 100) / 00);
                memory.write_byte(self.i + 2, self.v[x] % 10);
                ProgramCounter::Next
            }
            // FX55 - LD [I], Vx
            (0xF, _, 0x5, 0x5) => {
                for i in 0..x + 1 {
                    memory.write_byte(self.i + i as u16, self.v[i]);
                }
                ProgramCounter::Next
            }
            // FX65 - LD Vx, [I]
            (0xF, _, 0x6, 0x5) => {
                for i in 0..x + 1 {
                    self.v[i] = memory.read_byte(self.i + i as u16);
                }
                ProgramCounter::Next
            }
            _ => ProgramCounter::Next,
        };

        match pc_change {
            ProgramCounter::Next => self.pc += OPCODE_SIZE,
            ProgramCounter::Skip => self.pc += 2 * OPCODE_SIZE,
            ProgramCounter::Jump(addr) => self.pc = addr,
            ProgramCounter::Hold => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Cpu;

    #[test]
    fn timers_count_down_without_underflow() {
        let mut cpu = Cpu::new();
        cpu.delay_timer = 3;
        cpu.sound_timer = 1;

        cpu.tick_timers(1);

        assert_eq!(cpu.delay_timer, 2);
        assert_eq!(cpu.sound_timer, 0);

        cpu.tick_timers(10);

        assert_eq!(cpu.delay_timer, 0);
        assert_eq!(cpu.sound_timer, 0);
    }
}
