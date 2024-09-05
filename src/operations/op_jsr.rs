use crate::{registers::Registers, utils::sign_extend, vm_error::VmError};

/// Executes the JSR operation.
///
/// Performs a jump to a subroutine. It saves the current program counter
/// in the R7 register and then updates the PC to the target address, which can be provided
/// either by a direct offset (JSR) or by a register (JSRR).
///
/// # Parameters
///
/// - `registers`: A mutable reference to the `Registers` struct.
/// - `instr`: A 16-bit instruction.
///
/// # Returns
///
/// Returns `Ok(())` if the operation was successful, otherwise returns a `VmError`.
///
pub fn op_jsr(registers: &mut Registers, instr: u16) -> Result<(), VmError> {
    let long_flag = (instr >> 11) & 1;
    registers.r7 = registers.pc;
    if long_flag == 1 {
        let long_pc_offset = sign_extend(instr & 0x7FF, 11);
        registers.pc = (registers.pc as i16 + long_pc_offset) as u16; /* JSR */
    } else {
        let r1 = (instr >> 6) & 0x7;
        registers.pc = registers.get(r1)?; /* JSRR */
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn op_jsr_long_offset() {
        let mut registers = Registers::new();
        registers.pc = 0x3000;

        let instr: u16 = 0b0100_1000_0001_0000;
        op_jsr(&mut registers, instr).unwrap();

        assert_eq!(registers.r7, 0x3000);
        assert_eq!(registers.pc, 0x3010);
    }

    #[test]
    fn op_jsr_negative_offset() {
        let mut registers = Registers::new();
        registers.pc = 0x3000;

        let instr: u16 = 0b0100_1111_1111_1111;
        op_jsr(&mut registers, instr).unwrap();

        assert_eq!(registers.r7, 0x3000);
        assert_eq!(registers.pc, 0x2FFF);
    }

    #[test]
    fn op_jsrr() {
        let mut registers = Registers::new();
        registers.pc = 0x3000;
        registers.set(2, 0x4000).unwrap();

        let instr: u16 = 0b0_1000_0000_1000_0000;
        op_jsr(&mut registers, instr).unwrap();

        assert_eq!(registers.r7, 0x3000);
        assert_eq!(registers.pc, 0x4000);
    }

    #[test]
    fn op_jsr_preserves_other_registers() {
        let mut registers = Registers::new();
        registers.pc = 0x3000;
        registers.set(1, 0xABCD).unwrap();
        registers.set(2, 0x1234).unwrap();

        let instr: u16 = 0b0_1000_0000_1000_0000;
        op_jsr(&mut registers, instr).unwrap();

        assert_eq!(registers.r7, 0x3000);
        assert_eq!(registers.pc, 0x1234);
        assert_eq!(registers.get(1).unwrap(), 0xABCD);
    }

    #[test]
    fn op_jsr_long_offset_and_return() {
        let mut registers = Registers::new();
        registers.pc = 0x3000;

        let instr_jsr: u16 = 0b0100_1000_0000_0010;
        op_jsr(&mut registers, instr_jsr).unwrap();

        assert_eq!(registers.r7, 0x3000);
        assert_eq!(registers.pc, 0x3002);

        registers.pc = registers.r7;
        assert_eq!(registers.pc, 0x3000);
    }
}
