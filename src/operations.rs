use crate::{condition_flag::ConditionFlag, memory::Memory, opcode::Opcode, register::Register};

fn mem_read(address: u16, memory: &Memory) -> u16 {
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
        register.R_COND = ConditionFlag::FL_ZRO;
    } else if (register.get(r) >> 15) & 1 == 1 {
        register.R_COND = ConditionFlag::FL_NEG;
    } else {
        register.R_COND = ConditionFlag::FL_POS;
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
        register.R_R0 = register.get(r1)  + imm5;
    } else {
        let r2 = instr & 0x7;
        register.R_R0 = register.get(r1) + register.get(r2);
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
        register.R_R0 = register.get(r1) & imm5;
    } else {
        let r2 = instr & 0x7;
        register.R_R0 = register.get(r1) & register.get(r2);
        
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
    register.R_R0 = !register.get(r1);
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
    register.R_R0 = mem_read(register.R_PC + pc_offset, memory);
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
    register.R_R0 = mem_read(mem_read(register.R_PC + pc_offset, memory), memory);
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
    register.R_R0 = mem_read(register.get(r1) + offset, memory);
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
    register.R_R0 = register.R_PC + pc_offset;
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
    memory[(register.R_PC + pc_offset) as usize] = register.get(r0);
}

// STI {
//     uint16_t r0 = (instr >> 9) & 0x7;
//     uint16_t pc_offset = sign_extend(instr & 0x1FF, 9);
//     mem_write(mem_read(reg[R_PC] + pc_offset), reg[r0]);
// }

fn op_sti(register: &mut Register, instr: u16, memory: &mut Memory) {
    let r0 = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1FF, 9);
    memory[mem_read(register.R_PC + pc_offset, memory) as usize] = register.get(r0);
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

// handle trap reg[R_R7] = reg[R_PC];

// switch (instr & 0xFF)
// {
//     case TRAP_GETC:
//         @{TRAP GETC}
//         break;
//     case TRAP_OUT:
//         @{TRAP OUT}
//         break;
//     case TRAP_PUTS:
//         @{TRAP PUTS}
//         break;
//     case TRAP_IN:
//         @{TRAP IN}
//         break;
//     case TRAP_PUTSP:
//         @{TRAP PUTSP}
//         break;
//     case TRAP_HALT:
//         @{TRAP HALT}
//         break;
// }

fn handle_trap(register: &mut Register, instr: u16, memory: &mut Memory) {
    let trap_instr = instr & 0xFF;
    match trap_instr {
        0x20 => {
            // @{TRAP GETC}
        }
        0x21 => {
            // @{TRAP OUT}
        }
        0x22 => {
            // @{TRAP PUTS}
        }
        0x23 => {
            // @{TRAP IN}
        }
        0x24 => {
            // @{TRAP PUTSP}
        }
    }
}

pub fn handle_operations( register: &mut Register, instr: u16, op: u16, memory: &mut Memory) {
    match op {
        Opcode::OP_ADD => {
            // @{ADD}
            op_add(&mut register, instr);
        }
        Opcode::OP_AND => {
            // @{AND}
            op_and(&mut register, instr);
        }
        Opcode::OP_NOT => {
            // @{NOT}
            op_not(&mut register, instr)
        }
        Opcode::OP_BR => {
            // @{BR}
            op_br(&mut register, instr);
        }
        Opcode::OP_JMP => {
            // @{JMP}
            op_jmp(&mut register, instr);
        }
        Opcode::OP_JSR => {
            // @{JSR}
            op_jsr(&mut register, instr);
        }
        Opcode::OP_LD => {
            // @{LD}
            op_ld(&mut register, instr, &memory);
        }
        Opcode::OP_LDI => {
            // @{LDI}
            op_ldi(&mut register, instr, &memory);
        }
        Opcode::OP_LDR => {
            // @{LDR}
            op_ldr(register, instr, memory)
        }
        Opcode::OP_LEA => {
            // @{LEA}
            op_lea(register, instr);
        }
        Opcode::OP_ST => {
            // @{ST}
            op_st(register, instr, memory);
        }
        Opcode::OP_STI => {
            // @{STI}
            op_sti(register, instr, memory);
        }
        Opcode::OP_STR => {
            // @{STR}
            op_str(register, instr, memory);
        }
        Opcode::OP_TRAP => {
            // @{TRAP}
            handle_trap(register, instr, memory);
        }
        _ => {
            // @{BAD OPCODE}
        }
    }
}
