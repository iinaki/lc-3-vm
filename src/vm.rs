use std::process;

use termios::Termios;

use crate::{
    constants::{
        OP_ADD, OP_AND, OP_BR, OP_JMP, OP_JSR, OP_LD, OP_LDI, OP_LDR, OP_LEA, OP_NOT, OP_ST,
        OP_STI, OP_STR, OP_TRAP,
    },
    input_buffering::{disable_input_buffering, restore_input_buffering},
    memory::Memory,
    operations::{
        op_add::OpAdd, op_and::OpAnd, op_br::OpBr, op_jmp::OpJmp, op_jsr::OpJsr, op_ld::OpLd,
        op_ldi::OpLdi, op_ldr::OpLdr, op_lea::OpLea, op_not::OpNot, op_st::OpSt, op_sti::OpSti,
        op_str::OpStr, trap::Trap,
    },
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
    pub registers: Registers,
    pub memory: Memory,
    pub termios: Termios,
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

            self.handle_operations(instr, op, &mut running)?;
        }
        restore_input_buffering(&self.termios)?;
        process::exit(1);
    }

    /// Handles the execution of operations based on the provided opcode.
    ///
    /// # Parameters
    ///
    /// - `registers`: A mutable reference to the `Registers` struct.
    /// - `instr`: The instruction to be executed.
    /// - `op`: The operation code extracted from the instruction.
    /// - `memory`: A mutable reference to the `Memory` struct.
    /// - `running`: Boolean flag that indicates if the program is running.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the handling was successful, otherwise returns a `VmError`.
    ///
    pub fn handle_operations(
        &mut self,
        instr: u16,
        op: u16,
        running: &mut bool,
    ) -> Result<(), VmError> {
        match op {
            OP_ADD => self.op_add(instr),
            OP_AND => self.op_and(instr),
            OP_NOT => self.op_not(instr),
            OP_BR => {
                self.op_br(instr);
                Ok(())
            }
            OP_JMP => self.op_jmp(instr),
            OP_JSR => self.op_jsr(instr),
            OP_LD => self.op_ld(instr),
            OP_LDI => self.op_ldi(instr),
            OP_LDR => self.op_ldr(instr),
            OP_LEA => self.op_lea(instr),
            OP_ST => self.op_st(instr),
            OP_STI => self.op_sti(instr),
            OP_STR => self.op_str(instr),
            OP_TRAP => self.handle_trap(instr, running),
            _ => {
                println!("Bad opcode: {}", op);
                flush_stdout()?;
                self.trap_halt(running)
            }
        }
    }
}
