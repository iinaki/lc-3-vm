use crate::{
    registers::Registers,
    utils::{sign_extend, update_flags},
    vm_error::VmError,
};

/// Executes the LEA operation.
///
/// Computes an address by adding a sign-extended offset to the current value
/// of the program counter (PC). This computed address is then stored in the destination
/// register. The condition flags are updated based on the computed address.
///
/// # Parameters
///
/// - `registers`: A mutable reference to the `Registers` struct.
/// - `instr`: A 16-bit instruction.
///
pub fn op_lea(registers: &mut Registers, instr: u16) -> Result<(), VmError> {
    let r0 = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1FF, 9);
    registers.set(r0, (registers.pc as i16 + pc_offset) as u16)?;
    update_flags(registers, r0)
}

#[cfg(test)]
mod tests {
    use crate::constants::FL_ZRO;

    use super::*;

    #[test]
    fn op_lea_positive_offset() {
        let mut registers = Registers::new();

        let instr: u16 = 0b1110_0000_0000_0101; // LEA R0, PC + 5
        op_lea(&mut registers, instr).unwrap();

        assert_eq!(registers.get(0).unwrap(), 0x3005);
    }

    #[test]
    fn op_lea_negative_offset() {
        let mut registers = Registers::new();

        let instr: u16 = 0b1110_0001_1111_1011; // LEA R0, PC - 5
        op_lea(&mut registers, instr).unwrap();

        assert_eq!(registers.get(0).unwrap(), 0x2FFB);
    }

    #[test]
    fn op_lea_zero_offset() {
        let mut registers = Registers::new();

        let instr: u16 = 0b1110_0000_0000_0000; // LEA R0, PC + 0
        op_lea(&mut registers, instr).unwrap();

        assert_eq!(registers.get(0).unwrap(), 0x3000);
    }

    #[test]
    fn op_lea_update_flags() {
        let mut registers = Registers::new();
        registers.pc = 0x0000;

        let instr: u16 = 0b1110_0000_0000_0000; // LEA R0, PC + 0
        op_lea(&mut registers, instr).unwrap();

        assert_eq!(registers.get(0).unwrap(), 0x0000);
        assert_eq!(registers.cond, FL_ZRO);
    }

    #[test]
    fn op_lea_preserves_pc() {
        let mut registers = Registers::new();

        let instr: u16 = 0b1110_0000_0000_0101; // LEA R0, PC + 5
        let initial_pc = registers.pc;
        op_lea(&mut registers, instr).unwrap();

        assert_eq!(registers.pc, initial_pc);
    }
}
