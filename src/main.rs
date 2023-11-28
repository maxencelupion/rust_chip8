mod ram;
mod chip8;
mod cpu;
mod connector;
mod input;
mod display;

use std::fs::File;
use std::io::Read;
use chip8::Chip8;
extern crate minifb;
use minifb::{KeyRepeat, Key, WindowOptions, Window};

fn get_keycode_by_key(key: Option<Key>) -> Option<u8> {
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

fn main() {
    // PATH TO THE ROM TO LOAD
    let mut rom = File::open("data/INVADERS").unwrap();
    let mut data = Vec::<u8>::new();
    rom.read_to_end(&mut data);

    let width = 640;
    let height = 320;


    let mut buffer: Vec<u32> = vec![0; width * height];
    let mut window = Window::new(
        "Rust Chip8 Emulator ! - ESC to exit",
        width,
        height,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut chip8 = Chip8::new();
    chip8.load_rom(&data);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let keys_pressed = window.get_keys_pressed(KeyRepeat::No);
        let key: Option<_> = match keys_pressed {
            Some(keys) => {
                if keys.len() > 0 {
                    Some(keys[0])
                } else {
                    None
                }
            }
            None => None
        };

        let chip8_key = get_keycode_by_key(key);
        chip8.change_key_pressed(chip8_key);

        chip8.run_instruction();
        let chip8_buffer = chip8.get_dislay();

       for y in 0..height {
            let y_coord = y / 10;
            let offset = y * width;
            for x in 0..width {
                    let index = display::Display::get_position_from_coords(x / 10, y_coord);
                let pixel = chip8_buffer[index];
                let color_pixel = match pixel {
                     0 => 0x0,
                     1 => 0xffffff,
                     _ => unreachable!()
                };
                buffer[offset + x] = color_pixel;
            }
        }

        window.update_with_buffer(&buffer).unwrap();
    }
}
