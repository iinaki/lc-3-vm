pub mod op_add;
pub mod op_and;
pub mod op_br;
pub mod op_jmp;
pub mod op_jsr;
pub mod op_ld;
pub mod op_ldi;
pub mod op_ldr;
pub mod op_lea;
pub mod op_not;
pub mod op_st;
pub mod op_sti;
pub mod op_str;
pub mod trap;

use std::io::Write;

use crate::{
    constants::{
        FL_NEG, FL_POS, FL_ZRO, OP_ADD, OP_AND, OP_BR, OP_JMP, OP_JSR, OP_LD, OP_LDI, OP_LDR,
        OP_LEA, OP_NOT, OP_ST, OP_STI, OP_STR, OP_TRAP,
    },
    memory::Memory,
    register::Register,
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

fn sign_extend(x: u16, bit_count: u16) -> i16 {
    let y = if (x >> (bit_count - 1)) & 1 != 0 {
        x | (0xFFFF << bit_count)
    } else {
        x
    };
    y as i16
}

/// Flushes the stdout buffer
fn flush_stdout() {
    match std::io::stdout().flush() {
        Ok(_) => {}
        Err(e) => {
            println!("Error flushing stdout: {}", e);
        }
    };
}

fn update_flags(register: &mut Register, r: u16) {
    if register.get(r) == 0 {
        register.cond = FL_ZRO;
    } else if (register.get(r) >> 15) & 1 == 1 {
        register.cond = FL_NEG;
    } else {
        register.cond = FL_POS;
    }
}

pub fn handle_operations(
    register: &mut Register,
    instr: u16,
    op: u16,
    memory: &mut Memory,
    running: &mut bool,
) {
    println!("PERFORMING OP: {}", op);
    match op {
        OP_ADD => {
            op_add(register, instr);
        }
        OP_AND => {
            op_and(register, instr);
        }
        OP_NOT => op_not(register, instr),
        OP_BR => {
            op_br(register, instr);
        }
        OP_JMP => {
            op_jmp(register, instr);
        }
        OP_JSR => {
            op_jsr(register, instr);
        }
        OP_LD => {
            op_ld(register, instr, memory);
        }
        OP_LDI => {
            op_ldi(register, instr, memory);
        }
        OP_LDR => op_ldr(register, instr, memory),
        OP_LEA => {
            op_lea(register, instr);
        }
        OP_ST => {
            op_st(register, instr, memory);
        }
        OP_STI => {
            op_sti(register, instr, memory);
        }
        OP_STR => {
            op_str(register, instr, memory);
        }
        OP_TRAP => {
            handle_trap(register, instr, memory, running);
        }
        _ => {
            println!("Bad opcode: {}", op);
            trap_halt(running);
        }
    }
}
