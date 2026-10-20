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
        win.set_target_fps(60);

        Ok(Self {
            win,
            framebuffer: [PX_OFF; WIDTH * HEIGHT],
        })
    }

    pub fn handle_key_events(&self) -> [bool; 16] {
        let mut keys = [false; 16];
        self.win.get_keys().iter().for_each(|k| {
            match k {
                Key::Key1 => keys[0x1] = true,
                Key::Key2 => keys[0x2] = true,
                Key::Key3 => keys[0x3] = true,
                Key::Key4 => keys[0xc] = true,
                Key::Q => keys[0x4] = true,
                Key::W => keys[0x5] = true,
                Key::E => keys[0x6] = true,
                Key::R => keys[0xd] = true,
                Key::A => keys[0x7] = true,
                Key::S => keys[0x8] = true,
                Key::D => keys[0x9] = true,
                Key::F => keys[0xe] = true,
                Key::Z => keys[0xa] = true,
                Key::X => keys[0x0] = true,
                Key::C => keys[0xb] = true,
                Key::V => keys[0xf] = true,
                _ => (),
            };
        });

        keys
    }

    pub fn is_chip8_key_down(&self, key: u8) -> bool {
        self.handle_key_events()
            .get(key as usize)
            .copied()
            .unwrap_or(false)
    }

    pub fn pressed_chip8_key(&self) -> Option<u8> {
        self.handle_key_events()
            .iter()
            .position(|pressed| *pressed)
            .map(|key| key as u8)
    }

    pub fn is_key_down(&self, key: Key) -> bool {
        self.win.is_key_down(key)
    }

    pub fn is_open(&self) -> bool {
        self.win.is_open()
    }

    pub fn clear(&mut self) {
        self.framebuffer.fill(PX_OFF);
    }

    pub fn draw(&mut self, start_x: usize, start_y: usize, sprite: &[u8]) -> u8 {
        draw_sprite(&mut self.framebuffer, start_x, start_y, sprite)
    }

    pub fn refresh(&mut self) -> Result<(), minifb::Error> {
        self.win
            .update_with_buffer(&self.framebuffer, WIDTH, HEIGHT)?;
        Ok(())
    }
}

fn draw_sprite(
    framebuffer: &mut [u32; WIDTH * HEIGHT],
    start_x: usize,
    start_y: usize,
    sprite: &[u8],
) -> u8 {
    let mut collision = 0;

    for (row, sprite_byte) in sprite.iter().enumerate() {
        for col in 0..8 {
            let x = (start_x + col) % WIDTH;
            let y = (start_y + row) % HEIGHT;
            let index = y * WIDTH + x;
            let is_old_set = framebuffer[index] == PX_ON;

            framebuffer[index] = if is_bit_set(sprite_byte, (8 - col - 1) as u8) {
                if is_old_set {
                    collision = 1;
                    PX_OFF
                } else {
                    PX_ON
                }
            } else {
                framebuffer[index]
            }
        }
    }

    collision
}

fn is_bit_set(byte: &u8, n: u8) -> bool {
    byte & (1 << n) != 0
}

#[cfg(test)]
mod tests {
    use super::{HEIGHT, PX_OFF, PX_ON, WIDTH, draw_sprite};

    #[test]
    fn draw_wraps_around_screen_edges() {
        let mut framebuffer = [PX_OFF; WIDTH * HEIGHT];

        assert_eq!(
            draw_sprite(&mut framebuffer, 63, 31, &[0b1000_0001, 0b1000_0000]),
            0
        );

        assert_eq!(framebuffer[31 * WIDTH + 63], PX_ON);
        assert_eq!(framebuffer[31 * WIDTH + 6], PX_ON);
        assert_eq!(framebuffer[63], PX_ON);
    }

    #[test]
    fn drawing_over_set_pixels_erases_them_and_reports_collision() {
        let mut framebuffer = [PX_OFF; WIDTH * HEIGHT];

        assert_eq!(draw_sprite(&mut framebuffer, 10, 10, &[0b1100_0000]), 0);
        assert_eq!(draw_sprite(&mut framebuffer, 10, 10, &[0b1100_0000]), 1);

        assert_eq!(framebuffer[10 * WIDTH + 10], PX_OFF);
        assert_eq!(framebuffer[10 * WIDTH + 11], PX_OFF);
    }
}
