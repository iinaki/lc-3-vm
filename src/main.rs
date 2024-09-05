use std::env;

use lc_3_vm::{vm::Vm, vm_error::VmError};

fn main() -> Result<(), VmError> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err(VmError::BadArgsLength(
            "Usage: lc3 [image-file1] ...".to_string(),
        ));
    }

    // let mut vm = match Vm::new_from_images(args) {
    //     Ok(vm) => vm,
    //     Err(e) => {
    //         eprintln!("Error initializing VM: {}", e);
    //         process::exit(2);
    //     }
    // };
    let mut vm = Vm::new_from_images(args)?;

    // match vm.run() {
    //     Ok(_) => {}
    //     Err(e) => {
    //         eprintln!("Error running VM: {}", e);
    //         process::exit(2);
    //     }
    // }

    vm.run()
}
