use std::mem;

use crate::{memory::Memory, register::Register};

// CONDITION FLAGS
const FL_POS: u16 = 1 << 0; /* P */
pub const FL_ZRO: u16 = 1 << 1; /* Z */
const FL_NEG: u16 = 1 << 2; /* N */

// OP CODES
const OP_BR: u16 = 0; /* branch */
const OP_ADD: u16 = 1; /* add  */
const OP_LD: u16 = 2; /* load */
const OP_ST: u16 = 3; /* store */
const OP_JSR: u16 = 4; /* jump register */
const OP_AND: u16 = 5; /* bitwise and */
const OP_LDR: u16 = 6; /* load register */
const OP_STR: u16 = 7; /* store register */
const OP_RTI: u16 = 8; /* unused */
const OP_NOT: u16 = 9; /* bitwise not */
const OP_LDI: u16 = 10; /* load indirect */
const OP_STI: u16 = 11; /* store indirect */
const OP_JMP: u16 = 12; /* jump */
const OP_RES: u16 = 13; /* reserved (unused) */
const OP_LEA: u16 = 14; /* load effective address */
const OP_TRAP: u16 = 15; /* execute trap */

// TRAP CODES
const TRAP_GETC: u16 = 0x20; /* get character from keyboard, not echoed onto the terminal */
const TRAP_OUT: u16 = 0x21; /* output a character */
const TRAP_PUTS: u16 = 0x22; /* output a word string */
const TRAP_IN: u16 = 0x23; /* get character from keyboard, echoed onto the terminal */
const TRAP_PUTSP: u16 = 0x24; /* output a byte string */
const TRAP_HALT: u16 = 0x25; /* halt the program */

pub fn mem_read(address: u16, memory: &Memory) -> u16 {
    memory[address as usize] as u16
}

// uint16_t sign_extend(uint16_t x, int bit_count)
// {
//     if ((x >> (bit_count - 1)) & 1) {
//         x |= (0xFFFF << bit_count);
//     }
//     return x;
// }
fn sign_extend(mut x: u16, bit_count: u16) -> u16 {
    if (x >> (bit_count - 1)) & 1 == 1 {
        x |= 0xFFFF << bit_count;
    }
    x
}

// void update_flags(uint16_t r)
// {
//     if (reg[r] == 0)
//     {
//         reg[R_COND] = FL_ZRO;
//     }
//     else if (reg[r] >> 15) /* a 1 in the left-most bit indicates negative */
//     {
//         reg[R_COND] = FL_NEG;
//     }
//     else
//     {
//         reg[R_COND] = FL_POS;
//     }
// }
fn update_flags(register: &mut Register, r: u16) {
    if register.get(r) == 0 {
        register.R_COND = FL_ZRO;
    } else if (register.get(r) >> 15) & 1 == 1 {
        register.R_COND = FL_NEG;
    } else {
        register.R_COND = FL_POS;
    }
}

// ADD {
//     /* destination register (DR) */
//     uint16_t r0 = (instr >> 9) & 0x7;
//     /* first operand (SR1) */
//     uint16_t r1 = (instr >> 6) & 0x7;
//     /* whether we are in immediate mode */
//     uint16_t imm_flag = (instr >> 5) & 0x1;

//     if (imm_flag)
//     {
//         uint16_t imm5 = sign_extend(instr & 0x1F, 5);
//         reg[r0] = reg[r1] + imm5;
//     }
//     else
//     {
//         uint16_t r2 = instr & 0x7;
//         reg[r0] = reg[r1] + reg[r2];
//     }

//     update_flags(r0);
// }

fn op_add(register: &mut Register, instr: u16) {
    let r0 = (instr >> 9) & 0x7;
    let r1 = (instr >> 6) & 0x7;
    let imm_flag = (instr >> 5) & 0x1;

    if imm_flag == 1 {
        let imm5 = sign_extend(instr & 0x1F, 5);
        register.set(r0, register.get(r1) + imm5);
    } else {
        let r2 = instr & 0x7;
        register.set(r0, register.get(r1) + register.get(r2));
    }

    update_flags(register, r0);
}

// AND {
//     uint16_t r0 = (instr >> 9) & 0x7;
//     uint16_t r1 = (instr >> 6) & 0x7;
//     uint16_t imm_flag = (instr >> 5) & 0x1;

