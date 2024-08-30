use crate::register::Register;

use super::sign_extend;

// {
//     uint16_t long_flag = (instr >> 11) & 1;
//     reg[R_R7] = reg[R_PC];
//     if (long_flag)
//     {
//         uint16_t long_pc_offset = sign_extend(instr & 0x7FF, 11);
//         reg[R_PC] += long_pc_offset;  /* JSR */
//     }
//     else
//     {
//         uint16_t r1 = (instr >> 6) & 0x7;
//         reg[R_PC] = reg[r1]; /* JSRR */
//     }
// }

pub fn op_jsr(register: &mut Register, instr: u16) {
    let long_flag = (instr >> 11) & 1;
    register.r7 = register.pc;
    if long_flag == 1 {
        let long_pc_offset = sign_extend(instr & 0x7FF, 11) as i16;
        register.pc = (register.pc as i16 + long_pc_offset) as u16; /* JSR */
    } else {
        let r1 = (instr >> 6) & 0x7;
        register.pc = register.get(r1); /* JSRR */
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_op_jsr_long_offset() {
        let mut register = Register::new();
        register.pc = 0x3000;

        let instr: u16 = 0b0100_1_00000010000;
        op_jsr(&mut register, instr);

        assert_eq!(register.r7, 0x3000);
        assert_eq!(register.pc, 0x3010);
    }

    #[test]
    fn test_op_jsr_negative_offset() {
        let mut register = Register::new();
        register.pc = 0x3000;

        let instr: u16 = 0b0100_1_11111111111;
        op_jsr(&mut register, instr);

        assert_eq!(register.r7, 0x3000);
        assert_eq!(register.pc, 0x2FFF);
    }

    #[test]
    fn test_op_jsrr() {
        let mut register = Register::new();
        register.pc = 0x3000;
        register.set(2, 0x4000);

        let instr: u16 = 0b0100_0_000_010_000_000;
        op_jsr(&mut register, instr);

        assert_eq!(register.r7, 0x3000);
        assert_eq!(register.pc, 0x4000);
    }

    #[test]
    fn test_op_jsr_preserves_other_registers() {
        let mut register = Register::new();
        register.pc = 0x3000;
        register.set(1, 0xABCD);
        register.set(2, 0x1234);

        let instr: u16 = 0b0100_0_000_010_000_000;
        op_jsr(&mut register, instr);

        assert_eq!(register.r7, 0x3000);
        assert_eq!(register.pc, 0x1234);
        assert_eq!(register.get(1), 0xABCD);
    }

    #[test]
    fn test_op_jsr_long_offset_and_return() {
        let mut register = Register::new();
        register.pc = 0x3000;

        let instr_jsr: u16 = 0b0100_1_00000000010;
        op_jsr(&mut register, instr_jsr);

        assert_eq!(register.r7, 0x3000);
        assert_eq!(register.pc, 0x3002);

        register.pc = register.r7;
        assert_eq!(register.pc, 0x3000);
    }
}
