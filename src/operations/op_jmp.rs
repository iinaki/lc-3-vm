use crate::registers::Registers;

pub fn op_jmp(registers: &mut Registers, instr: u16) {
    let r1 = (instr >> 6) & 0x7;
    registers.pc = registers.get(r1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn op_jmp_changes_pc() {
        let mut registers = Registers::new();
        registers.set(1, 0x3033);

        let instr: u16 = 0b1100_000_001_000_000;
        op_jmp(&mut registers, instr);

        assert_eq!(registers.pc, 0x3033);
    }

    #[test]
    fn op_jmp_to_zero() {
        let mut registers = Registers::new();
        registers.set(2, 0x0000);

        let instr: u16 = 0b1100_000_010_000_000;
        op_jmp(&mut registers, instr);

        assert_eq!(registers.pc, 0x0000);
    }

    #[test]
    fn op_jmp_to_high_address() {
        let mut registers = Registers::new();
        registers.set(3, 0xFFFF);

        let instr: u16 = 0b1100_000_011_000_000;
        op_jmp(&mut registers, instr);

        assert_eq!(registers.pc, 0xFFFF);
    }
}
