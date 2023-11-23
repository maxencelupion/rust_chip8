use crate::ram::Ram;
use crate::input::Input;
use crate::display::Display;

pub struct Connector {
    ram: Ram,
    input: Input,
    display: Display
}

impl Connector {
    pub fn new() -> Connector {
        Connector {
            ram: Ram::new(),
            input: Input::new(),
            display: Display::new()
        }
    }

    pub fn read_byte_ram(&self, address: u16) -> u8 {
        self.ram.read_byte(address)
    }

    pub fn write_byte_ram(&mut self, address: u16, value: u8) {
        self.ram.write_byte(address, value)
    }

    pub fn debug_draw_byte(&mut self, b: u8, x: u8, y: u8) {
        self.display.debug_draw_sprite(b, x, y)
    }

    pub fn is_key_pressed(&self, key: u8) -> bool {
        self.input.is_key_pressed(key)
    }
}
