use crate::ram::Ram;
use crate::input::Input;
use crate::display::Display;
use minifb::{Key, Window};
use std::time;

pub struct Connector {
    ram: Ram,
    input: Input,
    display: Display,
    delay_timer: u8,
    sound_timer: u8,
}

impl Connector {
    pub fn new() -> Connector {
        Connector {
            ram: Ram::new(),
            input: Input::new(),
            display: Display::new(),
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    pub fn read_byte_ram(&self, address: u16) -> u8 {
        self.ram.read_byte(address)
    }

    pub fn write_byte_ram(&mut self, address: u16, value: u8) {
        self.ram.write_byte(address, value)
    }

    pub fn debug_draw_byte(&mut self, b: u8, x: u8, y: u8) -> bool {
        self.display.debug_draw_sprite(b, x, y)
    }

    pub fn clear_screen(&mut self) {
        self.display.clear_screen();
    }

    pub fn change_key_pressed(&mut self, key: Option<u8>) {
        self.input.change_key_pressed(key);
    }

    pub fn is_key_pressed(&self, key_tested: u8) -> bool {
        self.input.is_key_pressed(key_tested)
    }

    pub fn get_key_pressed(&self) -> Option<u8> {
        self.input.get_key_pressed()
    }

    pub fn get_delay_timer(&self) -> u8 {
        self.delay_timer
    }

    pub fn change_delay_timer(&mut self, value: u8) {
        self.delay_timer = value;
    }

    pub fn get_sound_timer(&self) -> u8 {
        self.sound_timer
    }

    pub fn change_sound_timer(&mut self, value: u8) {
        self.sound_timer = value;
    }

    pub fn tick(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
    }

    pub fn get_display(&self) -> &[u8] {
        self.display.get_display()
    }

    pub fn get_keycode_by_key(key: Option<Key>) -> Option<u8> {
        match key {
            Some(Key::Key1) => Some(0x1),
            Some(Key::Key2) => Some(0x2),
            Some(Key::Key3) => Some(0x3),
            Some(Key::Key4) => Some(0xC),

            Some(Key::Q) => Some(0x4),
            Some(Key::W) => Some(0x5),
            Some(Key::E) => Some(0x6),
            Some(Key::R) => Some(0xD),

            Some(Key::A) => Some(0x7),
            Some(Key::S) => Some(0x8),
            Some(Key::D) => Some(0x9),
            Some(Key::F) => Some(0xE),

            Some(Key::Z) => Some(0xA),
            Some(Key::X) => Some(0x0),
            Some(Key::C) => Some(0xB),
            Some(Key::V) => Some(0xF),
            _ => None,
        }
    }
}
