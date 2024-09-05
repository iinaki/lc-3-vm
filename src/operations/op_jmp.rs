use crate::{vm::Vm, vm_error::VmError};

pub trait OpJmp {
    fn op_jmp(&mut self, instr: u16) -> Result<(), VmError>;
}

impl OpJmp for Vm {
    /// Executes the JUMP operation.
    ///
    /// This opcode updates the program counter to the address stored in the specified
    /// register, allowing for an unconditional jump to a new location in the program.
    ///
    /// # Parameters
    ///
    /// - `registers`: A mutable reference to the `Registers` struct.
    /// - `instr`: A 16-bit instruction.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the operation was successful, otherwise returns a `VmError`.
    ///
    fn op_jmp(&mut self, instr: u16) -> Result<(), VmError> {
        let r1 = (instr >> 6) & 0x7;
        self.registers.pc = self.registers.get(r1)?;
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use crate::{memory::Memory, operations::op_jmp::OpJmp, registers::Registers, vm::Vm};

    fn create_vm() -> Vm {
        Vm {
            registers: Registers::new(),
            memory: Memory::new(),
        }
    }

    #[test]
    fn op_jmp_changes_pc() {
        let mut vm = create_vm();
        vm.registers.set(1, 0x3033).unwrap();

        let instr: u16 = 0b1100_0000_0100_0000;
        vm.op_jmp(instr).unwrap();

        assert_eq!(vm.registers.pc, 0x3033);
    }

    #[test]
    fn op_jmp_to_zero() {
        let mut vm = create_vm();
        vm.registers.set(2, 0x0000).unwrap();

        let instr: u16 = 0b1100_0000_1000_0000;
        vm.op_jmp(instr).unwrap();

        assert_eq!(vm.registers.pc, 0x0000);
    }

    #[test]
    fn op_jmp_to_high_address() {
        let mut vm = create_vm();
        vm.registers.set(3, 0xFFFF).unwrap();

        let instr: u16 = 0b1100_0000_1100_0000;
        vm.op_jmp(instr).unwrap();

        assert_eq!(vm.registers.pc, 0xFFFF);
    }
}
