use crate::ram::Ram;
use crate::cpu::Cpu;
use crate::cpu;

pub struct Chip8 {
    pub(crate) ram: Ram,
    cpu: Cpu
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 {
            ram: Ram::new(),
            cpu: Cpu::new()
        }
    }

    pub fn load_rom(&mut self, data: &Vec<u8>) {
        for value in 0..data.len() {
            self.ram.write_byte(cpu::START_ADDRESS + value as u16, data[value]);
        }
    }

    pub fn run_instruction(&mut self) {
        self.cpu.run_instruction(&mut self.ram);
        println!("{:?}", self.cpu);
    }
}