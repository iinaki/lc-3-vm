use crate::registers::Registers;

use super::{sign_extend, update_flags};

pub fn op_add(registers: &mut Registers, instr: u16) {
    let r0 = (instr >> 9) & 0x7;
    let r1 = (instr >> 6) & 0x7;
    let imm_flag = (instr >> 5) & 0x1;

    if imm_flag == 1 {
        let imm5 = sign_extend(instr & 0x1F, 5);
        registers.set(r0, registers.get(r1).wrapping_add(imm5 as u16));
    } else {
        let r2 = instr & 0x7;
        registers.set(r0, registers.get(r1).wrapping_add(registers.get(r2)));
    }

    update_flags(registers, r0);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        constants::{FL_NEG, FL_POS, FL_ZRO},
        registers::Registers,
    };

    // ADD TESTS
    #[test]
    fn op_add_with_registers() {
        let mut registers = Registers::new();
        registers.set(1, 10);
        registers.set(2, 15);

        let instr: u16 = 0b0001_0000_0100_0010;
        op_add(&mut registers, instr);
        println!(
            "registers
        : {:?}",
            registers
        );

        assert_eq!(registers.get(0), 25);
    }

    #[test]
    fn op_add_with_immediate_positive() {
        let mut registers = Registers::new();
        registers.set(1, 10);

        let instr: u16 = 0b0001_0000_0110_0001;
        op_add(&mut registers, instr);

        assert_eq!(registers.get(0), 11);
    }

    #[test]
    fn op_add_with_immediate_negative() {
        let mut registers = Registers::new();
        registers.set(1, 10);

        let instr: u16 = 0b0001_0000_0111_1111;
        op_add(&mut registers, instr);

        assert_eq!(registers.get(0), 9);
    }

    #[test]
    fn op_add_with_negative_result() {
        let mut registers = Registers::new();
        registers.set(1, 0);

        let instr: u16 = 0b0001_0000_0111_1111;
        op_add(&mut registers, instr);

        assert_eq!(registers.get(0), 0xFFFF);
        assert_eq!(registers.cond, FL_NEG);
    }

    #[test]
    fn op_add_with_zero_result() {
        let mut registers = Registers::new();
        registers.set(1, 1);

        let instr: u16 = 0b0001_0000_0111_1111;
        op_add(&mut registers, instr);

        assert_eq!(registers.get(0), 0);
        assert_eq!(registers.cond, FL_ZRO);
    }

    #[test]
    fn op_add_with_positive_result() {
        let mut registers = Registers::new();
        registers.set(1, 1);

        let instr: u16 = 0b0001_0000_0110_0001;
        op_add(&mut registers, instr);

        assert_eq!(registers.get(0), 2);
        assert_eq!(registers.cond, FL_POS);
    }
}
