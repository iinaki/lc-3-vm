use crate::{
    memory::Memory,
    registers::Registers,
    utils::{sign_extend, update_flags},
};

/// Executes the LDR operation.
///
/// # Parameters
///
/// - `registers`: A mutable reference to the `Registers` struct.
/// - `instr`: A 16-bit instruction.
///
pub fn op_ldr(registers: &mut Registers, instr: u16, memory: &mut Memory) {
    let r0 = (instr >> 9) & 0x7;
    let r1 = (instr >> 6) & 0x7;
    let offset = sign_extend(instr & 0x3F, 6);
    let addr = (registers.get(r1) as i16 + offset) as u16;
    registers.set(r0, memory.read(addr));
    update_flags(registers, r0);
}

#[cfg(test)]
mod tests {
    use crate::constants::FL_ZRO;

    use super::*;

    #[test]
    fn op_ldr_positive_offset() {
        let mut registers = Registers::new();
        let mut memory = Memory::new();

        registers.set(1, 0x3000);
        memory.write(0x3002, 0xABCD);

        let instr: u16 = 0b0110_0000_0100_0010; // LDR R0, R1, #2
        op_ldr(&mut registers, instr, &mut memory);

        assert_eq!(registers.get(0), 0xABCD);
    }

    #[test]
    fn op_ldr_negative_offset() {
        let mut registers = Registers::new();
        let mut memory = Memory::new();

        registers.set(1, 0x3002);
        memory.write(0x3000, 0x5678);

        let instr: u16 = 0b0110_0000_0111_1110; // LDR R0, R1, #-2
        op_ldr(&mut registers, instr, &mut memory);

        assert_eq!(registers.get(0), 0x5678);
    }

    #[test]
    fn op_ldr_zero_offset() {
        let mut registers = Registers::new();
        let mut memory = Memory::new();

        registers.set(1, 0x3000);
        memory.write(0x3000, 0x9ABC);

        let instr: u16 = 0b0110_0000_0100_0000; // LDR R0, R1, #0
        op_ldr(&mut registers, instr, &mut memory);

        assert_eq!(registers.get(0), 0x9ABC);
    }

    #[test]
    fn op_ldr_update_flags() {
        let mut registers = Registers::new();
        let mut memory = Memory::new();

        registers.set(1, 0x3000);
        memory.write(0x3000, 0x0000);

        let instr: u16 = 0b0110_0000_0100_0000; // LDR R0, R1, #0
        op_ldr(&mut registers, instr, &mut memory);

        assert_eq!(registers.get(0), 0x0000);
        assert_eq!(registers.cond, FL_ZRO);
    }

    #[test]
    fn op_ldr_preserves_pc() {
        let mut registers = Registers::new();
        let mut memory = Memory::new();

        registers.set(1, 0x3000);

        let instr: u16 = 0b0110_0000_0100_0010; // LDR R0, R1, #2
        let initial_pc = registers.pc;
        op_ldr(&mut registers, instr, &mut memory);

        assert_eq!(registers.pc, initial_pc);
    }
}
