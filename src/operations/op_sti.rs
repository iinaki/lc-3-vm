use crate::{memory::Memory, register::Register};

use super::sign_extend;

// STI {
//     uint16_t r0 = (instr >> 9) & 0x7;
//     uint16_t pc_offset = sign_extend(instr & 0x1FF, 9);
//     mem_write(mem_read(reg[R_PC] + pc_offset), reg[r0]);
// }

pub fn op_sti(register: &mut Register, instr: u16, memory: &mut Memory) {
    let r0 = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1FF, 9) as i16;
    let addr = memory.read((register.pc as i16 + pc_offset) as u16);
    memory.write(addr, register.get(r0));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_op_sti_basic() {
        let mut register = Register::new();
        let mut memory = Memory::new();

        register.pc = 0x3000;
        register.set(0, 0x1234);
        
        memory.write(0x3002, 0x4000);

        let instr: u16 = 0b1011_000_000_000010; // STI R0, #2
        op_sti(&mut register, instr, &mut memory);
        
        assert_eq!(memory.read(0x4000), 0x1234);
    }

    #[test]
    fn test_op_sti_negative_offset() {
        let mut register = Register::new();
        let mut memory = Memory::new();

        register.pc = 0x3000;
        register.set(0, 0x5678); 
        
        memory.write(0x2FFE, 0x5000);

        let instr: u16 = 0b1011_000_111_111110; // STI R0, #-2
        op_sti(&mut register, instr, &mut memory);
        
        assert_eq!(memory.read(0x5000), 0x5678);
    }

    #[test]
    fn test_op_sti_zero_offset() {
        let mut register = Register::new();
        let mut memory = Memory::new();

        register.pc = 0x3000;
        register.set(0, 0xABCD);
        
        memory.write(0x3000, 0x6000);

        let instr: u16 = 0b1011_000_000_000000; // STI R0, #0
        op_sti(&mut register, instr, &mut memory);
        
        assert_eq!(memory.read(0x6000), 0xABCD);
    }

    #[test]
    fn test_op_sti_overflow_offset() {
        let mut register = Register::new();
        let mut memory = Memory::new();

        register.pc = 0xFFFF;
        register.set(0, 0x4321); 
        
        memory.write(0x0000, 0x7000);

        let instr: u16 = 0b1011_000_000_000001; // STI R0, #1
        op_sti(&mut register, instr, &mut memory);
        
        assert_eq!(memory.read(0x7000), 0x4321);
    }

    #[test]
    fn test_op_sti_preserves_registers() {
        let mut register = Register::new();
        let mut memory = Memory::new();

        register.pc = 0x3000;
        register.set(0, 0x7777);
        register.set(1, 0x8888); 
        
        memory.write(0x3002, 0x8000);

        let instr: u16 = 0b1011_000_000_000010; // STI R0, #2
        op_sti(&mut register, instr, &mut memory);
        
        assert_eq!(register.get(1), 0x8888);
    }
}
