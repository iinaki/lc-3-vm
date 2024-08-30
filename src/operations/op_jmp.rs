use crate::register::Register;

// JMP {
//     /* Also handles RET */
//     uint16_t r1 = (instr >> 6) & 0x7;
//     reg[R_PC] = reg[r1];
// }

pub fn op_jmp(register: &mut Register, instr: u16) {
    let r1 = (instr >> 6) & 0x7;
    register.pc = register.get(r1);
}
