use crate::{
    memory::Memory,
    register::Register,
};

use super::{sign_extend, update_flags};

// LDI {
//     /* destination register (DR) */
//     uint16_t r0 = (instr >> 9) & 0x7;
//     /* PCoffset 9*/
//     uint16_t pc_offset = sign_extend(instr & 0x1FF, 9);
//     /* add pc_offset to the current PC, look at that memory location to get the final address */
//     reg[r0] = mem_read(mem_read(reg[R_PC] + pc_offset));
//     update_flags(r0);
// }

pub fn op_ldi(register: &mut Register, instr: u16, memory: &mut Memory) {
    let r0 = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1FF, 9);
    let addr = memory.read(r0 + pc_offset);

    register.set(r0, memory.read(addr));
    update_flags(register, r0);
}
