use crate::{
    registers::Registers,
    utils::{sign_extend, update_flags},
    vm_error::VmError,
};

/// Performs a bitwise AND operation between two operands.
///
/// The first operand is always a register, and the second
/// operand can be another register or an immediate value. The result of the AND
/// operation is stored in the dest register, and the condition
/// flags are updated based on the result.
///
/// # Parameters
///
/// - `registers`: A mutable reference to the `Registers` struct.
/// - `instr`: A 16-bit instruction.
///
pub fn op_and(registers: &mut Registers, instr: u16) -> Result<(), VmError> {
    let r0 = (instr >> 9) & 0x7;
    let r1 = (instr >> 6) & 0x7;
    let imm_flag = (instr >> 5) & 0x1;

    if imm_flag == 1 {
        let imm5 = sign_extend(instr & 0x1F, 5);
        registers.set(r0, (registers.get(r1)? as i16 & imm5) as u16)?;
    } else {
        let r2 = instr & 0x7;
        registers.set(r0, registers.get(r1)? & registers.get(r2)?)?;
    }
    update_flags(registers, r0)
}

#[cfg(test)]
mod tests {
    use crate::constants::{FL_NEG, FL_POS, FL_ZRO};

    use super::*;

    #[test]
    fn op_and_with_registers() {
        let mut registers = Registers::new();
        registers.set(1, 0b1100).unwrap();
        registers.set(2, 0b1010).unwrap();

        let instr: u16 = 0b0101_0000_0100_0010;
        op_and(&mut registers, instr).unwrap();

        assert_eq!(registers.get(0).unwrap(), 0b1000);
    }

    #[test]
    fn op_and_with_immediate() {
        let mut registers = Registers::new();
        registers.set(1, 0b1100).unwrap();

        let instr: u16 = 0b0101_0000_0110_0101;
        op_and(&mut registers, instr).unwrap();

        assert_eq!(registers.get(0).unwrap(), 0b0100);
    }

    #[test]
    fn op_and_with_zero_result() {
        let mut registers = Registers::new();
        registers.set(1, 0b1100).unwrap();

        let instr: u16 = 0b0101_0000_0110_0110;
        op_and(&mut registers, instr).unwrap();

        assert_eq!(registers.get(0).unwrap(), 0b0100);
        assert_eq!(registers.cond, FL_POS);
    }

    #[test]
    fn op_and_with_negative_result() {
        let mut registers = Registers::new();
        registers.set(1, 0xFFFF).unwrap();

        let instr: u16 = 0b0101_0000_0111_1111;
        op_and(&mut registers, instr).unwrap();

        assert_eq!(registers.get(0).unwrap(), 0xFFFF);
        assert_eq!(registers.cond, FL_NEG);
    }

    #[test]
    fn op_and_with_positive_result() {
        let mut registers = Registers::new();
        registers.set(1, 0b0111).unwrap();
        registers.set(2, 0b0011).unwrap();

        let instr: u16 = 0b0101_0000_0100_0010;
        op_and(&mut registers, instr).unwrap();

        assert_eq!(registers.get(0).unwrap(), 0b0011);
        assert_eq!(registers.cond, FL_POS);
    }

    #[test]
    fn op_and_with_zero_cond_flag() {
        let mut registers = Registers::new();
        registers.set(1, 0b0101).unwrap();
        registers.set(2, 0b1010).unwrap();

        let instr: u16 = 0b0101_0000_0100_0010;
        op_and(&mut registers, instr).unwrap();

        assert_eq!(registers.get(0).unwrap(), 0b0000);
        assert_eq!(registers.cond, FL_ZRO);
    }
}
