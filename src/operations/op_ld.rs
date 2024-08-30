use crate::{
    memory::Memory,
    register::Register,
};

use super::{sign_extend, update_flags};

// LD {
//     uint16_t r0 = (instr >> 9) & 0x7;
//     uint16_t pc_offset = sign_extend(instr & 0x1FF, 9);
//     reg[r0] = mem_read(reg[R_PC] + pc_offset);
//     update_flags(r0);
// }

pub fn op_ld(register: &mut Register, instr: u16, memory: &mut Memory) {
    let r0 = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1FF, 9);
    register.set(r0, memory.read(r0 + pc_offset));
    update_flags(register, r0);
}
