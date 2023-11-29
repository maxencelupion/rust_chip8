use crate::connector::Connector;
use rand::Rng;

pub(crate) const START_ADDRESS: u16 = 0x200;
pub struct Cpu {
    // 16 REGISTERS V0 TO VF
    vx: [u8; 16],
    pc: u16,
    i: u16,
    // CONTAINS ADDRESS TO RETURNS FROM SUB ROUTINES
    ret_stack: Vec<u16>,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            vx: [0; 16],
            pc: START_ADDRESS,
            i: 0,
            ret_stack: Vec::<u16>::new(),
        }
    }

    pub fn run_instruction(&mut self, connector: &mut Connector) {
        let high = connector.read_byte_ram(self.pc) as u16;
        let low = connector.read_byte_ram(self.pc + 1) as u16;
        let instruction: u16 =  (high << 8) | low;
        let nnn = instruction & 0x0FFF;
        let nn = (instruction & 0x0FF) as u8;
        let n = (instruction & 0x00F) as u8;
        let x = ((instruction & 0x0F00) >> 8) as u8;
        let y = ((instruction & 0x00F0) >> 4) as u8;

        if self.ret_stack.len() > 24 {
            panic!("Too much subroutines. Only 24 are allowed.")
        }

        match (instruction & 0xF000) >> 12 {
            0x0 => {
                match nn {
                    0xEE => {
                        // RETURNS FROM A SUBROUTINE
                        let addr = self.ret_stack.pop().unwrap();
                        self.pc = addr;
                    },
                    0xE0 => {
                        // CLEARS THE SCREEN
                        connector.clear_screen();
                        self.pc += 2;
                    },
                    _ => {
                        panic!("Unknown instruction for 0x0: {:#X}", nn);
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
                        let (total, overflow) = temp_x.overflowing_add(temp_y);
                        self.write_reg_vx(x, total);
                        self.write_reg_vx(0xF, overflow as u8);
                    },
                    0x5 => {
                        // SUBTRACTS VY OF VX. VF IS SET TO 0 IF THERE'S A BORROW,
                        // 0 WHEN THERE'S NONE
                        let (total, underflow) = temp_x.overflowing_sub(temp_y);
                        self.write_reg_vx(x, total);
                        self.write_reg_vx(0xF, underflow as u8);
                    },
                    0x6 => {
                        // Stores the least significant bit of VX in VF and then shifts
                        // VX to the right by 1.
                        self.write_reg_vx(0xF, temp_x & 0x1);
                        self.write_reg_vx(x, temp_x >> 1);
                    },
                    0x7 => {
                        // SETS VX TO VY MINUS VX. VF IS SET TO 0 WHEN THERE'S A BORROW AND 1 WHEN
                        // THERE'S NONE
                        let (total, underflow) = temp_y.overflowing_sub(temp_x);
                        self.write_reg_vx(x, total);
                        self.write_reg_vx(0xF, underflow as u8);
                    },
                    0xE => {
                        // STORES THE MOST SIGNIFICANT BIT OF VX IN VF AND THEN SHIFTS VX TO THE
                        // LEFT BY 1
                        let most = temp_x >> 7;
                        let shifted = temp_x << 1;
                        self.write_reg_vx(0xF, most);
                        self.write_reg_vx(x, shifted);
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
                        let key = self.read_reg_vx(x);
                        if connector.is_key_pressed(key) {
                            self.pc += 4;
                        } else {
                            self.pc += 2;
                        }
                    },
                    0xA1 => {
                        // SKIPS THE NEXT INSTRUCTION IF THE KEY STORED IN VX IS NOT PRESSED
                        let key = self.read_reg_vx(x);
                        if !connector.is_key_pressed(key) {
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
                let temp_x = self.read_reg_vx(x);
                match nn {
                    0x07 => {
                        // SETS VX TO THE VALUE OF THE DELAY TIMER.
                        let temp = connector.get_delay_timer();
                        self.write_reg_vx(x, temp);
                        self.pc += 2;
                    },
                    0x0A => {
                        // A KEY PRESSED IS AWAITED AND STORED IN VX.
                        let key = connector.get_key_pressed();
                        match key {
                            Some(val) => {
                                self.write_reg_vx(x, val);
                                self.pc += 2;
                            },
                            None => {
                            }
                        }
                    },
                    0x15 => {
                        // SETS THE DELAY TIMER TO VX.
                        connector.change_delay_timer(temp_x);
                        self.pc += 2;
                    },
                    0x18 => {
                        // SETS THE SOUND TIMER TO VX.
                        connector.change_sound_timer(temp_x);
                        self.pc += 2;
                    },
                    0x1E => {
                        // ADDS VX TO I
                        self.i += temp_x as u16;
                        self.pc += 2;
                    },
                    0x29 => {
                        // SETS I TO THE LOCATION OF THE SPRITE FOR THE CHARACTER IN VX.
                        self.i = temp_x as u16 * 5;
                        self.pc += 2;
                    },
                    0x33 => {
                        // STORES THE BINARY-CODED DECIMAL REPRESENTATION OF VX, WITH THE HUNDREDS
                        // DIGIT IN MEMORY AT LOCATION IN I, THE TENS DIGIT AT LOCATION I+1, AND THE
                        // ONES DIGIT AT LOCATION I+2.
                        connector.write_byte_ram(self.i, temp_x / 100);
                        connector.write_byte_ram(self.i + 1, (temp_x % 100) / 10);
                        connector.write_byte_ram(self.i + 2, temp_x % 10);
                        self.pc += 2;
                    },
                    0x55 => {
                        // STORES FROM V0 TO VX INCLUDED IN MEMORY, STARTING AT ADDRESS I.
                        // THE OFFSET FROM I IS INCREASED BY 1 FOR EACH VALUE WRITTEN, BUT I
                        // ITSELF IS UNMODIFIED.
                        for j in 0..=x as usize {
                            let value = self.read_reg_vx(j as u8);
                            connector.write_byte_ram(self.i + j as u16, value);
                        }
                        self.pc += 2;
                    },
                    0x65 => {
                        // FILLS FROM V0 TO VX INCLUDED WITH VALUES FROM MEMORY, STARTING AT
                        // ADDRESS I. THE OFFSET FROM I IS INCREASED BY 1 FOR EACH VALUE READ, BUT
                        // I ITSELF IS UNMODIFIED.
                        for j in 0..=x as usize {
                            let value = connector.read_byte_ram(self.i + j as u16);
                            self.write_reg_vx(j as u8, value);
                        }
                        self.pc += 2;
                    },
                    _ => {
                        panic!("Unknow instruction for 0xF.")
                    }
                }
            }
            _ =>  {
                panic!("Unknown instruction {:#X}", instruction);
            }
        }
    }

    pub fn debug_draw_sprite(&mut self, connector: &mut Connector, x: u8, y: u8, height: u8) {
        let mut should_set_vf = false;
        for sprite_y in 0..height {
            let b = connector.read_byte_ram(self.i + sprite_y as u16);
            if connector.debug_draw_byte(b, x, y + sprite_y) {
                should_set_vf = true;
            }
        }
        if should_set_vf {
            self.write_reg_vx(0xF, 1);
        } else {
            self.write_reg_vx(0xF, 0);
        }
    }

    pub fn write_reg_vx(&mut self, x: u8, value: u8) {
        self.vx[x as usize] = value;
    }

    pub fn read_reg_vx(&mut self, x: u8) -> u8 {
        self.vx[x as usize]
    }
}
