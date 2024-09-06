use crate::{utils::sign_extend, vm::Vm, vm_error::VmError};

impl Vm {
    /// Executes the LD operation.
    ///
    /// Loads a value from memory into a register. The address is computed by
    /// adding a sign-extended offset to the current program counter, and the result is stored in the destination register. The condition flags are then updated based on the loaded value.
    ///
    /// # Parameters
    ///
    /// - `instr`: A 16-bit instruction.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the operation was successful, otherwise returns a `VmError`.
    ///
    pub fn op_ld(&mut self, instr: u16) -> Result<(), VmError> {
        let r0 = (instr >> 9) & 0x7;
        let pc_offset = sign_extend(instr & 0x1FF, 9);
        let address = (self.registers.pc as i16 + pc_offset) as u16;
        self.registers.set(r0, self.memory.read(address)?)?;
        self.registers.update_flags(r0)
    }
}
#[cfg(test)]
mod tests {
    use crate::{constants::FL_ZRO, memory::Memory, registers::Registers, vm::Vm};

    fn create_vm() -> Vm {
        Vm {
            registers: Registers::new(),
            memory: Memory::new(),
        }
    }

    #[test]
    fn op_ld_positive_offset() {
        let mut vm = create_vm();

        vm.memory.write(0x3002, 0x1234);

        let instr: u16 = 0b0010_0000_0000_0010; // LD R0, PC+2
        vm.op_ld(instr).unwrap();

        assert_eq!(vm.registers.get(0).unwrap(), 0x1234);
    }

    #[test]
    fn op_ld_negative_offset() {
        let mut vm = create_vm();

        vm.registers.pc = 0x3000;
        vm.memory.write(0x2FFE, 0xABCD);

        let instr: u16 = 0b0010_0001_1111_1110; // LD R0, PC-2
        vm.op_ld(instr).unwrap();

        assert_eq!(vm.registers.get(0).unwrap(), 0xABCD);
    }

    #[test]
    fn op_ld_zero_offset() {
        let mut vm = create_vm();

        vm.registers.pc = 0x3000;
        vm.memory.write(0x3000, 0x5678);

        let instr: u16 = 0b0010_0000_0000_0000; // LD R0, PC+0
        vm.op_ld(instr).unwrap();

        assert_eq!(vm.registers.get(0).unwrap(), 0x5678);
    }

    #[test]
    fn op_ld_update_flags() {
        let mut vm = create_vm();

        vm.registers.pc = 0x3000;
        vm.memory.write(0x3000, 0x0000);

        let instr: u16 = 0b0010_0000_0000_0000; // LD R0, PC+0
        vm.op_ld(instr).unwrap();

        assert_eq!(vm.registers.get(0).unwrap(), 0x0000);
        assert_eq!(vm.registers.cond, FL_ZRO);
    }

    #[test]
    fn op_ld_preserves_pc() {
        let mut vm = create_vm();

        vm.registers.pc = 0x3000;

        let instr: u16 = 0b0010_0000_0000_0010; // LD R0, PC+2
        vm.op_ld(instr).unwrap();

        assert_eq!(vm.registers.pc, 0x3000);
    }
}
