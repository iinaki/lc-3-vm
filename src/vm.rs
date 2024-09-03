use std::process;

use termios::Termios;

use crate::{
    input_buffering::{disable_input_buffering, restore_input_buffering},
    memory::Memory,
    operations::handle_operations,
    registers::Registers,
    utils::{flush_stdout, read_image_file},
};

pub struct Vm {
    registers: Registers,
    memory: Memory,
    termios: Termios,
}

impl Vm {
    pub fn new_from_images(args: Vec<String>) -> Vm {
        let mut memory = Memory::new();
        let registers = Registers::new();
        let termios = disable_input_buffering();

        for path in &args[1..] {
            println!("Loading image file: {}", path);
            flush_stdout();
            match read_image_file(path, &mut memory) {
                Ok(_) => (),
                Err(e) => {
                    println!("Error reading image file: {}", e);
                    flush_stdout();
                    restore_input_buffering(&termios);
                    process::exit(2);
                }
            }
        }

        Vm {
            registers,
            memory,
            termios,
        }
    }

    pub fn run(&mut self) {
        let mut running = true;
        while running {
            //@{Fetch}
            let pc = self.registers.pc;
            let instr = self.memory.read(pc);
            self.registers.pc = self.registers.pc.wrapping_add(1);
            let op = instr >> 12;

            handle_operations(
                &mut self.registers,
                instr,
                op,
                &mut self.memory,
                &mut running,
            );
        }
        //@{Shutdown}
        restore_input_buffering(&self.termios);
        process::exit(1);
    }
}
