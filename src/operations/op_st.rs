use crate::{utils::sign_extend, vm::Vm, vm_error::VmError};

pub trait OpSt {
    fn op_st(&mut self, instr: u16) -> Result<(), VmError>;
}

impl OpSt for Vm {
    /// Executes the ST operation.
    ///
    /// Stores the value from the specified register into memory
    /// at the address computed by adding a PC-relative offset to the current program counter.
    ///
    /// # Parameters
    ///
    /// - `instr`: A 16-bit instruction.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the operation was successful, otherwise returns a `VmError`.
    ///
    fn op_st(&mut self, instr: u16) -> Result<(), VmError> {
        let r0 = (instr >> 9) & 0x7;
        let pc_offset = sign_extend(instr & 0x1FF, 9);
        self.memory.write(
            (self.registers.pc as i16 + pc_offset) as u16,
            self.registers.get(r0)?,
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{memory::Memory, registers::Registers, vm::Vm};

    use super::*;

    fn create_vm() -> Vm {
        Vm {
            registers: Registers::new(),
            memory: Memory::new(),
        }
    }

    #[test]
    fn op_st_basic() {
        let mut vm = create_vm();

        vm.registers.set(0, 0x1234).unwrap();

        let instr: u16 = 0b0011_0000_0000_0010; // ST R0, #2
        vm.op_st(instr).unwrap();

        assert_eq!(vm.memory.read(vm.registers.pc + 2).unwrap(), 0x1234);
    }

    #[test]
    fn op_st_negative_offset() {
        let mut vm = create_vm();

        vm.registers.pc = 0x3000;
        vm.registers.set(0, 0x5678).unwrap();

        let instr: u16 = 0b0011_0001_1111_1110; // ST R0, #-2
        vm.op_st(instr).unwrap();

        assert_eq!(vm.memory.read(0x2FFE).unwrap(), 0x5678);
    }

    #[test]
    fn op_st_zero_offset() {
        let mut vm = create_vm();

        vm.registers.pc = 0x3000;
        vm.registers.set(0, 0xABCD).unwrap();

        let instr: u16 = 0b0011_0000_0000_0000; // ST R0, #0
        vm.op_st(instr).unwrap();

        assert_eq!(vm.memory.read(0x3000).unwrap(), 0xABCD);
    }

    #[test]
    fn op_st_overflow_offset() {
        let mut vm = create_vm();

        vm.registers.pc = 0xFFFF;
        vm.registers.set(0, 0x4321).unwrap();

        let instr: u16 = 0b0011_0000_0000_0001; // ST R0, #1
        vm.op_st(instr).unwrap();

        assert_eq!(vm.memory.read(0x0000).unwrap(), 0x4321);
    }

    #[test]
    fn op_st_preserves_registers() {
        let mut vm = create_vm();

        vm.registers.pc = 0x3000;
        vm.registers.set(0, 0x7777).unwrap();
        vm.registers.set(1, 0x8888).unwrap();

        let instr: u16 = 0b0011_0000_0000_0010; // ST R0, #2
        vm.op_st(instr).unwrap();

        assert_eq!(vm.registers.get(1).unwrap(), 0x8888);
    }
}
