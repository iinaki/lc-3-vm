use crate::{memory::Memory, registers::Registers, utils::sign_extend, vm_error::VmError};

/// Executes the STR operation.
///
/// Stores the value from a specified register into memory.
/// The target memory address is calculated by adding a signed offset to the value from another register.
///
/// # Parameters
///
/// - `registers`: A mutable reference to the `Registers` struct.
/// - `instr`: A 16-bit instruction.
///
pub fn op_str(registers: &mut Registers, instr: u16, memory: &mut Memory) -> Result<(), VmError> {
    let r0 = (instr >> 9) & 0x7;
    let r1 = (instr >> 6) & 0x7;
    let offset = sign_extend(instr & 0x3F, 6);
    memory.write(
        (registers.get(r1)? as i16 + offset) as u16,
        registers.get(r0)?,
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn op_str_basic() {
        let mut registers = Registers::new();
        let mut memory = Memory::new();

        registers.set(0, 0xABCD).unwrap();
        registers.set(1, 0x3000).unwrap();

        let instr: u16 = 0b0111_0000_0100_0010; // STR R0, R1, #2
        op_str(&mut registers, instr, &mut memory).unwrap();

        assert_eq!(memory.read(0x3002).unwrap(), 0xABCD);
    }

    #[test]
    fn op_str_negative_offset() {
        let mut registers = Registers::new();
        let mut memory = Memory::new();

        registers.set(0, 0x1234).unwrap();
        registers.set(1, 0x3004).unwrap();

        let instr: u16 = 0b0111_0000_0111_1110; // STR R0, R1, #-2
        op_str(&mut registers, instr, &mut memory).unwrap();

        assert_eq!(memory.read(0x3002).unwrap(), 0x1234);
    }

    #[test]
    fn op_str_zero_offset() {
        let mut registers = Registers::new();
        let mut memory = Memory::new();

        registers.set(0, 0x5678).unwrap();
        registers.set(1, 0x4000).unwrap();

        let instr: u16 = 0b0111_0000_0100_0000; // STR R0, R1, #0
        op_str(&mut registers, instr, &mut memory).unwrap();

        assert_eq!(memory.read(0x4000).unwrap(), 0x5678);
    }

    #[test]
    fn op_str_large_offset() {
        let mut registers = Registers::new();
        let mut memory = Memory::new();

        registers.set(0, 0x9ABC).unwrap();
        registers.set(1, 0x1000).unwrap();

        let instr: u16 = 0b0111_0000_0100_1111; // STR R0, R1, #15
        op_str(&mut registers, instr, &mut memory).unwrap();

        assert_eq!(memory.read(0x100F).unwrap(), 0x9ABC);
    }

    #[test]
    fn op_str_overflow_address() {
        let mut registers = Registers::new();
        let mut memory = Memory::new();

        registers.set(0, 0x4321).unwrap();
        registers.set(1, 0xFFFF).unwrap();

        let instr: u16 = 0b0111_0000_0100_0001; // STR R0, R1, #1
        op_str(&mut registers, instr, &mut memory).unwrap();

        assert_eq!(memory.read(0x0000).unwrap(), 0x4321);
    }
}
