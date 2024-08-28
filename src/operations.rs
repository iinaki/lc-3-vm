use crate::{condition_flag::ConditionFlag, opcode::Opcode, register::Register};

fn mem_read(address: u16, memory: &[i32; 65536]) -> u16 {
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

// LDI {
//     /* destination register (DR) */
//     uint16_t r0 = (instr >> 9) & 0x7;
//     /* PCoffset 9*/
//     uint16_t pc_offset = sign_extend(instr & 0x1FF, 9);
//     /* add pc_offset to the current PC, look at that memory location to get the final address */
//     reg[r0] = mem_read(mem_read(reg[R_PC] + pc_offset));
//     update_flags(r0);
// }

fn op_ldi(register: &mut Register, instr: u16, memory: &[i32; 65536]) {
    let r0 = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1FF, 9);
    register.set(r0, mem_read(mem_read(register.R_PC + pc_offset, memory), memory));
    update_flags(register, r0);
}

pub fn handle_operations( register: &mut Register, instr: u16, op: u16, memory: &mut [i32; 65536]) {
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
        }
        Opcode::OP_BR => {
            // @{BR}
        }
        Opcode::OP_JMP => {
            // @{JMP}
        }
        Opcode::OP_JSR => {
            // @{JSR}
        }
        Opcode::OP_LD => {
            // @{LD}
        }
        Opcode::OP_LDI => {
            // @{LDI}
            op_ldi(&mut register, instr, &memory);
        }
        Opcode::OP_LDR => {
            // @{LDR}
        }
        Opcode::OP_LEA => {
            // @{LEA}
        }
        Opcode::OP_ST => {
            // @{ST}
        }
        Opcode::OP_STI => {
            // @{STI}
        }
        Opcode::OP_STR => {
            // @{STR}
        }
        Opcode::OP_TRAP => {
            // @{TRAP}
        }
        _ => {
            // @{BAD OPCODE}
        }
    }
}
