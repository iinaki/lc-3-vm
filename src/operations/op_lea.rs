use crate::register::Register;

use super::{sign_extend, update_flags};

// LEA {
//     uint16_t r0 = (instr >> 9) & 0x7;
//     uint16_t pc_offset = sign_extend(instr & 0x1FF, 9);
//     reg[r0] = reg[R_PC] + pc_offset;
//     update_flags(r0);
// }

pub fn op_lea(register: &mut Register, instr: u16) {
    let r0 = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1FF, 9);
    register.set(r0, register.pc + pc_offset);
    update_flags(register, r0);
}
