use std::{env, process};

use lc_3_vm::vm::Vm;

fn main() {
    // @{Load Arguments}
    // handle de args
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: lc3 [image-file1] ...");
        process::exit(2);
    }

    let mut vm = Vm::new_from_images(args);
    vm.run();
}
