use crate::{memory::Memory, register::Register};

use super::{sign_extend, update_flags};

// LD {
//     uint16_t r0 = (instr >> 9) & 0x7;
//     uint16_t pc_offset = sign_extend(instr & 0x1FF, 9);
//     reg[r0] = mem_read(reg[R_PC] + pc_offset);
//     update_flags(r0);
// }

pub fn op_ld(register: &mut Register, instr: u16, memory: &mut Memory) {
    let r0 = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1FF, 9);
    let address = (register.pc as i16 + pc_offset) as u16;
    register.set(r0, memory.read(address));
    update_flags(register, r0);
}

#[cfg(test)]
mod tests {
    use crate::constants::FL_ZRO;

    use super::*;

    #[test]
    fn test_op_ld_positive_offset() {
        let mut register = Register::new();
        let mut memory = Memory::new();

        memory.write(0x3002, 0x1234);

        let instr: u16 = 0b0010_000_000000010; // LD R0, PC+2
        op_ld(&mut register, instr, &mut memory);

        assert_eq!(register.get(0), 0x1234);
    }

    #[test]
    fn test_op_ld_negative_offset() {
        let mut register = Register::new();
        let mut memory = Memory::new();

        register.pc = 0x3000;
        memory.write(0x2FFE, 0xABCD);

        let instr: u16 = 0b0010_000_111111110; // LD R0, PC-2
        op_ld(&mut register, instr, &mut memory);

        assert_eq!(register.get(0), 0xABCD);
    }

    #[test]
    fn test_op_ld_zero_offset() {
        let mut register = Register::new();
        let mut memory = Memory::new();

        register.pc = 0x3000;
        memory.write(0x3000, 0x5678);

        let instr: u16 = 0b0010_000_000000000; // LD R0, PC+0
        op_ld(&mut register, instr, &mut memory);

        assert_eq!(register.get(0), 0x5678);
    }

    #[test]
    fn test_op_ld_update_flags() {
        let mut register = Register::new();
        let mut memory = Memory::new();

        register.pc = 0x3000;
        memory.write(0x3000, 0x0000);

        let instr: u16 = 0b0010_000_000000000; // LD R0, PC+0
        op_ld(&mut register, instr, &mut memory);

        assert_eq!(register.get(0), 0x0000);
        assert_eq!(register.cond, FL_ZRO);
    }

    #[test]
    fn test_op_ld_preserves_pc() {
        let mut register = Register::new();
        let mut memory = Memory::new();

        register.pc = 0x3000;

        let instr: u16 = 0b0010_000_000000010; // LD R0, PC+2
        op_ld(&mut register, instr, &mut memory);

        assert_eq!(register.pc, 0x3000);
    }
}
