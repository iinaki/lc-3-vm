use std::{env, process};

use lc_3_vm::{
    memory::{Memory, MEMORY_SIZE},
    operations::{handle_operations, mem_read},
    register::Register,
    utils::read_image_file,
};

fn main() {
    let mut memory: Memory = [0; MEMORY_SIZE];
    let mut register = Register::new();

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
        let instr = mem_read(register.R_PC, &memory);
        let op = instr >> 12;

        handle_operations(&mut register, instr, op, &mut memory, &mut running);
    }
    //@{Shutdown}
    process::exit(1);
}
