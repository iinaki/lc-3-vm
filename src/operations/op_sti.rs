use crate::{memory::Memory, registers::Registers, utils::sign_extend};

/// Executes the STI operation.
/// 
/// Stores the value from the specified register into memory at an address determined indirectly. 
/// The address is first calculated using a PC-relative offset, and the value from the register is stored at the memory address retrieved from that location.
///
/// # Parameters
///
/// - `registers`: A mutable reference to the `Registers` struct.
/// - `instr`: A 16-bit instruction.
///
pub fn op_sti(registers: &mut Registers, instr: u16, memory: &mut Memory) {
    let r0 = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1FF, 9);
    let addr = memory.read((registers.pc as i16 + pc_offset) as u16);
    memory.write(addr, registers.get(r0));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn op_sti_basic() {
        let mut registers = Registers::new();
        let mut memory = Memory::new();

        registers.pc = 0x3000;
        registers.set(0, 0x1234);

        memory.write(0x3002, 0x4000);

        let instr: u16 = 0b1011_0000_0000_0010; // STI R0, #2
        op_sti(&mut registers, instr, &mut memory);

        assert_eq!(memory.read(0x4000), 0x1234);
    }

    #[test]
    fn op_sti_negative_offset() {
        let mut registers = Registers::new();
        let mut memory = Memory::new();

        registers.pc = 0x3000;
        registers.set(0, 0x5678);

        memory.write(0x2FFE, 0x5000);

        let instr: u16 = 0b1011_0001_1111_1110; // STI R0, #-2
        op_sti(&mut registers, instr, &mut memory);

        assert_eq!(memory.read(0x5000), 0x5678);
    }

    #[test]
    fn op_sti_zero_offset() {
        let mut registers = Registers::new();
        let mut memory = Memory::new();

        registers.pc = 0x3000;
        registers.set(0, 0xABCD);

        memory.write(0x3000, 0x6000);

        let instr: u16 = 0b1011_0000_0000_0000; // STI R0, #0
        op_sti(&mut registers, instr, &mut memory);

        assert_eq!(memory.read(0x6000), 0xABCD);
    }

    #[test]
    fn op_sti_overflow_offset() {
        let mut registers = Registers::new();
        let mut memory = Memory::new();

        registers.pc = 0xFFFF;
        registers.set(0, 0x4321);

        memory.write(0x0000, 0x7000);

        let instr: u16 = 0b1011_0000_0000_0001; // STI R0, #1
        op_sti(&mut registers, instr, &mut memory);

        assert_eq!(memory.read(0x7000), 0x4321);
    }

    #[test]
    fn op_sti_preserves_registerss() {
        let mut registers = Registers::new();
        let mut memory = Memory::new();

        registers.pc = 0x3000;
        registers.set(0, 0x7777);
        registers.set(1, 0x8888);

        memory.write(0x3002, 0x8000);

        let instr: u16 = 0b1011_0000_0000_0010; // STI R0, #2
        op_sti(&mut registers, instr, &mut memory);

        assert_eq!(registers.get(1), 0x8888);
    }
}
