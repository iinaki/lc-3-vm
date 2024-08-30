use crate::{memory::Memory, register::Register};

use super::sign_extend;

// ST {
//     uint16_t r0 = (instr >> 9) & 0x7;
//     uint16_t pc_offset = sign_extend(instr & 0x1FF, 9);
//     mem_write(reg[R_PC] + pc_offset, reg[r0]);
// }

pub fn op_st(register: &mut Register, instr: u16, memory: &mut Memory) {
    let r0 = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1FF, 9);
    //memory[(r0 + pc_offset) as usize] = register.get(r0);
    memory.write(r0 + pc_offset, register.get(r0));
}
