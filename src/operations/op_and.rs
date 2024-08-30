use crate::register::Register;

use super::{sign_extend, update_flags};

// AND {
//     uint16_t r0 = (instr >> 9) & 0x7;
//     uint16_t r1 = (instr >> 6) & 0x7;
//     uint16_t imm_flag = (instr >> 5) & 0x1;

//     if (imm_flag)
//     {
//         uint16_t imm5 = sign_extend(instr & 0x1F, 5);
//         reg[r0] = reg[r1] & imm5;
//     }
//     else
//     {
//         uint16_t r2 = instr & 0x7;
//         reg[r0] = reg[r1] & reg[r2];
//     }
//     update_flags(r0);
// }
pub fn op_and(register: &mut Register, instr: u16) {
    let r0 = (instr >> 9) & 0x7;
    let r1 = (instr >> 6) & 0x7;
    let imm_flag = (instr >> 5) & 0x1;

    if imm_flag == 1 {
        let imm5 = sign_extend(instr & 0x1F, 5);
        register.set(r0, register.get(r1) & imm5);
    } else {
        let r2 = instr & 0x7;
        register.set(r0, register.get(r1) & register.get(r2));
    }
    update_flags(register, r0);
}

#[cfg(test)]
mod tests {
    use crate::constants::{FL_NEG, FL_POS, FL_ZRO};

    use super::*;

    #[test]
    fn test_op_and_with_registers() {
        let mut register = Register::new();
        register.set(1, 0b1100);
        register.set(2, 0b1010);

        let instr: u16 = 0b0101_000_001_000_010;
        op_and(&mut register, instr);

        assert_eq!(register.get(0), 0b1000);
    }

    #[test]
    fn test_op_and_with_immediate() {
        let mut register = Register::new();
        register.set(1, 0b1100);

        let instr: u16 = 0b0101_000_001_1_00101;
        op_and(&mut register, instr);

        assert_eq!(register.get(0), 0b0100);
    }

    #[test]
    fn test_op_and_with_zero_result() {
        let mut register = Register::new();
        register.set(1, 0b1100);

        let instr: u16 = 0b0101_000_001_1_00110;
        op_and(&mut register, instr);

        assert_eq!(register.get(0), 0b0100);
        assert_eq!(register.cond, FL_POS);
    }

    #[test]
    fn test_op_and_with_negative_result() {
        let mut register = Register::new();
        register.set(1, 0xFFFF);

        let instr: u16 = 0b0101_000_001_1_11111;
        op_and(&mut register, instr);

        assert_eq!(register.get(0), 0xFFFF);
        assert_eq!(register.cond, FL_NEG);
    }

    #[test]
    fn test_op_and_with_positive_result() {
        let mut register = Register::new();
        register.set(1, 0b0111);
        register.set(2, 0b0011);

        let instr: u16 = 0b0101_000_001_000_010;
        op_and(&mut register, instr);

        assert_eq!(register.get(0), 0b0011);
        assert_eq!(register.cond, FL_POS);
    }

    #[test]
    fn test_op_and_with_zero_cond_flag() {
        let mut register = Register::new();
        register.set(1, 0b0101);
        register.set(2, 0b1010);

        let instr: u16 = 0b0101_000_001_000_010;
        op_and(&mut register, instr);

        assert_eq!(register.get(0), 0b0000);
        assert_eq!(register.cond, FL_ZRO);
    }
}
