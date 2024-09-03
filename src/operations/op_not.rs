use crate::{registers::Registers, utils::update_flags};

pub fn op_not(registers: &mut Registers, instr: u16) {
    let r0 = (instr >> 9) & 0x7;
    let r1 = (instr >> 6) & 0x7;
    registers.set(r0, !registers.get(r1));
    update_flags(registers, r0);
}

#[cfg(test)]
mod tests {
    use crate::constants::{FL_NEG, FL_ZRO};

    use super::*;

    #[test]
    fn op_not_basic() {
        let mut registers = Registers::new();
        registers.set(1, 0x0F0F);

        let instr: u16 = 0b1001_0000_0111_1111; // NOT R0, R1
        op_not(&mut registers, instr);

        assert_eq!(registers.get(0), 0xF0F0);
    }

    #[test]
    fn op_not_zero() {
        let mut registers = Registers::new();
        registers.set(1, 0x0000);

        let instr: u16 = 0b1001_0000_0111_1111; // NOT R0, R1
        op_not(&mut registers, instr);

        assert_eq!(registers.get(0), 0xFFFF);
    }

    #[test]
    fn op_not_all_ones() {
        let mut registers = Registers::new();
        registers.set(1, 0xFFFF);

        let instr: u16 = 0b1001_0000_0111_1111; // NOT R0, R1
        op_not(&mut registers, instr);

        assert_eq!(registers.get(0), 0x0000);
        assert_eq!(registers.cond, FL_ZRO);
    }

    #[test]
    fn op_not_update_flags_negative() {
        let mut registers = Registers::new();
        registers.set(1, 0x7FFF);

        let instr: u16 = 0b1001_0000_0111_1111; // NOT R0, R1
        op_not(&mut registers, instr);

        assert_eq!(registers.get(0), 0x8000);
        assert_eq!(registers.cond, FL_NEG);
    }
}
