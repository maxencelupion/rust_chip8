use std::fs::File;
use std::io::Read;
use chip8::Chip8;
use connector::Connector;
use minifb::{KeyRepeat, Key, WindowOptions, Window};

mod ram;
mod chip8;
mod cpu;
mod connector;
mod input;
mod display;

fn main() {
    // PATH TO THE ROM
    let mut rom = File::open("data/CONNECT4").unwrap();
    let mut data = Vec::<u8>::new();
    // LOAD THE ROM
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

        let chip8_key = Connector::get_keycode_by_key(key);
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
