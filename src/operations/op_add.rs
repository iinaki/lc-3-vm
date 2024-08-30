use crate::register::Register;

use super::{sign_extend, update_flags};


// ADD {
//     /* destination register (DR) */
//     uint16_t r0 = (instr >> 9) & 0x7;
//     /* first operand (SR1) */
//     uint16_t r1 = (instr >> 6) & 0x7;
//     /* whether we are in immediate mode */
//     uint16_t imm_flag = (instr >> 5) & 0x1;

//     if (imm_flag)
//     {
//         uint16_t imm5 = sign_extend(instr & 0x1F, 5);
//         reg[r0] = reg[r1] + imm5;
//     }
//     else
//     {
//         uint16_t r2 = instr & 0x7;
//         reg[r0] = reg[r1] + reg[r2];
//     }

//     update_flags(r0);
// }

pub fn op_add(register: &mut Register, instr: u16) {
    let r0 = (instr >> 9) & 0x7;
    let r1 = (instr >> 6) & 0x7;
    let imm_flag = (instr >> 5) & 0x1;

    if imm_flag == 1 {
        let imm5 = sign_extend(instr & 0x1F, 5) as i16;
        register.set(r0, (register.get(r1) as i16 + imm5) as u16);
    } else {
        let r2 = instr & 0x7;
        register.set(r0, register.get(r1) + register.get(r2));
        println!("R0: {}", r0);
        println!("R1: {}", r1);
        println!("R2: {}", r2);
    }

    update_flags(register, r0);
}


#[cfg(test)]
mod tests {
    use crate::{constants::{FL_NEG, FL_POS, FL_ZRO}, register::Register};
    use super::*;

    // ADD TESTS
    #[test]
    fn test_op_add_with_registers() {
        let mut register = Register::new();
        register.set(1, 10);  
        register.set(2, 15);  

        let instr: u16 = 0b0001_000_001_000_010;
        op_add(&mut register, instr);
        println!("REGISTERS: {:?}", register);

        assert_eq!(register.get(0), 25); 
    }

    #[test]
    fn test_op_add_with_immediate_positive() {
        let mut register = Register::new();
        register.set(1, 10);

        let instr: u16 = 0b0001_000_001_1_00001; 
        op_add(&mut register, instr);

        assert_eq!(register.get(0), 11); 
    }

    #[test]
    fn test_op_add_with_immediate_negative() {
        let mut register = Register::new();
        register.set(1, 10); 

        let instr: u16 = 0b0001_000_001_1_11111;
        op_add(&mut register, instr);

        assert_eq!(register.get(0), 9);
    }

    #[test]
    fn test_op_add_with_negative_result() {
        let mut register = Register::new();
        register.set(1, 0);

        let instr: u16 = 0b0001_000_001_1_11111;
        op_add(&mut register, instr);

        assert_eq!(register.get(0), 0xFFFF); 
        assert_eq!(register.cond, FL_NEG); 
    }

    #[test]
    fn test_op_add_with_zero_result() {
        let mut register = Register::new();
        register.set(1, 1); 

        let instr: u16 = 0b0001_000_001_1_11111; 
        op_add(&mut register, instr);

        assert_eq!(register.get(0), 0); 
        assert_eq!(register.cond, FL_ZRO); 
    }

    #[test]
    fn test_op_add_with_positive_result() {
        let mut register = Register::new();
        register.set(1, 1); 

        let instr: u16 = 0b0001_000_001_1_00001;
        op_add(&mut register, instr);

        assert_eq!(register.get(0), 2); 
        assert_eq!(register.cond, FL_POS); 
    }
}
