use crate::{memory::Memory, registers::Registers, utils::sign_extend, vm_error::VmError};

/// Executes the ST operation.
///
/// Stores the value from the specified register into memory
/// at the address computed by adding a PC-relative offset to the current program counter.
///
/// # Parameters
///
/// - `registers`: A mutable reference to the `Registers` struct.
/// - `instr`: A 16-bit instruction.
///
pub fn op_st(registers: &mut Registers, instr: u16, memory: &mut Memory) -> Result<(), VmError> {
    let r0 = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1FF, 9);
    memory.write((registers.pc as i16 + pc_offset) as u16, registers.get(r0)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn op_st_basic() {
        let mut registers = Registers::new();
        let mut memory = Memory::new();

        registers.set(0, 0x1234).unwrap();

        let instr: u16 = 0b0011_0000_0000_0010; // ST R0, #2
        op_st(&mut registers, instr, &mut memory).unwrap();

        assert_eq!(memory.read(registers.pc + 2).unwrap(), 0x1234);
    }

    #[test]
    fn op_st_negative_offset() {
        let mut registers = Registers::new();
        let mut memory = Memory::new();

        registers.pc = 0x3000;
        registers.set(0, 0x5678).unwrap();

        let instr: u16 = 0b0011_0001_1111_1110; // ST R0, #-2
        op_st(&mut registers, instr, &mut memory).unwrap();

        assert_eq!(memory.read(0x2FFE).unwrap(), 0x5678);
    }

    #[test]
    fn op_st_zero_offset() {
        let mut registers = Registers::new();
        let mut memory = Memory::new();

        registers.pc = 0x3000;
        registers.set(0, 0xABCD).unwrap();

        let instr: u16 = 0b0011_0000_0000_0000; // ST R0, #0
        op_st(&mut registers, instr, &mut memory).unwrap();

        assert_eq!(memory.read(0x3000).unwrap(), 0xABCD);
    }

    #[test]
    fn op_st_overflow_offset() {
        let mut registers = Registers::new();
        let mut memory = Memory::new();

        registers.pc = 0xFFFF;
        registers.set(0, 0x4321).unwrap();

        let instr: u16 = 0b0011_0000_0000_0001; // ST R0, #1
        op_st(&mut registers, instr, &mut memory).unwrap();

        assert_eq!(memory.read(0x0000).unwrap(), 0x4321);
    }

    #[test]
    fn op_st_preserves_registers() {
        let mut registers = Registers::new();
        let mut memory = Memory::new();

        registers.pc = 0x3000;
        registers.set(0, 0x7777).unwrap();
        registers.set(1, 0x8888).unwrap();

        let instr: u16 = 0b0011_0000_0000_0010; // ST R0, #2
        op_st(&mut registers, instr, &mut memory).unwrap();

        assert_eq!(registers.get(1).unwrap(), 0x8888);
    }
}
