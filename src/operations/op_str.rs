use crate::{memory::Memory, registers::Registers};

use super::sign_extend;

pub fn op_str(registers: &mut Registers, instr: u16, memory: &mut Memory) {
    let r0 = (instr >> 9) & 0x7;
    let r1 = (instr >> 6) & 0x7;
    let offset = sign_extend(instr & 0x3F, 6);
    memory.write((registers.get(r1) as i16 + offset) as u16, registers.get(r0));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn op_str_basic() {
        let mut registers = Registers::new();
        let mut memory = Memory::new();

        registers.set(0, 0xABCD);
        registers.set(1, 0x3000);

        let instr: u16 = 0b0111_000_001_000010; // STR R0, R1, #2
        op_str(&mut registers, instr, &mut memory);

        assert_eq!(memory.read(0x3002), 0xABCD);
    }

    #[test]
    fn op_str_negative_offset() {
        let mut registers = Registers::new();
        let mut memory = Memory::new();

        registers.set(0, 0x1234);
        registers.set(1, 0x3004);

        let instr: u16 = 0b0111_000_001_111110; // STR R0, R1, #-2
        op_str(&mut registers, instr, &mut memory);

        assert_eq!(memory.read(0x3002), 0x1234);
    }

    #[test]
    fn op_str_zero_offset() {
        let mut registers = Registers::new();
        let mut memory = Memory::new();

        registers.set(0, 0x5678);
        registers.set(1, 0x4000);

        let instr: u16 = 0b0111_000_001_000000; // STR R0, R1, #0
        op_str(&mut registers, instr, &mut memory);

        assert_eq!(memory.read(0x4000), 0x5678);
    }

    #[test]
    fn op_str_large_offset() {
        let mut registers = Registers::new();
        let mut memory = Memory::new();

        registers.set(0, 0x9ABC);
        registers.set(1, 0x1000);

        let instr: u16 = 0b0111_000_001_001111; // STR R0, R1, #15
        op_str(&mut registers, instr, &mut memory);

        assert_eq!(memory.read(0x100F), 0x9ABC);
    }

    #[test]
    fn op_str_overflow_address() {
        let mut registers = Registers::new();
        let mut memory = Memory::new();

        registers.set(0, 0x4321);
        registers.set(1, 0xFFFF);

        let instr: u16 = 0b0111_000_001_000001; // STR R0, R1, #1
        op_str(&mut registers, instr, &mut memory);

        assert_eq!(memory.read(0x0000), 0x4321);
    }
}
