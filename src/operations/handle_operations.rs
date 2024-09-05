use crate::{
    constants::{
        OP_ADD, OP_AND, OP_BR, OP_JMP, OP_JSR, OP_LD, OP_LDI, OP_LDR, OP_LEA, OP_NOT, OP_ST,
        OP_STI, OP_STR, OP_TRAP,
    },
    memory::Memory,
    registers::Registers,
    utils::flush_stdout,
    vm_error::VmError,
};

use crate::operations::{
    op_add::op_add,
    op_and::op_and,
    op_br::op_br,
    op_jmp::op_jmp,
    op_jsr::op_jsr,
    op_ld::op_ld,
    op_ldi::op_ldi,
    op_ldr::op_ldr,
    op_lea::op_lea,
    op_not::op_not,
    op_st::op_st,
    op_sti::op_sti,
    op_str::op_str,
    trap::{handle_trap, trap_halt},
};

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
    registers: &mut Registers,
    instr: u16,
    op: u16,
    memory: &mut Memory,
    running: &mut bool,
) -> Result<(), VmError> {
    match op {
        OP_ADD => op_add(registers, instr),
        OP_AND => op_and(registers, instr),
        OP_NOT => op_not(registers, instr),
        OP_BR => {
            op_br(registers, instr);
            Ok(())
        }
        OP_JMP => op_jmp(registers, instr),
        OP_JSR => op_jsr(registers, instr),
        OP_LD => op_ld(registers, instr, memory),
        OP_LDI => op_ldi(registers, instr, memory),
        OP_LDR => op_ldr(registers, instr, memory),
        OP_LEA => op_lea(registers, instr),
        OP_ST => op_st(registers, instr, memory),
        OP_STI => op_sti(registers, instr, memory),
        OP_STR => op_str(registers, instr, memory),
        OP_TRAP => handle_trap(registers, instr, memory, running),
        _ => {
            println!("Bad opcode: {}", op);
            flush_stdout()?;
            trap_halt(running)
        }
    }
}
