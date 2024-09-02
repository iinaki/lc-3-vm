use crate::{memory::Memory, register::Register};

use super::sign_extend;

// STR {
//     uint16_t r0 = (instr >> 9) & 0x7;
//     uint16_t r1 = (instr >> 6) & 0x7;
//     uint16_t offset = sign_extend(instr & 0x3F, 6);
//     mem_write(reg[r1] + offset, reg[r0]);
// }

pub fn op_str(register: &mut Register, instr: u16, memory: &mut Memory) {
    let r0 = (instr >> 9) & 0x7;
    let r1 = (instr >> 6) & 0x7;
    let offset = sign_extend(instr & 0x3F, 6);
    memory.write((register.get(r1) as i16 + offset) as u16, register.get(r0));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_op_str_basic() {
        let mut register = Register::new();
        let mut memory = Memory::new();

        register.set(0, 0xABCD);
        register.set(1, 0x3000);

        let instr: u16 = 0b0111_000_001_000010; // STR R0, R1, #2
        op_str(&mut register, instr, &mut memory);

        assert_eq!(memory.read(0x3002), 0xABCD);
    }

    #[test]
    fn test_op_str_negative_offset() {
        let mut register = Register::new();
        let mut memory = Memory::new();

        register.set(0, 0x1234);
        register.set(1, 0x3004);

        let instr: u16 = 0b0111_000_001_111110; // STR R0, R1, #-2
        op_str(&mut register, instr, &mut memory);

        assert_eq!(memory.read(0x3002), 0x1234);
    }

    #[test]
    fn test_op_str_zero_offset() {
        let mut register = Register::new();
        let mut memory = Memory::new();

        register.set(0, 0x5678);
        register.set(1, 0x4000);

        let instr: u16 = 0b0111_000_001_000000; // STR R0, R1, #0
        op_str(&mut register, instr, &mut memory);

        assert_eq!(memory.read(0x4000), 0x5678);
    }

    #[test]
    fn test_op_str_large_offset() {
        let mut register = Register::new();
        let mut memory = Memory::new();

        register.set(0, 0x9ABC);
        register.set(1, 0x1000);

        let instr: u16 = 0b0111_000_001_001111; // STR R0, R1, #15
        op_str(&mut register, instr, &mut memory);

        assert_eq!(memory.read(0x100F), 0x9ABC);
    }

    #[test]
    fn test_op_str_overflow_address() {
        let mut register = Register::new();
        let mut memory = Memory::new();

        register.set(0, 0x4321);
        register.set(1, 0xFFFF);

        let instr: u16 = 0b0111_000_001_000001; // STR R0, R1, #1
        op_str(&mut register, instr, &mut memory);

        assert_eq!(memory.read(0x0000), 0x4321);
    }
}
