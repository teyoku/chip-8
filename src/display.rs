pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;

pub struct Display {
    pixels: [bool; WIDTH * HEIGHT],
}

impl Display {
    pub fn new() -> Self {
        Self {
            pixels: [false; WIDTH * HEIGHT],
        }
    }

    pub fn clear(&mut self) {
        self.pixels.fill(false);
    }

    pub fn draw_sprite(&mut self, start_x: usize, start_y: usize, sprite: &[u8]) -> bool {
        let mut collision = false;
        let sprite_height = sprite.len();

        for (row, sprite_byte) in (0..sprite_height).enumerate() {
            for col in 0..8 {
                let bit = (sprite_byte >> (7 - col)) & 1;

                if bit == 1 {
                    let x = (start_x + col) % WIDTH;
                    let y = (start_y + row) % HEIGHT;
                    let index = y * WIDTH + x;

                    if self.pixels[index] {
                        collision = true;
                        self.pixels[index] = false;
                    } else {
                        self.pixels[index] = true;
                    }
                }
            }
        }

        collision
    }

    pub fn flush_to_buffer(
        &self,
        buffer: &mut [u32; WIDTH * HEIGHT],
        fg_color: u32,
        bg_color: u32,
    ) {
        for (i, &pixel_on) in self.pixels.iter().enumerate() {
            buffer[i] = if pixel_on { fg_color } else { bg_color };
        }
    }
}
