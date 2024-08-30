use crate::register::Register;

use super::sign_extend;

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

pub fn op_jsr(register: &mut Register, instr: u16) {
    let long_flag = (instr >> 11) & 1;
    register.r7 = register.pc;
    if long_flag == 1 {
        let long_pc_offset = sign_extend(instr & 0x7FF, 11);
        register.pc += long_pc_offset; /* JSR */
    } else {
        let r1 = (instr >> 6) & 0x7;
        register.pc = register.get(r1); /* JSRR */
    }
}
