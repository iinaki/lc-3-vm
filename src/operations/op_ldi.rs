use crate::{memory::Memory, register::Register};

use super::{sign_extend, update_flags};

pub fn op_ldi(register: &mut Register, instr: u16, memory: &mut Memory) {
    let r0 = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1FF, 9);
    let addr = (register.pc as i16 + pc_offset) as u16;
    let indirect_addr = memory.read(addr);
    register.set(r0, memory.read(indirect_addr));
    update_flags(register, r0);
}

#[cfg(test)]
mod tests {
    use crate::constants::FL_ZRO;

    use super::*;

    #[test]
    fn op_ldi_positive_offset() {
        let mut register = Register::new();
        let mut memory = Memory::new();

        register.pc = 0x3000;
        memory.write(0x3002, 0x4000);
        memory.write(0x4000, 0x1234);

        let instr: u16 = 0b1010_000_000000010; // LDI R0, PC+2
        op_ldi(&mut register, instr, &mut memory);

        assert_eq!(register.get(0), 0x1234);
    }

    #[test]
    fn op_ldi_negative_offset() {
        let mut register = Register::new();
        let mut memory = Memory::new();

        register.pc = 0x3000;
        memory.write(0x2FFE, 0x4000);
        memory.write(0x4000, 0xABCD);

        let instr: u16 = 0b1010_000_111111110; // LDI R0, PC-2
        op_ldi(&mut register, instr, &mut memory);

        assert_eq!(register.get(0), 0xABCD);
    }

    #[test]
    fn op_ldi_zero_offset() {
        let mut register = Register::new();
        let mut memory = Memory::new();

        register.pc = 0x3000;
        memory.write(0x3000, 0x5000);
        memory.write(0x5000, 0x5678);

        let instr: u16 = 0b1010_000_000000000; // LDI R0, PC+0
        op_ldi(&mut register, instr, &mut memory);

        assert_eq!(register.get(0), 0x5678);
    }

    #[test]
    fn op_ldi_update_flags() {
        let mut register = Register::new();
        let mut memory = Memory::new();

        register.pc = 0x3000;
        memory.write(0x3000, 0x0000);
        memory.write(0x0000, 0x0000);

        let instr: u16 = 0b1010_000_000000000; // LDI R0, PC+0
        op_ldi(&mut register, instr, &mut memory);

        assert_eq!(register.get(0), 0x0000);
        assert_eq!(register.cond, FL_ZRO);
    }

    #[test]
    fn op_ldi_preserves_pc() {
        let mut register = Register::new();
        let mut memory = Memory::new();

        register.pc = 0x3000;

        let instr: u16 = 0b1010_000_000000010; // LDI R0, PC+2
        op_ldi(&mut register, instr, &mut memory);

        assert_eq!(register.pc, 0x3000);
    }
}
