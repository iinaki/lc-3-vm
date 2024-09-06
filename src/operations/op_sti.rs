use crate::{utils::sign_extend, vm::Vm, vm_error::VmError};

impl Vm {
    /// Executes the STI operation.
    ///
    /// Stores the value from the specified register into memory at an address determined indirectly.
    /// The address is first calculated using a PC-relative offset, and the value from the register is stored at the memory address retrieved from that location.
    ///
    /// # Parameters
    ///
    /// - `instr`: A 16-bit instruction.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the operation was successful, otherwise returns a `VmError`.
    ///
    pub fn op_sti(&mut self, instr: u16) -> Result<(), VmError> {
        let r0 = (instr >> 9) & 0x7;
        let pc_offset = sign_extend(instr & 0x1FF, 9);
        let addr = self
            .memory
            .read((self.registers.pc as i16 + pc_offset) as u16)?;
        self.memory.write(addr, self.registers.get(r0)?);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{memory::Memory, registers::Registers, vm::Vm};

    fn create_vm() -> Vm {
        Vm {
            registers: Registers::new(),
            memory: Memory::new(),
        }
    }

    #[test]
    fn op_sti_basic() {
        let mut vm = create_vm();

        vm.registers.pc = 0x3000;
        vm.registers.set(0, 0x1234).unwrap();

        vm.memory.write(0x3002, 0x4000);

        let instr: u16 = 0b1011_0000_0000_0010; // STI R0, #2
        vm.op_sti(instr).unwrap();

        assert_eq!(vm.memory.read(0x4000).unwrap(), 0x1234);
    }

    #[test]
    fn op_sti_negative_offset() {
        let mut vm = create_vm();

        vm.registers.pc = 0x3000;
        vm.registers.set(0, 0x5678).unwrap();

        vm.memory.write(0x2FFE, 0x5000);

        let instr: u16 = 0b1011_0001_1111_1110; // STI R0, #-2
        vm.op_sti(instr).unwrap();

        assert_eq!(vm.memory.read(0x5000).unwrap(), 0x5678);
    }

    #[test]
    fn op_sti_zero_offset() {
        let mut vm = create_vm();

        vm.registers.pc = 0x3000;
        vm.registers.set(0, 0xABCD).unwrap();

        vm.memory.write(0x3000, 0x6000);

        let instr: u16 = 0b1011_0000_0000_0000; // STI R0, #0
        vm.op_sti(instr).unwrap();

        assert_eq!(vm.memory.read(0x6000).unwrap(), 0xABCD);
    }

    #[test]
    fn op_sti_overflow_offset() {
        let mut vm = create_vm();

        vm.registers.pc = 0xFFFF;
        vm.registers.set(0, 0x4321).unwrap();

        vm.memory.write(0x0000, 0x7000);

        let instr: u16 = 0b1011_0000_0000_0001; // STI R0, #1
        vm.op_sti(instr).unwrap();

        assert_eq!(vm.memory.read(0x7000).unwrap(), 0x4321);
    }

    #[test]
    fn op_sti_preserves_registerss() {
        let mut vm = create_vm();

        vm.registers.pc = 0x3000;
        vm.registers.set(0, 0x7777).unwrap();
        vm.registers.set(1, 0x8888).unwrap();

        vm.memory.write(0x3002, 0x8000);

        let instr: u16 = 0b1011_0000_0000_0010; // STI R0, #2
        vm.op_sti(instr).unwrap();

        assert_eq!(vm.registers.get(1).unwrap(), 0x8888);
    }
}
