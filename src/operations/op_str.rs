use crate::{memory::Memory, register::Register};

use super::sign_extend;

// STR {
//     uint16_t r0 = (instr >> 9) & 0x7;
//     uint16_t r1 = (instr >> 6) & 0x7;
//     uint16_t offset = sign_extend(instr & 0x3F, 6);
//     mem_write(reg[r1] + offset, reg[r0]);
// }

pub fn op_str(register: &mut Register, instr: u16, memory: &mut Memory) {
    let r0 = (instr >> 9) & 0x7;
    let r1 = (instr >> 6) & 0x7;
    let offset = sign_extend(instr & 0x3F, 6);
    memory.write(register.get(r1) + offset, register.get(r0));
}
