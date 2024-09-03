use crate::{memory::Memory, registers::Registers};

use super::sign_extend;

pub fn op_st(registers: &mut Registers, instr: u16, memory: &mut Memory) {
    let r0 = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1FF, 9);
    memory.write((registers.pc as i16 + pc_offset) as u16, registers.get(r0));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn op_st_basic() {
        let mut registers = Registers::new();
        let mut memory = Memory::new();

        registers.set(0, 0x1234);

        let instr: u16 = 0b0011_000_000_000010; // ST R0, #2
        op_st(&mut registers, instr, &mut memory);

        assert_eq!(memory.read(registers.pc + 2), 0x1234);
    }

    #[test]
    fn op_st_negative_offset() {
        let mut registers = Registers::new();
        let mut memory = Memory::new();

        registers.pc = 0x3000;
        registers.set(0, 0x5678);

        let instr: u16 = 0b0011_000_111_111110; // ST R0, #-2
        op_st(&mut registers, instr, &mut memory);

        assert_eq!(memory.read(0x2FFE), 0x5678);
    }

    #[test]
    fn op_st_zero_offset() {
        let mut registers = Registers::new();
        let mut memory = Memory::new();

        registers.pc = 0x3000;
        registers.set(0, 0xABCD);

        let instr: u16 = 0b0011_000_000_000000; // ST R0, #0
        op_st(&mut registers, instr, &mut memory);

        assert_eq!(memory.read(0x3000), 0xABCD);
    }

    #[test]
    fn op_st_overflow_offset() {
        let mut registers = Registers::new();
        let mut memory = Memory::new();

        registers.pc = 0xFFFF;
        registers.set(0, 0x4321);

        let instr: u16 = 0b0011_000_000_000001; // ST R0, #1
        op_st(&mut registers, instr, &mut memory);

        assert_eq!(memory.read(0x0000), 0x4321);
    }

    #[test]
    fn op_st_preserves_registers() {
        let mut registers = Registers::new();
        let mut memory = Memory::new();

        registers.pc = 0x3000;
        registers.set(0, 0x7777);
        registers.set(1, 0x8888);

        let instr: u16 = 0b0011_000_000_000010; // ST R0, #2
        op_st(&mut registers, instr, &mut memory);

        assert_eq!(registers.get(1), 0x8888);
    }
}
