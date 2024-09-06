use crate::{utils::sign_extend, vm::Vm, vm_error::VmError};

pub trait OpStr {
    fn op_str(&mut self, instr: u16) -> Result<(), VmError>;
}

impl OpStr for Vm {
    /// Executes the STR operation.
    ///
    /// Stores the value from a specified register into memory.
    /// The target memory address is calculated by adding a signed offset to the value from another register.
    ///
    /// # Parameters
    ///
    /// - `instr`: A 16-bit instruction.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the operation was successful, otherwise returns a `VmError`.
    ///
    fn op_str(&mut self, instr: u16) -> Result<(), VmError> {
        let r0 = (instr >> 9) & 0x7;
        let r1 = (instr >> 6) & 0x7;
        let offset = sign_extend(instr & 0x3F, 6);
        self.memory.write(
            (self.registers.get(r1)? as i16 + offset) as u16,
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
    fn op_str_basic() {
        let mut vm = create_vm();

        vm.registers.set(0, 0xABCD).unwrap();
        vm.registers.set(1, 0x3000).unwrap();

        let instr: u16 = 0b0111_0000_0100_0010; // STR R0, R1, #2
        vm.op_str(instr).unwrap();

        assert_eq!(vm.memory.read(0x3002).unwrap(), 0xABCD);
    }

    #[test]
    fn op_str_negative_offset() {
        let mut vm = create_vm();

        vm.registers.set(0, 0x1234).unwrap();
        vm.registers.set(1, 0x3004).unwrap();

        let instr: u16 = 0b0111_0000_0111_1110; // STR R0, R1, #-2
        vm.op_str(instr).unwrap();

        assert_eq!(vm.memory.read(0x3002).unwrap(), 0x1234);
    }

    #[test]
    fn op_str_zero_offset() {
        let mut vm = create_vm();

        vm.registers.set(0, 0x5678).unwrap();
        vm.registers.set(1, 0x4000).unwrap();

        let instr: u16 = 0b0111_0000_0100_0000; // STR R0, R1, #0
        vm.op_str(instr).unwrap();

        assert_eq!(vm.memory.read(0x4000).unwrap(), 0x5678);
    }

    #[test]
    fn op_str_large_offset() {
        let mut vm = create_vm();

        vm.registers.set(0, 0x9ABC).unwrap();
        vm.registers.set(1, 0x1000).unwrap();

        let instr: u16 = 0b0111_0000_0100_1111; // STR R0, R1, #15
        vm.op_str(instr).unwrap();

        assert_eq!(vm.memory.read(0x100F).unwrap(), 0x9ABC);
    }

    #[test]
    fn op_str_overflow_address() {
        let mut vm = create_vm();

        vm.registers.set(0, 0x4321).unwrap();
        vm.registers.set(1, 0xFFFF).unwrap();

        let instr: u16 = 0b0111_0000_0100_0001; // STR R0, R1, #1
        vm.op_str(instr).unwrap();

        assert_eq!(vm.memory.read(0x0000).unwrap(), 0x4321);
    }
}
