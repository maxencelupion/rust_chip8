use std::fmt;
use crate::ram::Ram;
pub(crate) const START_ADDRESS: u16 = 0x200;
pub struct Cpu {
    vx: [u8; 16],
    prev_pc: u16,
    pc: u16,
    i: u16,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            vx: [0; 16],
            pc: START_ADDRESS,
            prev_pc: 0,
            i: 0,
        }
    }

    pub fn run_instruction(&mut self, ram: &mut Ram) {
        let instruction_high = ram.read_byte(self.pc) as u16;
        let instruction_low = ram.read_byte(self.pc + 1) as u16;
        let instruction: u16 =  (instruction_high << 8) | instruction_low;
        let nnn = instruction & 0x0FFF; // 12 bits
        let nn = (instruction & 0x0FF) as u8; // 8 bits
        let n = (instruction & 0x00F) as u8; // 4 bits
        let x = ((instruction & 0x0F00) >> 8) as u8; // 4 bits
        let y = ((instruction & 0x00F0) >> 4) as u8; // 4 bits

        println!("Instruction: {:#X}", instruction);
        println!("nn: {:#X}", nn);

        if self.prev_pc == self.pc {
            panic!("Infinite loop detected");
        }

        self.prev_pc = self.pc;

        match (instruction & 0xF000) >> 12 {
            0x1 => {
                // JUMPS TO NNN
                self.pc = nnn;
            },
            0x3 => {
                // SKIPS THE NEXT INSTRUCTION IF VX EQUALS NN
                let temp = self.read_reg_vx(x);
                if temp == nn {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            },
            0x4 => {
                // SKIPS THE NEXT INSTRUCTION IF VX DOES NOT EQUAL NN
                let temp = self.read_reg_vx(x);
                if temp != nn {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            },
            0x5 => {
              // SKIPS THE NEXT INSTRUCTION IF VX EQUALS VY
                let temp_x = self.read_reg_vx(x);
                let temp_y = self.read_reg_vx(y);
                if temp_x == temp_y {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            },
            0x6 => {
                // SETS VX TO NN
                self.write_reg_vx(x, nn);
                self.pc += 2;
            },
            0x7 => {
                // ADDS NN TO VX
                let temp = self.read_reg_vx(x);
                self.write_reg_vx(x, temp.wrapping_add(nn));
                self.pc += 2;
            }
            0x8 => {
                // SETS VX TO VY
                let temp = self.read_reg_vx(y);
                self.write_reg_vx(x, temp);
                self.pc += 2;
            }
            0xA => {
                // SETS I TO NNN
                self.i = nnn;
                self.pc += 2;
            },
            0xD => {
                // DRAWS SPRITE AT COORDINATE (VX, VY) W 8 PIXELS WIDTH AND N PIXELS HEIGHT
                let temp_x = self.read_reg_vx(x);
                let temp_y = self.read_reg_vx(y);
                //self.draw_sprite(ram, temp_x, temp_y, n);
                self.pc += 2;
            },
            0xF => {
                // ADDS VX TO I
                let temp = self.read_reg_vx(x);
                self.i += temp as u16;
                self.pc += 2;
            }
            _ => panic!("Unknown instruction {:#X}", instruction),
        }
    }

    pub fn draw_sprite(&self, ram: &mut Ram, x: u8, y: u8, height: u8) {
        for j in 0..height {
            let mut b = ram.read_byte(self.i + j as u16);
            for _ in 0..8 {
                match (b & 0b1000_0000) >> 7 {
                    0 => {
                        print!("_");
                    },
                    1 => {
                        print!("#");
                    },
                    _ => unreachable!(),
                }
                b <<= 1;
            }
            println!();
        }
        println!();
    }

    pub fn write_reg_vx(&mut self, x: u8, value: u8) {
        self.vx[x as usize] = value;
    }

    pub fn read_reg_vx(&mut self, x: u8) -> u8 {
        self.vx[x as usize]
    }
}

impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "pc: {:#X}\n", self.pc);
        write!(f, "vx: ");
        for i in self.vx.iter() {
            write!(f, "{:#X} ", *i);
        }
        write!(f, "\n");
        write!(f, "i: {:#X}\n", self.i)
    }
}