//     if (imm_flag)
//     {
//         uint16_t imm5 = sign_extend(instr & 0x1F, 5);
//         reg[r0] = reg[r1] & imm5;
//     }
//     else
//     {
//         uint16_t r2 = instr & 0x7;
//         reg[r0] = reg[r1] & reg[r2];
//     }
//     update_flags(r0);
// }
fn op_and(register: &mut Register, instr: u16) {
    let r0 = (instr >> 9) & 0x7;
    let r1 = (instr >> 6) & 0x7;
    let imm_flag = (instr >> 5) & 0x1;

    if imm_flag == 1 {
        let imm5 = sign_extend(instr & 0x1F, 5);
        register.set(r0, register.get(r1) & imm5);
    } else {
        let r2 = instr & 0x7;
        register.set(r0, register.get(r1) & register.get(r2));
    }
    update_flags(register, r0);
}

// NOT{
//     uint16_t r0 = (instr >> 9) & 0x7;
//     uint16_t r1 = (instr >> 6) & 0x7;
//     reg[r0] = ~reg[r1];
//     update_flags(r0);
// }

fn op_not(register: &mut Register, instr: u16) {
    let r0 = (instr >> 9) & 0x7;
    let r1 = (instr >> 6) & 0x7;
    register.set(r0, !register.get(r1));
    update_flags(register, r0);
}

// BR {
//     uint16_t pc_offset = sign_extend(instr & 0x1FF, 9);
//     uint16_t cond_flag = (instr >> 9) & 0x7;
//     if (cond_flag & reg[R_COND])
//     {
//         reg[R_PC] += pc_offset;
//     }
// }

fn op_br(register: &mut Register, instr: u16) {
    let pc_offset = sign_extend(instr & 0x1FF, 9);
    let cond_flag = (instr >> 9) & 0x7;
    if cond_flag & register.R_COND as u16 != 0 {
        register.R_PC += pc_offset;
    }
}

// JMP {
//     /* Also handles RET */
//     uint16_t r1 = (instr >> 6) & 0x7;
//     reg[R_PC] = reg[r1];
// }

fn op_jmp(register: &mut Register, instr: u16) {
    let r1 = (instr >> 6) & 0x7;
    register.R_PC = register.get(r1);
}

// {
//     uint16_t long_flag = (instr >> 11) & 1;
//     reg[R_R7] = reg[R_PC];
//     if (long_flag)
//     {
//         uint16_t long_pc_offset = sign_extend(instr & 0x7FF, 11);
//         reg[R_PC] += long_pc_offset;  /* JSR */
//     }
//     else
//     {
//         uint16_t r1 = (instr >> 6) & 0x7;
//         reg[R_PC] = reg[r1]; /* JSRR */
//     }
// }

fn op_jsr(register: &mut Register, instr: u16) {
    let long_flag = (instr >> 11) & 1;
    register.R_R7 = register.R_PC;
    if long_flag == 1 {
        let long_pc_offset = sign_extend(instr & 0x7FF, 11);
        register.R_PC += long_pc_offset; /* JSR */
    } else {
        let r1 = (instr >> 6) & 0x7;
        register.R_PC = register.get(r1); /* JSRR */
    }
}

// LD {
//     uint16_t r0 = (instr >> 9) & 0x7;
//     uint16_t pc_offset = sign_extend(instr & 0x1FF, 9);
//     reg[r0] = mem_read(reg[R_PC] + pc_offset);
//     update_flags(r0);
// }

fn op_ld(register: &mut Register, instr: u16, memory: &Memory) {
    let r0 = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1FF, 9);
    register.set(r0, mem_read(register.R_PC + pc_offset, memory));
    update_flags(register, r0);
}

// LDI {
//     /* destination register (DR) */
//     uint16_t r0 = (instr >> 9) & 0x7;
//     /* PCoffset 9*/
//     uint16_t pc_offset = sign_extend(instr & 0x1FF, 9);
//     /* add pc_offset to the current PC, look at that memory location to get the final address */
//     reg[r0] = mem_read(mem_read(reg[R_PC] + pc_offset));
//     update_flags(r0);
// }

fn op_ldi(register: &mut Register, instr: u16, memory: &Memory) {
    let r0 = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1FF, 9);
    register.set(
        r0,
        mem_read(mem_read(register.R_PC + pc_offset, memory), memory),
    );
    update_flags(register, r0);
}

// LDR {
//     uint16_t r0 = (instr >> 9) & 0x7;
//     uint16_t r1 = (instr >> 6) & 0x7;
//     uint16_t offset = sign_extend(instr & 0x3F, 6);
//     reg[r0] = mem_read(reg[r1] + offset);
//     update_flags(r0);
// }

fn op_ldr(register: &mut Register, instr: u16, memory: &Memory) {
    let r0 = (instr >> 9) & 0x7;
    let r1 = (instr >> 6) & 0x7;
    let offset = sign_extend(instr & 0x3F, 6);
    register.set(r0, mem_read(register.get(r1) + offset, memory));
    update_flags(register, r0);
}

