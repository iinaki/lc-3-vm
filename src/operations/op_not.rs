use crate::register::Register;

use super::update_flags;

// NOT{
//     uint16_t r0 = (instr >> 9) & 0x7;
//     uint16_t r1 = (instr >> 6) & 0x7;
//     reg[r0] = ~reg[r1];
//     update_flags(r0);
// }

pub fn op_not(register: &mut Register, instr: u16) {
    let r0 = (instr >> 9) & 0x7;
    let r1 = (instr >> 6) & 0x7;
    register.set(r0, !register.get(r1));
    update_flags(register, r0);
}
