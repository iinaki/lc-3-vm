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
    register.set(r0, (register.pc as i16 + pc_offset) as u16);
    update_flags(register, r0);
}

#[cfg(test)]
mod tests {
    use crate::constants::FL_ZRO;

    use super::*;

    #[test]
    fn test_op_lea_positive_offset() {
        let mut register = Register::new();

        let instr: u16 = 0b1110_000_000000101; // LEA R0, PC + 5
        op_lea(&mut register, instr);

        assert_eq!(register.get(0), 0x3005);
    }

    #[test]
    fn test_op_lea_negative_offset() {
        let mut register = Register::new();

        let instr: u16 = 0b1110_000_111111011; // LEA R0, PC - 5
        op_lea(&mut register, instr);

        assert_eq!(register.get(0), 0x2FFB);
    }

    #[test]
    fn test_op_lea_zero_offset() {
        let mut register = Register::new();

        let instr: u16 = 0b1110_000_000000000; // LEA R0, PC + 0
        op_lea(&mut register, instr);

        assert_eq!(register.get(0), 0x3000);
    }

    #[test]
    fn test_op_lea_update_flags() {
        let mut register = Register::new();
        register.pc = 0x0000;

        let instr: u16 = 0b1110_000_000000000; // LEA R0, PC + 0
        op_lea(&mut register, instr);

        assert_eq!(register.get(0), 0x0000);
        assert_eq!(register.cond, FL_ZRO);
    }

    #[test]
    fn test_op_lea_preserves_pc() {
        let mut register = Register::new();

        let instr: u16 = 0b1110_000_000000101; // LEA R0, PC + 5
        let initial_pc = register.pc;
        op_lea(&mut register, instr);

        assert_eq!(register.pc, initial_pc);
    }
}
