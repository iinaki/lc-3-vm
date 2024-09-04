use crate::registers::Registers;

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
pub fn op_jmp(registers: &mut Registers, instr: u16) {
    let r1 = (instr >> 6) & 0x7;
    registers.pc = registers.get(r1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn op_jmp_changes_pc() {
        let mut registers = Registers::new();
        registers.set(1, 0x3033);

        let instr: u16 = 0b1100_0000_0100_0000;
        op_jmp(&mut registers, instr);

        assert_eq!(registers.pc, 0x3033);
    }

    #[test]
    fn op_jmp_to_zero() {
        let mut registers = Registers::new();
        registers.set(2, 0x0000);

        let instr: u16 = 0b1100_0000_1000_0000;
        op_jmp(&mut registers, instr);

        assert_eq!(registers.pc, 0x0000);
    }

    #[test]
    fn op_jmp_to_high_address() {
        let mut registers = Registers::new();
        registers.set(3, 0xFFFF);

        let instr: u16 = 0b1100_0000_1100_0000;
        op_jmp(&mut registers, instr);

        assert_eq!(registers.pc, 0xFFFF);
    }
}
