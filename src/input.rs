use minifb::{Key, Window};

pub struct Input {
    key_pressed: Option<u8>,
}

impl Input {
    pub fn new() -> Input {
        Input {
            key_pressed: None
        }
    }

    pub fn is_key_pressed(&self, key_tested: u8) -> bool {
        match self.key_pressed {
            Some(key) => key == key_tested,
            _ => false
        }
    }

    pub fn change_key_pressed(&mut self, key: Option<u8>) {
        self.key_pressed = key;
    }

    pub fn get_key_pressed(&self) -> Option<u8> {
        self.key_pressed
    }
}
