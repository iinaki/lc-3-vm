use std::{env, process};

use lc_3_vm::{
    input_buffering::{disable_input_buffering, restore_input_buffering},
    memory::Memory,
    operations::handle_operations,
    registers::Registers,
    utils::{flush_stdout, read_image_file},
};

fn main() {
    let mut memory = Memory::new();
    let mut registers = Registers::new();

    let termios = disable_input_buffering();

    // @{Load Arguments}
    // handle de args
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: lc3 [image-file1] ...");
        process::exit(2);
    }

    let path = &args[1];
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

    let mut running = true;
    while running {
        //@{Fetch}
        let pc = registers.pc;
        let instr = memory.read(pc);
        registers.pc = registers.pc.wrapping_add(1);
        let op = instr >> 12;

        handle_operations(&mut registers, instr, op, &mut memory, &mut running);
    }
    //@{Shutdown}
    restore_input_buffering(&termios);
    process::exit(1);
}
