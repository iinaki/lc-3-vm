use crate::{memory::Memory, register::Register};

use super::{sign_extend, update_flags};

pub fn op_ldr(register: &mut Register, instr: u16, memory: &mut Memory) {
    let r0 = (instr >> 9) & 0x7;
    let r1 = (instr >> 6) & 0x7;
    let offset = sign_extend(instr & 0x3F, 6);
    let addr = (register.get(r1) as i16 + offset) as u16;
    register.set(r0, memory.read(addr));
    update_flags(register, r0);
}

#[cfg(test)]
mod tests {
    use crate::constants::FL_ZRO;

    use super::*;

    #[test]
    fn op_ldr_positive_offset() {
        let mut register = Register::new();
        let mut memory = Memory::new();

        register.set(1, 0x3000);
        memory.write(0x3002, 0xABCD);

        let instr: u16 = 0b0110_000_001000010; // LDR R0, R1, #2
        op_ldr(&mut register, instr, &mut memory);

        assert_eq!(register.get(0), 0xABCD);
    }

    #[test]
    fn op_ldr_negative_offset() {
        let mut register = Register::new();
        let mut memory = Memory::new();

        register.set(1, 0x3002);
        memory.write(0x3000, 0x5678);

        let instr: u16 = 0b0110_000_001111110; // LDR R0, R1, #-2
        op_ldr(&mut register, instr, &mut memory);

        assert_eq!(register.get(0), 0x5678);
    }

    #[test]
    fn op_ldr_zero_offset() {
        let mut register = Register::new();
        let mut memory = Memory::new();

        register.set(1, 0x3000);
        memory.write(0x3000, 0x9ABC);

        let instr: u16 = 0b0110_000_001000000; // LDR R0, R1, #0
        op_ldr(&mut register, instr, &mut memory);

        assert_eq!(register.get(0), 0x9ABC);
    }

    #[test]
    fn op_ldr_update_flags() {
        let mut register = Register::new();
        let mut memory = Memory::new();

        register.set(1, 0x3000);
        memory.write(0x3000, 0x0000);

        let instr: u16 = 0b0110_000_001000000; // LDR R0, R1, #0
        op_ldr(&mut register, instr, &mut memory);

        assert_eq!(register.get(0), 0x0000);
        assert_eq!(register.cond, FL_ZRO);
    }

    #[test]
    fn op_ldr_preserves_pc() {
        let mut register = Register::new();
        let mut memory = Memory::new();

        register.set(1, 0x3000);

        let instr: u16 = 0b0110_000_001000010; // LDR R0, R1, #2
        let initial_pc = register.pc;
        op_ldr(&mut register, instr, &mut memory);

        assert_eq!(register.pc, initial_pc);
    }
}
