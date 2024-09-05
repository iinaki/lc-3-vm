use std::process;

use termios::Termios;

use crate::{
    input_buffering::{disable_input_buffering, restore_input_buffering},
    memory::Memory,
    operations::handle_operations::handle_operations,
    registers::Registers,
    utils::{flush_stdout, read_image_file},
    vm_error::VmError,
};

/// Represents the virtual machine (VM) that emulates the LC-3 computer.
///
/// # Fields
/// * `registers` - Holds the state of the LC-3 registers.
/// * `memory` - Manages the memory of the LC-3 machine.
/// * `termios` - Stores the terminal I/O settings to handle input buffering.
///
pub struct Vm {
    registers: Registers,
    memory: Memory,
    termios: Termios,
}

impl Vm {
    /// Creates a new `Vm` instance from a set of image files.
    ///
    /// This method initializes the memory and registers of the LC-3 machine,
    /// loads the specified image files into memory, and configures the terminal
    /// for non-blocking input.
    ///
    /// # Arguments
    ///
    /// * `args` - A vector of strings representing the paths to the image files.
    ///
    /// # Returns
    ///
    /// A Result with a fully initialized `Vm` instance ready to run the loaded program or an error if something went wrong.
    ///
    pub fn new_from_images(args: Vec<String>) -> Result<Vm, VmError> {
        let mut memory = Memory::new();
        let registers = Registers::new();
        let termios = disable_input_buffering()?;

        for path in &args[1..] {
            println!("Loading image file: {}", path);
            flush_stdout()?;
            read_image_file(path, &mut memory)?;
        }

        Ok(Vm {
            registers,
            memory,
            termios,
        })
    }

    /// Runs the loaded program.
    ///
    /// This method enters the main loop of the virtual machine, where it fetches, decodes,
    /// and executes instructions. It also manages the program counter (PC)
    /// and ensures that the terminal settings are restored when the execution finishes.
    ///
    /// # Errors
    ///
    /// If the execution encounters a critical error (which causes to halt), the terminal settings will be restored and the program will exit with an error code.
    ///
    /// # Returns
    ///
    /// A Result indicating whether the execution was successful or an error occurred.
    ///
    pub fn run(&mut self) -> Result<(), VmError> {
        let mut running = true;
        while running {
            let pc = self.registers.pc;
            let instr = self.memory.read(pc)?;
            self.registers.pc = self.registers.pc.wrapping_add(1);
            let op = instr >> 12;

            handle_operations(
                &mut self.registers,
                instr,
                op,
                &mut self.memory,
                &mut running,
            )?;
        }
        restore_input_buffering(&self.termios)?;
        process::exit(1);
    }
}
