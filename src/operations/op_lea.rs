use crate::{utils::sign_extend, vm::Vm, vm_error::VmError};

impl Vm {
    /// Executes the LEA operation.
    ///
    /// Computes an address by adding a sign-extended offset to the current value
    /// of the program counter (PC). This computed address is then stored in the destination
    /// register. The condition flags are updated based on the computed address.
    ///
    /// # Parameters
    ///
    /// - `instr`: A 16-bit instruction.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the operation was successful, otherwise returns a `VmError`.
    ///
    pub fn op_lea(&mut self, instr: u16) -> Result<(), VmError> {
        let r0 = (instr >> 9) & 0x7;
        let pc_offset = sign_extend(instr & 0x1FF, 9);
        self.registers
            .set(r0, (self.registers.pc as i16 + pc_offset) as u16)?;
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
    fn op_lea_positive_offset() {
        let mut vm = create_vm();

        let instr: u16 = 0b1110_0000_0000_0101; // LEA R0, PC + 5
        vm.op_lea(instr).unwrap();

        assert_eq!(vm.registers.get(0).unwrap(), 0x3005);
    }

    #[test]
    fn op_lea_negative_offset() {
        let mut vm = create_vm();

        let instr: u16 = 0b1110_0001_1111_1011; // LEA R0, PC - 5
        vm.op_lea(instr).unwrap();

        assert_eq!(vm.registers.get(0).unwrap(), 0x2FFB);
    }

    #[test]
    fn op_lea_zero_offset() {
        let mut vm = create_vm();

        let instr: u16 = 0b1110_0000_0000_0000; // LEA R0, PC + 0
        vm.op_lea(instr).unwrap();

        assert_eq!(vm.registers.get(0).unwrap(), 0x3000);
    }

    #[test]
    fn op_lea_update_flags() {
        let mut vm = create_vm();
        vm.registers.pc = 0x0000;

        let instr: u16 = 0b1110_0000_0000_0000; // LEA R0, PC + 0
        vm.op_lea(instr).unwrap();

        assert_eq!(vm.registers.get(0).unwrap(), 0x0000);
        assert_eq!(vm.registers.cond, FL_ZRO);
    }

    #[test]
    fn op_lea_preserves_pc() {
        let mut vm = create_vm();

        let instr: u16 = 0b1110_0000_0000_0101; // LEA R0, PC + 5
        let initial_pc = vm.registers.pc;
        vm.op_lea(instr).unwrap();

        assert_eq!(vm.registers.pc, initial_pc);
    }
}
