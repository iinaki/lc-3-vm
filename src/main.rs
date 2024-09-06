use std::env;

use lc_3_vm::{vm::Vm, vm_error::VmError};

fn main() -> Result<(), VmError> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err(VmError::BadArgsLength(
            "Usage: lc3 [image-file1] ...".to_string(),
        ));
    }

    let mut vm = Vm::new_from_images(args)?;

    vm.run()
}
