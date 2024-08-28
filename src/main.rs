use std::{env, process};

use lc_3_vm::register::Register;
use lc_3_vm::opcode::Opcode;
use lc_3_vm::condition_flag::ConditionFlag;
use lc_3_vm::operations::handle_operations;

const MEMORY_MAX: usize = 1 << 16;
const PC_START: u16 = 0x3000;

fn read_image(file_name: &str) -> bool {
    true // Placeholde
}

fn main() {
    let mut memory = [0; MEMORY_MAX];
    let mut register = Register::new();

    // @{Load Arguments}
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: lc3 [image-file1] ...");
        process::exit(2);
    }

    for arg in &args[1..] {
        if !read_image(arg) {
            println!("Failed to load image: {}", arg);
            process::exit(1);
        }
    }

    // @{Setup}

    /* since exactly one condition flag should be set at any given time, set the Z flag */
    register.R_COND = ConditionFlag::FL_ZRO;

    /* set the PC to starting position */
    /* 0x3000 is the default */
    register.R_PC = PC_START;

    let running = true;
    while running {
        /* FETCH */
        let instr: u16 = mem_read(register.R_PC += 1);
        let op: u16 = instr >> 12;

        handle_operations(&mut register, instr, op, &mut memory);
    }
    //@{Shutdown}
}
