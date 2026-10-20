use std::{
    error::Error,
    time::{Duration, Instant},
};

use crate::{
    cpu::Cpu,
    memory::{MEMORY_SIZE, Memory},
    window::Window,
};

pub const PROGRAM_START: usize = 0x200;
const TIMER_TICK: Duration = Duration::from_nanos(16_666_667);

pub struct Chip8 {
    memory: Memory,
    cpu: Cpu,
    window: Window,
    last_timer_update: Instant,
    timer_remainder: Duration,
}

impl Chip8 {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            memory: Memory::new(),
            cpu: Cpu::new(),
            window: Window::new("CHIP-8 Emulator")?,
            last_timer_update: Instant::now(),
            timer_remainder: Duration::ZERO,
        })
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
        self.update_timers();
        let opcode = self.fetch_opcode();

        self.cpu.execute(opcode, &mut self.memory, &mut self.window);
    }

    fn update_timers(&mut self) {
        let now = Instant::now();
        self.timer_remainder += now.duration_since(self.last_timer_update);
        self.last_timer_update = now;

        let elapsed_nanos = self.timer_remainder.as_nanos();
        let tick_nanos = TIMER_TICK.as_nanos();
        let ticks = elapsed_nanos / tick_nanos;
        if ticks > 0 {
            self.cpu.tick_timers(ticks.min(u64::MAX as u128) as u64);
            self.timer_remainder = Duration::from_nanos((elapsed_nanos % tick_nanos) as u64);
        }
    }

    fn fetch_opcode(&self) -> u16 {
        let pc = self.cpu.pc;
        let high = self.memory.read_byte(pc);
        let low = self.memory.read_byte(pc + 1);

        u16::from_be_bytes([high, low])
    }
}
