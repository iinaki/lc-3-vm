use std::{env, process};

use lc_3_vm::vm::Vm;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: lc3 [image-file1] ...");
        process::exit(2);
    }

    let mut vm = match Vm::new_from_images(args) {
        Ok(vm) => vm,
        Err(e) => {
            eprintln!("Error initializing VM: {}", e);
            process::exit(2);
        }
    };

    match vm.run() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error running VM: {}", e);
            process::exit(2);
        }
    }
}
