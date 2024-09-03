use crate::{memory::Memory, registers::Registers};

use super::{sign_extend, update_flags};

pub fn op_ld(registers: &mut Registers, instr: u16, memory: &mut Memory) {
    let r0 = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1FF, 9);
    let address = (registers.pc as i16 + pc_offset) as u16;
    registers.set(r0, memory.read(address));
    update_flags(registers, r0);
}

#[cfg(test)]
mod tests {
    use crate::constants::FL_ZRO;

    use super::*;

    #[test]
    fn op_ld_positive_offset() {
        let mut registers = Registers::new();
        let mut memory = Memory::new();

        memory.write(0x3002, 0x1234);

        let instr: u16 = 0b0010_0000_0000_0010; // LD R0, PC+2
        op_ld(&mut registers, instr, &mut memory);

        assert_eq!(registers.get(0), 0x1234);
    }

    #[test]
    fn op_ld_negative_offset() {
        let mut registers = Registers::new();
        let mut memory = Memory::new();

        registers.pc = 0x3000;
        memory.write(0x2FFE, 0xABCD);

        let instr: u16 = 0b0010_0001_1111_1110; // LD R0, PC-2
        op_ld(&mut registers, instr, &mut memory);

        assert_eq!(registers.get(0), 0xABCD);
    }

    #[test]
    fn op_ld_zero_offset() {
        let mut registers = Registers::new();
        let mut memory = Memory::new();

        registers.pc = 0x3000;
        memory.write(0x3000, 0x5678);

        let instr: u16 = 0b0010_0000_0000_0000; // LD R0, PC+0
        op_ld(&mut registers, instr, &mut memory);

        assert_eq!(registers.get(0), 0x5678);
    }

    #[test]
    fn op_ld_update_flags() {
        let mut registers = Registers::new();
        let mut memory = Memory::new();

        registers.pc = 0x3000;
        memory.write(0x3000, 0x0000);

        let instr: u16 = 0b0010_0000_0000_0000; // LD R0, PC+0
        op_ld(&mut registers, instr, &mut memory);

        assert_eq!(registers.get(0), 0x0000);
        assert_eq!(registers.cond, FL_ZRO);
    }

    #[test]
    fn op_ld_preserves_pc() {
        let mut registers = Registers::new();
        let mut memory = Memory::new();

        registers.pc = 0x3000;

        let instr: u16 = 0b0010_0000_0000_0010; // LD R0, PC+2
        op_ld(&mut registers, instr, &mut memory);

        assert_eq!(registers.pc, 0x3000);
    }
}
