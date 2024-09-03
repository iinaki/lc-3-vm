use std::io::Read;

use crate::{
    constants::{MEMORY_SIZE, MR_KBDR, MR_KBSR},
    input_buffering::check_key,
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
            // pub fn handle_keyboard(memory: &mut Memory) {
            //     let mut buffer = [0; 1];
            //     std::io::stdin().read_exact(&mut buffer).unwrap();
            
            //     if buffer[0] != 0 {
            //         memory.write(MemoryMappedReg::Kbsr as u16, 1 << 15);
            //         memory.write(MemoryMappedReg::Kbdr as u16, buffer[0] as u16);
            //     } else {
            //         memory.write(MemoryMappedReg::Kbsr as u16, 0)
            //     }
            // }

            // println!("Checking for key");
            // if check_key() {
            //     self.memory[MR_KBSR as usize] = 1 << 15;

            //     // getchar
            //     let mut buffer = [0; 1];
            //     self.memory[MR_KBDR as usize] = match std::io::stdin().read(&mut buffer) {
            //         Ok(_) => buffer[0] as u16,
            //         Err(e) => {
            //             println!("Error reading from stdin: {}", e);
            //             0
            //         }
            //     };
            // } else {
            //     self.memory[MR_KBSR as usize] = 0;
            // }
            let mut buffer = [0; 1];
            self.memory[MR_KBDR as usize] = match std::io::stdin().read_exact(&mut buffer) {
                Ok(_) => buffer[0] as u16,
                Err(e) => {
                    println!("Error reading from stdin: {}", e);
                    0
                }
            };
            if buffer[0] != 0 {
                self.memory[MR_KBSR as usize] = 1 << 15;
            } else {
                self.memory[MR_KBSR as usize] = 0;
                self.memory[MR_KBDR as usize] = 0;
            }
        }
        self.memory[address as usize]
    }

    pub fn write(&mut self, address: u16, val: u16) {
        self.memory[address as usize] = val;
    }
}
