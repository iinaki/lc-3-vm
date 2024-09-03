use std::io::Read;

use crate::{
    constants::{MEMORY_SIZE, MR_KBDR, MR_KBSR},
    utils::flush_stdout,
};

#[derive(Debug)]
pub struct Memory {
    pub memory: [u16; MEMORY_SIZE],
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            memory: [0; MEMORY_SIZE],
        }
    }

    pub fn read(&mut self, address: u16) -> u16 {
        if address == MR_KBSR {
            let mut buffer = [0; 1];
            let char = match std::io::stdin().read_exact(&mut buffer) {
                Ok(_) => buffer[0] as u16,
                Err(e) => {
                    println!("Error reading from stdin: {}", e);
                    flush_stdout();
                    0
                }
            };
            if char != 0 {
                self.memory[MR_KBSR as usize] = 1 << 15;
                self.memory[MR_KBDR as usize] = char;
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
