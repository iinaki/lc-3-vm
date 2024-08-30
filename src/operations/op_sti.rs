use crate::{
    memory::Memory,
    register::Register,
};

use super::sign_extend;

// STI {
//     uint16_t r0 = (instr >> 9) & 0x7;
//     uint16_t pc_offset = sign_extend(instr & 0x1FF, 9);
//     mem_write(mem_read(reg[R_PC] + pc_offset), reg[r0]);
// }

pub fn op_sti(register: &mut Register, instr: u16, memory: &mut Memory) {
    let r0 = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1FF, 9);
    let addr = memory.read(r0 + pc_offset);
    memory.write(addr, register.get(r0));
}
