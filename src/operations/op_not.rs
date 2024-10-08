use crate::{vm::Vm, vm_error::VmError};

impl Vm {
    /// Executes the NOT operation.
    ///
    /// Performs a bitwise NOT operation on the value in the source register,
    /// storing the result in the destination register. The condition flags are then updated
    /// based on the result.
    ///
    /// # Parameters
    ///
    /// - `instr`: A 16-bit instruction.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the operation was successful, otherwise returns a `VmError`.
    ///
    pub fn op_not(&mut self, instr: u16) -> Result<(), VmError> {
        let r0 = (instr >> 9) & 0x7;
        let r1 = (instr >> 6) & 0x7;
        self.registers.set(r0, !self.registers.get(r1)?)?;
        self.registers.update_flags(r0)
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        constants::{FL_NEG, FL_ZRO},
        memory::Memory,
        registers::Registers,
        vm::Vm,
    };

    fn create_vm() -> Vm {
        Vm {
            registers: Registers::new(),
            memory: Memory::new(),
        }
    }

    #[test]
    fn op_not_basic() {
        let mut vm = create_vm();
        vm.registers.set(1, 0x0F0F).unwrap();

        let instr: u16 = 0b1001_0000_0111_1111; // NOT R0, R1
        vm.op_not(instr).unwrap();

        assert_eq!(vm.registers.get(0).unwrap(), 0xF0F0);
    }

    #[test]
    fn op_not_zero() {
        let mut vm = create_vm();
        vm.registers.set(1, 0x0000).unwrap();

        let instr: u16 = 0b1001_0000_0111_1111; // NOT R0, R1
        vm.op_not(instr).unwrap();

        assert_eq!(vm.registers.get(0).unwrap(), 0xFFFF);
    }

    #[test]
    fn op_not_all_ones() {
        let mut vm = create_vm();
        vm.registers.set(1, 0xFFFF).unwrap();

        let instr: u16 = 0b1001_0000_0111_1111; // NOT R0, R1
        vm.op_not(instr).unwrap();

        assert_eq!(vm.registers.get(0).unwrap(), 0x0000);
        assert_eq!(vm.registers.cond, FL_ZRO);
    }

    #[test]
    fn op_not_update_flags_negative() {
        let mut vm = create_vm();
        vm.registers.set(1, 0x7FFF).unwrap();

        let instr: u16 = 0b1001_0000_0111_1111; // NOT R0, R1
        vm.op_not(instr).unwrap();

        assert_eq!(vm.registers.get(0).unwrap(), 0x8000);
        assert_eq!(vm.registers.cond, FL_NEG);
    }
}
