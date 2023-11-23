pub struct Input {
    key_pressed: Option<u8>,
}

impl Input {
    pub fn new() -> Input {
        Input {
            key_pressed: None
        }
    }

    pub fn is_key_pressed(&self, key: u8) -> bool {
        true
    }
}
