use crate::register::Register;

// JMP {
//     /* Also handles RET */
//     uint16_t r1 = (instr >> 6) & 0x7;
//     reg[R_PC] = reg[r1];
// }

pub fn op_jmp(register: &mut Register, instr: u16) {
    let r1 = (instr >> 6) & 0x7;
    register.pc = register.get(r1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_op_jmp() {
        let mut register = Register::new();
        register.set(1, 0x3000);  

        let instr: u16 = 0b1100_000_001_000_000; 
        op_jmp(&mut register, instr);

        assert_eq!(register.pc, 0x3000);
    }

    #[test]
    fn test_op_jmp_to_zero() {
        let mut register = Register::new();
        register.set(2, 0x0000); 

        let instr: u16 = 0b1100_000_010_000_000; 
        op_jmp(&mut register, instr);

        assert_eq!(register.pc, 0x0000); 
    }

    #[test]
    fn test_op_jmp_to_high_address() {
        let mut register = Register::new();
        register.set(3, 0xFFFF);  

        let instr: u16 = 0b1100_000_011_000_000; 
        op_jmp(&mut register, instr);

        assert_eq!(register.pc, 0xFFFF); 
    }
}
