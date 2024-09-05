use crate::{utils::sign_extend, vm::Vm, vm_error::VmError};

pub trait OpAdd {
    fn op_add(&mut self, instr: u16) -> Result<(), VmError>;
}

impl OpAdd for Vm {
    /// Executes the ADD operation.
    ///
    /// The add can be between two registers or between a register and an immediate value.
    /// The result is stored in a dest register, and the condition flags are updated
    /// to reflect the result of the operation.
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
    fn op_add(&mut self, instr: u16) -> Result<(), VmError> {
        let r0 = (instr >> 9) & 0x7;
        let r1 = (instr >> 6) & 0x7;
        let imm_flag = (instr >> 5) & 0x1;

        if imm_flag == 1 {
            let imm5 = sign_extend(instr & 0x1F, 5);
            self.registers
                .set(r0, self.registers.get(r1)?.wrapping_add(imm5 as u16))?;
        } else {
            let r2 = instr & 0x7;
            self.registers.set(
                r0,
                self.registers
                    .get(r1)?
                    .wrapping_add(self.registers.get(r2)?),
            )?;
        }

        self.registers.update_flags(r0)
    }
}

#[cfg(test)]
mod tests {
    use termios::Termios;

    use crate::{
        constants::{FL_NEG, FL_POS, FL_ZRO},
        memory::Memory,
        operations::op_add::OpAdd,
        registers::Registers,
        vm::Vm,
    };

    fn create_vm() -> Vm {
        Vm {
            registers: Registers::new(),
            memory: Memory::new(),
            termios: Termios::from_fd(0).unwrap(),
        }
    }

    #[test]
    fn op_add_with_registers() {
        let mut vm = create_vm();

        vm.registers.set(1, 10).unwrap();
        vm.registers.set(2, 15).unwrap();

        let instr: u16 = 0b0001_0000_0100_0010;
        vm.op_add(instr).unwrap();

        assert_eq!(vm.registers.get(0).unwrap(), 25);
    }

    #[test]
    fn op_add_with_immediate_positive() {
        let mut vm = create_vm();
        vm.registers.set(1, 10).unwrap();

        let instr: u16 = 0b0001_0000_0110_0001;
        vm.op_add(instr).unwrap();

        assert_eq!(vm.registers.get(0).unwrap(), 11);
    }

    #[test]
    fn op_add_with_immediate_negative() {
        let mut vm = create_vm();
        vm.registers.set(1, 10).unwrap();

        let instr: u16 = 0b0001_0000_0111_1111;
        vm.op_add(instr).unwrap();

        assert_eq!(vm.registers.get(0).unwrap(), 9);
    }

    #[test]
    fn op_add_with_negative_result() {
        let mut vm = create_vm();
        vm.registers.set(1, 0).unwrap();

        let instr: u16 = 0b0001_0000_0111_1111;
        vm.op_add(instr).unwrap();

        assert_eq!(vm.registers.get(0).unwrap(), 0xFFFF);
        assert_eq!(vm.registers.cond, FL_NEG);
    }

    #[test]
    fn op_add_with_zero_result() {
        let mut vm = create_vm();
        vm.registers.set(1, 1).unwrap();

        let instr: u16 = 0b0001_0000_0111_1111;
        vm.op_add(instr).unwrap();

        assert_eq!(vm.registers.get(0).unwrap(), 0);
        assert_eq!(vm.registers.cond, FL_ZRO);
    }

    #[test]
    fn op_add_with_positive_result() {
        let mut vm = create_vm();
        vm.registers.set(1, 1).unwrap();

        let instr: u16 = 0b0001_0000_0110_0001;
        vm.op_add(instr).unwrap();

        assert_eq!(vm.registers.get(0).unwrap(), 2);
        assert_eq!(vm.registers.cond, FL_POS);
    }
}
