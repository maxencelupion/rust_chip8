mod ram;
mod chip8;
mod cpu;

use std::fs::File;
use std::io::Read;
use chip8::Chip8;

fn main() {
    // PATH TO THE ROM TO LOAD
    let mut rom = File::open("data/INVADERS").unwrap();
    let mut data = Vec::<u8>::new();
    rom.read_to_end(&mut data);

    let mut chip8 = Chip8::new();
    chip8.load_rom(&data);

    loop {
        chip8.run_instruction();
    }
}
