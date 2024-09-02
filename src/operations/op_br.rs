use crate::register::Register;

use super::sign_extend;

// BR {
//     uint16_t pc_offset = sign_extend(instr & 0x1FF, 9);
//     uint16_t cond_flag = (instr >> 9) & 0x7;
//     if (cond_flag & reg[R_COND])
//     {
//         reg[R_PC] += pc_offset;
//     }
// }

// fn op_br(register: &mut Register, instr: u16) {
//     let pc_offset = sign_extend(instr & 0x1FF, 9);
//     let cond_flag = (instr >> 9) & 0x7;
//     if cond_flag & register.cond as u16 != 0 {
//         register.pc += pc_offset;
//     }
// }
pub fn op_br(register: &mut Register, instr: u16) {
    let pc_offset = sign_extend(instr & 0x1FF, 9);
    let cond_flag = (instr >> 9) & 0x7;
    if cond_flag & register.cond != 0 {
        register.pc = ((register.pc as i16) + pc_offset) as u16;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{constants::FL_POS, register::Register};

    // BR TESTS
    #[test]
    fn br_branch_taken_positive_offset() {
        let mut register = Register::new();
        register.cond = FL_POS;

        let instr: u16 = 0b0000_001_000000101;
        op_br(&mut register, instr);

        assert_eq!(register.pc, 0x3005);
    }

    #[test]
    fn br_branch_not_taken() {
        let mut register = Register::new();
        register.cond = FL_POS;

        let instr: u16 = 0b0000_010_000000101;
        op_br(&mut register, instr);

        assert_eq!(register.pc, 0x3000);
    }

    #[test]
    fn br_branch_taken_negative_offset() {
        let mut register = Register::new();
        register.cond = FL_POS;

        let instr: u16 = 0b0000_001_111111011;
        op_br(&mut register, instr);

        assert_eq!(register.pc, 0x2FFB);
    }

    #[test]
    fn br_branch_zero_offset() {
        let mut register = Register::new();
        register.cond = FL_POS;

        let instr: u16 = 0b0000_001_000000000;
        op_br(&mut register, instr);

        println!("PC: {}", register.pc);

        assert_eq!(register.pc, 0x3000);
    }
}
