const WIDTH: usize = 64;
const HEIGHT: usize = 32;

pub struct Display {
    screen: [u8; WIDTH * HEIGHT],
}

impl Display {
    pub fn new() -> Display {
        Display {
            screen: [0; WIDTH * HEIGHT],
        }
    }

    pub fn debug_draw_sprite(&mut self, b: u8, x: u8, y: u8) {
        let mut byte = b;
        for _ in 0..8 { // 8 pixels width
                match (byte & 0b1000_0000) >> 7 {
                    0 => {
                        print!("_");
                    },
                    1 => {
                        print!("#");
                    },
                    _ => unreachable!(),
                }
                byte <<= 1;
            }
    }
}
