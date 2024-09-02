use crate::register::Register;

use super::update_flags;

// NOT{
//     uint16_t r0 = (instr >> 9) & 0x7;
//     uint16_t r1 = (instr >> 6) & 0x7;
//     reg[r0] = ~reg[r1];
//     update_flags(r0);
// }

pub fn op_not(register: &mut Register, instr: u16) {
    let r0 = (instr >> 9) & 0x7;
    let r1 = (instr >> 6) & 0x7;
    register.set(r0, !register.get(r1));
    update_flags(register, r0);
}

#[cfg(test)]
mod tests {
    use crate::constants::{FL_NEG, FL_ZRO};

    use super::*;

    #[test]
    fn test_op_not_basic() {
        let mut register = Register::new();
        register.set(1, 0x0F0F); 

        let instr: u16 = 0b1001_000_001_111111; // NOT R0, R1
        op_not(&mut register, instr);

        assert_eq!(register.get(0), 0xF0F0);
    }

    #[test]
    fn test_op_not_zero() {
        let mut register = Register::new();
        register.set(1, 0x0000);

        let instr: u16 = 0b1001_000_001_111111; // NOT R0, R1
        op_not(&mut register, instr);

        assert_eq!(register.get(0), 0xFFFF); 
    }

    #[test]
    fn test_op_not_all_ones() {
        let mut register = Register::new();
        register.set(1, 0xFFFF); 

        let instr: u16 = 0b1001_000_001_111111; // NOT R0, R1
        op_not(&mut register, instr);

        assert_eq!(register.get(0), 0x0000);
        assert_eq!(register.cond, FL_ZRO);
    }

    #[test]
    fn test_op_not_update_flags_negative() {
        let mut register = Register::new();
        register.set(1, 0x7FFF); 

        let instr: u16 = 0b1001_000_001_111111; // NOT R0, R1
        op_not(&mut register, instr);

        assert_eq!(register.get(0), 0x8000); 
        assert_eq!(register.cond, FL_NEG); 
    }
}
