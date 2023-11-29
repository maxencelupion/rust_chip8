use std::fs::File;
use std::io::Read;
use chip8::Chip8;
use connector::Connector;
use std::time::{Instant, Duration};
use minifb::{KeyRepeat, Key, WindowOptions, Window};

mod ram;
mod chip8;
mod cpu;
mod connector;
mod input;
mod display;

fn main() {
    // PATH TO THE ROM
    let mut rom = File::open("data/PONG").unwrap();
    let mut data = Vec::<u8>::new();

    // LOAD THE ROM
    rom.read_to_end(&mut data).unwrap();

    // WINDOW SIZE
    let width = 640;
    let height = 320;

    // WINDOW BUFFER
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

    let mut last_key_update_time = Instant::now();
    let mut last_instruction_run_time = Instant::now();
    let mut last_display_time = Instant::now();


    while window.is_open() && !window.is_key_down(Key::Escape) {
        let keys_pressed = window.get_keys_pressed(KeyRepeat::Yes);
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
        if chip8_key.is_some() || Instant::now() - last_key_update_time >= Duration::from_millis(200) {
            last_key_update_time = Instant::now();
            chip8.change_key_pressed(chip8_key);
        }

        if Instant::now() - last_instruction_run_time > Duration::from_millis(2) {
            chip8.run_instruction();
            last_instruction_run_time = Instant::now();
        }

        //if chip8.get_sound_timer() == 0 {
            // IMPLEMENT SOUND
        //}

        if Instant::now() - last_display_time > Duration::from_millis(10) {
            let chip8_buffer = chip8.get_display();

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
            last_display_time = Instant::now();
            window.update_with_buffer(&buffer).unwrap();
        }
    }
}
