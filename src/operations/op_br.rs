use crate::registers::Registers;

use super::sign_extend;

pub fn op_br(registers: &mut Registers, instr: u16) {
    let pc_offset = sign_extend(instr & 0x1FF, 9);
    let cond_flag = (instr >> 9) & 0x7;
    if cond_flag & registers.cond != 0 {
        registers.pc = ((registers.pc as i16) + pc_offset) as u16;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{constants::FL_POS, registers::Registers};

    // BR TESTS
    #[test]
    fn br_branch_taken_positive_offset() {
        let mut registers = Registers::new();
        registers.cond = FL_POS;

        let instr: u16 = 0b0000_001_000000101;
        op_br(&mut registers, instr);

        assert_eq!(registers.pc, 0x3005);
    }

    #[test]
    fn br_branch_not_taken() {
        let mut registers = Registers::new();
        registers.cond = FL_POS;

        let instr: u16 = 0b0000_010_000000101;
        op_br(&mut registers, instr);

        assert_eq!(registers.pc, 0x3000);
    }

    #[test]
    fn br_branch_taken_negative_offset() {
        let mut registers = Registers::new();
        registers.cond = FL_POS;

        let instr: u16 = 0b0000_001_111111011;
        op_br(&mut registers, instr);

        assert_eq!(registers.pc, 0x2FFB);
    }

    #[test]
    fn br_branch_zero_offset() {
        let mut registers = Registers::new();
        registers.cond = FL_POS;

        let instr: u16 = 0b0000_001_000000000;
        op_br(&mut registers, instr);

        println!("PC: {}", registers.pc);

        assert_eq!(registers.pc, 0x3000);
    }
}
