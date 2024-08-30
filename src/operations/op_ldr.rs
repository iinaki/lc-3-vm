use crate::{
    memory::Memory,
    register::Register,
};

use super::{sign_extend, update_flags};

// LDR {
//     uint16_t r0 = (instr >> 9) & 0x7;
//     uint16_t r1 = (instr >> 6) & 0x7;
//     uint16_t offset = sign_extend(instr & 0x3F, 6);
//     reg[r0] = mem_read(reg[r1] + offset);
//     update_flags(r0);
// }

pub fn op_ldr(register: &mut Register, instr: u16, memory: &mut Memory) {
    let r0 = (instr >> 9) & 0x7;
    let r1 = (instr >> 6) & 0x7;
    let offset = sign_extend(instr & 0x3F, 6);
    register.set(r0, memory.read(register.get(r1) + offset));
    update_flags(register, r0);
}
