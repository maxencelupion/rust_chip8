// ORIGINAL SIZE
pub const WIDTH_SCREEN: usize = 64;
pub const HEIGHT_SCREEN: usize = 32;

pub struct Display {
    screen: [u8; WIDTH_SCREEN * HEIGHT_SCREEN],
}

impl Display {
    pub fn new() -> Display {
        Display {
            screen: [0; WIDTH_SCREEN * HEIGHT_SCREEN],
        }
    }

    pub fn get_position_from_coords(x: usize, y: usize) -> usize {
        y * WIDTH_SCREEN + x
    }
    pub fn debug_draw_sprite(&mut self, b: u8, x: u8, y: u8) -> bool {
        let mut collision = false;
        let mut coord_x = x as usize;
        let mut coord_y = y as usize;
        let mut byte = b;

        for _ in 0..8 {
            coord_x %= WIDTH_SCREEN;
            coord_y %= HEIGHT_SCREEN;
            let position = Display::get_position_from_coords(coord_x, coord_y);
            let bit = (byte & 0b1000_0000) >> 7;
            let prev_value = self.screen[position];
            self.screen[position] ^= bit;

            if prev_value == 1 && self.screen[position] == 0 {
                collision = true;
            }
            coord_x += 1;
            byte = byte << 1;
        }
        collision
    }

    pub fn clear_screen(&mut self) {
        for i in &mut self.screen {
            *i = 0;
        }
    }

    pub fn get_display(&self) -> &[u8] {
        &self.screen
    }

}
