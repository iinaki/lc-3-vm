use crate::{registers::Registers, vm_error::VmError};

/// Executes the JUMP operation.
///
/// This opcode updates the program counter to the address stored in the specified
/// register, allowing for an unconditional jump to a new location in the program.
///
/// # Parameters
///
/// - `registers`: A mutable reference to the `Registers` struct.
/// - `instr`: A 16-bit instruction.
///
pub fn op_jmp(registers: &mut Registers, instr: u16) -> Result<(), VmError> {
    let r1 = (instr >> 6) & 0x7;
    registers.pc = registers.get(r1)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn op_jmp_changes_pc() {
        let mut registers = Registers::new();
        registers.set(1, 0x3033).unwrap();

        let instr: u16 = 0b1100_0000_0100_0000;
        op_jmp(&mut registers, instr).unwrap();

        assert_eq!(registers.pc, 0x3033);
    }

    #[test]
    fn op_jmp_to_zero() {
        let mut registers = Registers::new();
        registers.set(2, 0x0000).unwrap();

        let instr: u16 = 0b1100_0000_1000_0000;
        op_jmp(&mut registers, instr).unwrap();

        assert_eq!(registers.pc, 0x0000);
    }

    #[test]
    fn op_jmp_to_high_address() {
        let mut registers = Registers::new();
        registers.set(3, 0xFFFF).unwrap();

        let instr: u16 = 0b1100_0000_1100_0000;
        op_jmp(&mut registers, instr).unwrap();

        assert_eq!(registers.pc, 0xFFFF);
    }
}