// LEA {
//     uint16_t r0 = (instr >> 9) & 0x7;
//     uint16_t pc_offset = sign_extend(instr & 0x1FF, 9);
//     reg[r0] = reg[R_PC] + pc_offset;
//     update_flags(r0);
// }

fn op_lea(register: &mut Register, instr: u16) {
    let r0 = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1FF, 9);
    register.set(r0, register.R_PC + pc_offset);
    update_flags(register, r0);
}

// ST {
//     uint16_t r0 = (instr >> 9) & 0x7;
//     uint16_t pc_offset = sign_extend(instr & 0x1FF, 9);
//     mem_write(reg[R_PC] + pc_offset, reg[r0]);
// }

fn op_st(register: &mut Register, instr: u16, memory: &mut Memory) {
    let r0 = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1FF, 9);
    memory[(r0 + pc_offset) as usize] = register.get(r0);
}

// STI {
//     uint16_t r0 = (instr >> 9) & 0x7;
//     uint16_t pc_offset = sign_extend(instr & 0x1FF, 9);
//     mem_write(mem_read(reg[R_PC] + pc_offset), reg[r0]);
// }

fn op_sti(register: &mut Register, instr: u16, memory: &mut Memory) {
    let r0 = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1FF, 9);
    memory[mem_read(r0 + pc_offset, memory) as usize] = register.get(r0);
}

// STR {
//     uint16_t r0 = (instr >> 9) & 0x7;
//     uint16_t r1 = (instr >> 6) & 0x7;
//     uint16_t offset = sign_extend(instr & 0x3F, 6);
//     mem_write(reg[r1] + offset, reg[r0]);
// }

fn op_str(register: &mut Register, instr: u16, memory: &mut Memory) {
    let r0 = (instr >> 9) & 0x7;
    let r1 = (instr >> 6) & 0x7;
    let offset = sign_extend(instr & 0x3F, 6);
    memory[(register.get(r1) + offset) as usize] = register.get(r0);
}

// TRAP PUTS {
//     /* one char per word */
//     uint16_t* c = memory + reg[R_R0];
//     while (*c)
//     {
//         putc((char)*c, stdout);
//         ++c;
//     }
//     fflush(stdout);
// }

fn trap_puts(register: &mut Register, memory: &Memory) {
    let mut c = mem_read(register.R_R0, memory);
    while memory[c as usize] != 0 {
        print!("{}", memory[c as usize] as u8 as char);
        c += 1;
    }
}

fn handle_trap(register: &mut Register, instr: u16, memory: &mut Memory) {
    // reg[R_R7] = reg[R_PC];
    register.R_R7 = register.R_PC;

    let trap_instr = instr & 0xFF;
    match trap_instr {
        TRAP_GETC => {
            // @{TRAP GETC}
        }
        TRAP_OUT => {
            // @{TRAP OUT}
        }
        TRAP_PUTS => {
            // @{TRAP PUTS}
            trap_puts(register, memory);
        }
        TRAP_IN => {
            // @{TRAP IN}
        }
        TRAP_PUTSP => {
            // @{TRAP PUTSP}
        }
        TRAP_HALT => {
            // @{TRAP HALT}
        }
        _ => {
            // @{BAD TRAP}
        }
    }
}

pub fn handle_operations(register: &mut Register, instr: u16, op: u16, memory: &mut Memory) {
    match op {
        OP_ADD => {
            // @{ADD}
            op_add(register, instr);
        }
        OP_AND => {
            // @{AND}
            op_and(register, instr);
        }
        OP_NOT => {
            // @{NOT}
            op_not(register, instr)
        }
        OP_BR => {
            // @{BR}
            op_br(register, instr);
        }
        OP_JMP => {
            // @{JMP}
            op_jmp(register, instr);
        }
        OP_JSR => {
            // @{JSR}
            op_jsr(register, instr);
        }
        OP_LD => {
            // @{LD}
            op_ld(register, instr, &memory);
        }
        OP_LDI => {
            // @{LDI}
            op_ldi(register, instr, &memory);
        }
        OP_LDR => {
            // @{LDR}
            op_ldr(register, instr, memory)
        }
        OP_LEA => {
            // @{LEA}
            op_lea(register, instr);
        }
        OP_ST => {
            // @{ST}
            op_st(register, instr, memory);
        }
        OP_STI => {
            // @{STI}
            op_sti(register, instr, memory);
        }
        OP_STR => {
            // @{STR}
            op_str(register, instr, memory);
        }
        OP_TRAP => {
            // @{TRAP}
            handle_trap(register, instr, memory);
        }
        _ => {
            // @{BAD OPCODE}
        }
    }
}
