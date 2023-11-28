use crate::cpu::Cpu;
use crate::cpu;
use crate::connector::Connector;

pub struct Chip8 {
    connector: Connector,
    cpu: Cpu
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 {
            connector: Connector::new(),
            cpu: Cpu::new()
        }
    }

    pub fn load_rom(&mut self, data: &Vec<u8>) {
        for value in 0..data.len() {
            self.connector.write_byte_ram(cpu::START_ADDRESS + value as u16, data[value]);
        }
    }

    pub fn run_instruction(&mut self) {
        self.connector.tick();
        self.cpu.run_instruction(&mut self.connector);
        println!("{:?}", self.cpu);
    }

    pub fn get_dislay(&self) -> &[u8] {
        self.connector.get_display()
    }

    pub fn change_key_pressed(&mut self, key: Option<u8>) {
        self.connector.change_key_pressed(key);
    }
}