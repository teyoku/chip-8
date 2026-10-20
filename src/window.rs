use minifb::{Key, Scale, WindowOptions};

pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;
const PX_OFF: u32 = 0x81c784;
const PX_ON: u32 = 0x29302a;

pub struct Window {
    win: minifb::Window,
    framebuffer: [u32; WIDTH * HEIGHT],
}

impl Window {
    pub fn new(title: &str) -> Result<Self, minifb::Error> {
        let mut win = minifb::Window::new(
            title,
            WIDTH,
            HEIGHT,
            WindowOptions {
                scale: Scale::X8,
                ..WindowOptions::default()
            },
        )?;
        win.set_target_fps(20);

        Ok(Self {
            win,
            framebuffer: [PX_OFF; WIDTH * HEIGHT],
        })
    }

    pub fn is_key_down(&self, key: Key) -> bool {
        self.is_key_down(key)
    }

    pub fn is_open(&self) -> bool {
        self.win.is_open()
    }

    pub fn clear(&mut self) {
        self.framebuffer.fill(PX_OFF);
    }

    pub fn draw(&mut self, start_x: usize, start_y: usize, sprite: &[u8]) -> u8 {
        let mut collision = 0;

        for (row, sprite_byte) in sprite.iter().enumerate() {
            for col in 0..8 {
                let x = (start_x + col) % WIDTH;
                let y = (start_y + row) % HEIGHT;
                let index = y * WIDTH + x;
                let is_old_set = self.framebuffer[index] == PX_ON;

                self.framebuffer[index] = if is_bit_set(sprite_byte, (8 - col - 1) as u8) {
                    if is_old_set {
                        collision = 1;
                        PX_OFF
                    } else {
                        PX_ON
                    }
                } else {
                    self.framebuffer[index]
                }
            }
        }

        collision
    }

    pub fn refresh(&mut self) -> Result<(), minifb::Error> {
        self.win
            .update_with_buffer(&self.framebuffer, WIDTH, HEIGHT)?;
        Ok(())
    }
}

fn is_bit_set(byte: &u8, n: u8) -> bool {
    byte & (1 << n) != 0
}