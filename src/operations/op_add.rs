use crate::{
    registers::Registers,
    utils::{sign_extend, update_flags},
    vm_error::VmError,
};

/// Executes the ADD operation.
///
/// The add can be between two registers or between a register and an immediate value.
/// The result is stored in a dest register, and the condition flags are updated
/// to reflect the result of the operation.
///
/// # Parameters
///
/// - `registers`: A mutable reference to the `Registers` struct.
/// - `instr`: A 16-bit instruction.
///
pub fn op_add(registers: &mut Registers, instr: u16) -> Result<(), VmError> {
    let r0 = (instr >> 9) & 0x7;
    let r1 = (instr >> 6) & 0x7;
    let imm_flag = (instr >> 5) & 0x1;

    if imm_flag == 1 {
        let imm5 = sign_extend(instr & 0x1F, 5);
        registers.set(r0, registers.get(r1)?.wrapping_add(imm5 as u16))?;
    } else {
        let r2 = instr & 0x7;
        registers.set(r0, registers.get(r1)?.wrapping_add(registers.get(r2)?))?;
    }

    update_flags(registers, r0)
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
        registers.set(1, 10).unwrap();
        registers.set(2, 15).unwrap();

        let instr: u16 = 0b0001_0000_0100_0010;
        op_add(&mut registers, instr).unwrap();
        println!(
            "registers
        : {:?}",
            registers
        );

        assert_eq!(registers.get(0).unwrap(), 25);
    }

    #[test]
    fn op_add_with_immediate_positive() {
        let mut registers = Registers::new();
        registers.set(1, 10).unwrap();

        let instr: u16 = 0b0001_0000_0110_0001;
        op_add(&mut registers, instr).unwrap();

        assert_eq!(registers.get(0).unwrap(), 11);
    }

    #[test]
    fn op_add_with_immediate_negative() {
        let mut registers = Registers::new();
        registers.set(1, 10).unwrap();

        let instr: u16 = 0b0001_0000_0111_1111;
        op_add(&mut registers, instr).unwrap();

        assert_eq!(registers.get(0).unwrap(), 9);
    }

    #[test]
    fn op_add_with_negative_result() {
        let mut registers = Registers::new();
        registers.set(1, 0).unwrap();

        let instr: u16 = 0b0001_0000_0111_1111;
        op_add(&mut registers, instr).unwrap();

        assert_eq!(registers.get(0).unwrap(), 0xFFFF);
        assert_eq!(registers.cond, FL_NEG);
    }

    #[test]
    fn op_add_with_zero_result() {
        let mut registers = Registers::new();
        registers.set(1, 1).unwrap();

        let instr: u16 = 0b0001_0000_0111_1111;
        op_add(&mut registers, instr).unwrap();

        assert_eq!(registers.get(0).unwrap(), 0);
        assert_eq!(registers.cond, FL_ZRO);
    }

    #[test]
    fn op_add_with_positive_result() {
        let mut registers = Registers::new();
        registers.set(1, 1).unwrap();

        let instr: u16 = 0b0001_0000_0110_0001;
        op_add(&mut registers, instr).unwrap();

        assert_eq!(registers.get(0).unwrap(), 2);
        assert_eq!(registers.cond, FL_POS);
    }
}
