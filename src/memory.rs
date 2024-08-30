use std::io::Read;

use crate::{constants::{MR_KBDR, MR_KBSR}, input_buffering::check_key, operations::trap_getc};

pub const MEMORY_SIZE: usize = 65536;

pub struct Memory {
    pub memory: [u16; MEMORY_SIZE],
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            memory: [0; MEMORY_SIZE],
        }
    }

    pub fn read(&mut self, address: u16) -> u16 {
        if address == MR_KBSR {
            if check_key() {
                self.memory[MR_KBSR as usize] = 1 << 15;

                // getchar
                let mut buffer = [0; 1];
                self.memory[MR_KBDR as usize] = match std::io::stdin().read_exact(&mut buffer) {
                    Ok(_) => buffer[0] as u16,
                    Err(e) => {
                        println!("Error reading from stdin: {}", e);
                        0
                    }
                };
                 
            } else {
                self.memory[MR_KBSR as usize] = 0;
            }
        }
        self.memory[address as usize]
    }

    pub fn write(&mut self, address: u16, val: u16) {
        self.memory[address as usize] = val;
    }
}
