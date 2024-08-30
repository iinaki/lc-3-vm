use std::{env, os::fd, process};
use libc::{signal, SIGINT, STDIN_FILENO};
use termios::*;

use lc_3_vm::{
    input_buffering::{disable_input_buffering, handle_interrupt}, memory::Memory, operations::handle_operations, register::Register, utils::read_image_file
};

fn main() {
    let mut memory = Memory::new();
    let mut register = Register::new();

    let mut termios = Termios::from_fd(STDIN_FILENO).unwrap();

    unsafe { signal(SIGINT, handle_interrupt as usize) };
    disable_input_buffering(&mut termios);

    // @{Load Arguments}
    // handle de args
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: lc3 [image-file1] ...");
        process::exit(2);
    }

    let path = &args[1];
    match read_image_file(path, &mut memory) {
        Ok(_) => (),
        Err(e) => {
            println!("Error reading image file: {}", e);
            process::exit(2);
        }
    }

    let mut running = true;
    while running {
        /* FETCH */
        register.R_PC += 1;
        let instr = memory.read(register.R_PC);
        let op = instr >> 12;

        handle_operations(&mut register, instr, op, &mut memory, &mut running);
    }
    //@{Shutdown}
    process::exit(1);
}
