use std::fmt;
use crate::connector::Connector;
use rand::Rng;
use crate::display::Display;

pub(crate) const START_ADDRESS: u16 = 0x200;
pub struct Cpu {
    vx: [u8; 16],
    prev_pc: u16,
    pc: u16,
    i: u16,
    ret_stack: Vec<u16>,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            vx: [0; 16],
            pc: START_ADDRESS,
            prev_pc: 0,
            i: 0,
            ret_stack: Vec::<u16>::new(),
        }
    }

    pub fn run_instruction(&mut self, connector: &mut Connector) {
        let instruction_high = connector.read_byte_ram(self.pc) as u16;
        let instruction_low = connector.read_byte_ram(self.pc + 1) as u16;
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

        if self.ret_stack.len() > 24 {
            panic!("Too much subroutines. Only 24 are allowed.")
        }

        self.prev_pc = self.pc;

        match (instruction & 0xF000) >> 12 {
            0x0 => {
                match nn {
                    0xEE => {
                        // RETURNS FROM A SUBROUTINE
                        self.pc = self.ret_stack[self.ret_stack.len() - 1];
                        self.ret_stack.pop();
                    },
                    0xE0 => {
                        // CLEARS THE SCREEN
                        self.pc += 2;
                    },
                    _ => {
                        panic!("Unknown instruction for 0x0.")
                    },
                }
            },
            0x1 => {
                // JUMPS TO NNN
                self.pc = nnn;
            },
            0x2 => {
                // CALLS SUBROUTINE AT NNN
                self.ret_stack.push(self.pc + 2);
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
                let temp_x = self.read_reg_vx(x);
                let temp_y = self.read_reg_vx(y);
                match n {
                    0x0 => {
                        // SETS VX TO VY
                        self.write_reg_vx(x, temp_y);
                    },
                    0x1 => {
                        // SETS VX TO BITWISE OPERATION VX or VY
                        self.write_reg_vx(x, temp_x | temp_y);
                    },
                    0x2 => {
                        // SETS VX TO BITWISE OPERATION VX and VY
                        self.write_reg_vx(x, temp_x & temp_y);
                    },
                    0x3 => {
                        // SETS VX TO BITWISE OPERATION VX xor VY
                        self.write_reg_vx(x, temp_x ^ temp_y);
                    },
                    0x4 => {
                        // ADDS VY TO VX. VF IS SET TO 1 IF THERE'S A CARRY, 0 WHEN THERE IS NOT
                        self.write_reg_vx(x, temp_x + temp_y);
                        // ADD CODE TO CHANGE VF
                    },
                    0x5 => {
                        // SUBTRACTS VY OF VX. VF IS SET TO 0 IF THERE'S A BORROW,
                        // 0 WHEN THERE'S NONE
                        self.write_reg_vx(x, temp_x - temp_y);
                        // ADD CODE TO CHANGE VF
                    },
                    0x6 => {
                        // STORES THE LEAST SIGNIFICANT BIT OF VX IN VF AND SHIFTS VX TO THE
                        // RIGHT BY 1
                    },
                    0x7 => {
                        // SET VX TO VY MINUS VX. VF IS SET TO 0 IF THERE'S A BORROW,
                        // 0 WHEN THERE'S NONE
                        self.write_reg_vx(x, temp_y - temp_x);
                        // ADD CODE TO CHANGE VF
                    },
                    0xE => {
                        // STORES THE MOST SIGNIFICANT BIT OF VX IN VF AND SHIFTS VX TO THE
                        // LEFT BY 1
                    },
                    _ => {
                        panic!("Unknown instruction for 0x8.")
                    },
                }
                self.pc += 2;
            }
            0x9 => {
                // SKIPS THE NEXT INSTRUCTION IF VX DOES NOT EQUALS VY
                let temp_x = self.read_reg_vx(x);
                let temp_y = self.read_reg_vx(y);
                if temp_x != temp_y {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            },
            0xA => {
                // SETS I TO NNN
                self.i = nnn;
                self.pc += 2;
            },
            0xB => {
                // JUMPS TO NNN + V0
                let temp = self.vx[0] as u16;
                self.pc = nnn + temp;
            },
            0xC => {
                // SETS VX TO THE RESULT OF A BITWISE & OPERATION ON A RANDOM NUMBER AND NN
                let random_number = rand::thread_rng().gen_range(0, 255) as u8;
                self.write_reg_vx(x, random_number & nn);
                self.pc += 2;
            }
            0xD => {
                // DRAWS SPRITE AT COORDINATE (VX, VY) W 8 PIXELS WIDTH AND N PIXELS HEIGHT
                let temp_x = self.read_reg_vx(x);
                let temp_y = self.read_reg_vx(y);
                self.debug_draw_sprite(connector, temp_x, temp_y, n);
                self.pc += 2;
            },
            0xE => {
                match nn {
                    0x9E => {
                        // SKIPS THE NEXT INSTRUCTION IF THE KEY STORED IN VX IS PRESSED
                        if connector.is_key_pressed(self.read_reg_vx(x)) {
                            self.pc += 4;
                        } else {
                            self.pc += 2;
                        }
                    },
                    0xA1 => {
                        // SKIPS THE NEXT INSTRUCTION IF THE KEY STORED IN VX IS NOT PRESSED
                        if !connector.is_key_pressed(self.read_reg_vx(x)) {
                            self.pc += 4;
                        } else {
                            self.pc += 2;
                        }
                    }
                    _ => {
                        panic!("Unknow instruction for 0xE.")
                    },
                }
            }
            0xF => {
                match nn {
                    0x07 => {
                        // SETS VX TO THE VALUE OF THE DELAY TIMER.
                    },
                    0x0A => {
                        // A KEY PRESSED IS AWAITED AND STORED IN VX.
                    },
                    0x15 => {
                        // SETS THE DELAY TIMER TO VX.
                    },
                    0x18 => {
                        // SETS THE SOUND TIMER TO VX.
                    },
                    0x1E => {
                        // ADDS VX TO I
                        let temp = self.read_reg_vx(x);
                        self.i += temp as u16;
                    },
                    0x29 => {
                        // SETS I TO THE LOCATION OF THE SPRITE FOR THE CHARACTER IN VX.
                    },
                    0x33 => {
                        // STORES THE BINARY-CODED DECIMAL REPRESENTATION OF VX, WITH THE HUNDREDS
                        // DIGIT IN MEMORY AT LOCATION IN I, THE TENS DIGIT AT LOCATION I+1, AND THE
                        // ONES DIGIT AT LOCATION I+2.
                    },
                    0x55 => {
                        // STORES FROM V0 TO VX INCLUDED IN MEMORY, STARTING AT ADDRESS I.
                        // THE OFFSET FROM I IS INCREASED BY 1 FOR EACH VALUE WRITTEN, BUT I
                        // ITSELF IS UNMODIFIED.
                    },
                    0x65 => {
                        // FILLS FROM V0 TO VX INCLUDED WITH VALUES FROM MEMORY, STARTING AT
                        // ADDRESS I. THE OFFSET FROM I IS INCREASED BY 1 FOR EACH VALUE READ, BUT
                        // I ITSELF IS UNMODIFIED.
                    },
                    _ => {
                        panic!("Unknow instruction for 0xF.")
                    }
                }
                self.pc += 2;
            }
            _ =>  {
                panic!("Unknown instruction {:#X}", instruction);
            }
        }
    }

    pub fn debug_draw_sprite(&self, connector: &mut Connector, x: u8, y: u8, height: u8) {
        for j in 0..height { // N pixels height
            let mut b = connector.read_byte_ram(self.i + j as u16);
            connector.debug_draw_byte(b, x, y);
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
