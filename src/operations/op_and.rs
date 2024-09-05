use crate::{utils::sign_extend, vm::Vm, vm_error::VmError};

pub trait OpAnd {
    fn op_and(&mut self, instr: u16) -> Result<(), VmError>;
}

impl OpAnd for Vm {
    /// Performs a bitwise AND operation between two operands.
    ///
    /// The first operand is always a register, and the second
    /// operand can be another register or an immediate value. The result of the AND
    /// operation is stored in the dest register, and the condition
    /// flags are updated based on the result.
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
    fn op_and(&mut self, instr: u16) -> Result<(), VmError> {
        let r0 = (instr >> 9) & 0x7;
        let r1 = (instr >> 6) & 0x7;
        let imm_flag = (instr >> 5) & 0x1;

        if imm_flag == 1 {
            let imm5 = sign_extend(instr & 0x1F, 5);
            self.registers
                .set(r0, (self.registers.get(r1)? as i16 & imm5) as u16)?;
        } else {
            let r2 = instr & 0x7;
            self.registers
                .set(r0, self.registers.get(r1)? & self.registers.get(r2)?)?;
        }
        self.registers.update_flags(r0)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        constants::{FL_NEG, FL_POS, FL_ZRO},
        memory::Memory,
        operations::op_and::OpAnd,
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
    fn op_and_with_registers() {
        let mut vm = create_vm();
        vm.registers.set(1, 0b1100).unwrap();
        vm.registers.set(2, 0b1010).unwrap();

        let instr: u16 = 0b0101_0000_0100_0010;
        vm.op_and(instr).unwrap();

        assert_eq!(vm.registers.get(0).unwrap(), 0b1000);
    }

    #[test]
    fn op_and_with_immediate() {
        let mut vm = create_vm();
        vm.registers.set(1, 0b1100).unwrap();

        let instr: u16 = 0b0101_0000_0110_0101;
        vm.op_and(instr).unwrap();

        assert_eq!(vm.registers.get(0).unwrap(), 0b0100);
    }

    #[test]
    fn op_and_with_zero_result() {
        let mut vm = create_vm();
        vm.registers.set(1, 0b1100).unwrap();

        let instr: u16 = 0b0101_0000_0110_0110;
        vm.op_and(instr).unwrap();

        assert_eq!(vm.registers.get(0).unwrap(), 0b0100);
        assert_eq!(vm.registers.cond, FL_POS);
    }

    #[test]
    fn op_and_with_negative_result() {
        let mut vm = create_vm();
        vm.registers.set(1, 0xFFFF).unwrap();

        let instr: u16 = 0b0101_0000_0111_1111;
        vm.op_and(instr).unwrap();

        assert_eq!(vm.registers.get(0).unwrap(), 0xFFFF);
        assert_eq!(vm.registers.cond, FL_NEG);
    }

    #[test]
    fn op_and_with_positive_result() {
        let mut vm = create_vm();
        vm.registers.set(1, 0b0111).unwrap();
        vm.registers.set(2, 0b0011).unwrap();

        let instr: u16 = 0b0101_0000_0100_0010;
        vm.op_and(instr).unwrap();

        assert_eq!(vm.registers.get(0).unwrap(), 0b0011);
        assert_eq!(vm.registers.cond, FL_POS);
    }

    #[test]
    fn op_and_with_zero_cond_flag() {
        let mut vm = create_vm();
        vm.registers.set(1, 0b0101).unwrap();
        vm.registers.set(2, 0b1010).unwrap();

        let instr: u16 = 0b0101_0000_0100_0010;
        vm.op_and(instr).unwrap();

        assert_eq!(vm.registers.get(0).unwrap(), 0b0000);
        assert_eq!(vm.registers.cond, FL_ZRO);
    }
}
