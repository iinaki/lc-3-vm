use crate::{utils::sign_extend, vm::Vm, vm_error::VmError};

pub trait OpLdi {
    fn op_ldi(&mut self, instr: u16) -> Result<(), VmError>;
}

impl OpLdi for Vm {
    /// Executes the LDI operation.
    ///
    /// Performs an indirect load. It first retrieves an address from memory using
    /// a sign-extended offset added to the program counter. Then, it uses that address to load
    /// the final value from memory into the destination register. The condition flags are updated
    /// based on the loaded value.
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
    fn op_ldi(&mut self, instr: u16) -> Result<(), VmError> {
        let r0 = (instr >> 9) & 0x7;
        let pc_offset = sign_extend(instr & 0x1FF, 9);
        let addr = (self.registers.pc as i16 + pc_offset) as u16;
        let indirect_addr = self.memory.read(addr)?;
        self.registers.set(r0, self.memory.read(indirect_addr)?)?;
        self.registers.update_flags(r0)
    }
}

#[cfg(test)]
mod tests {
    use crate::{constants::FL_ZRO, memory::Memory, registers::Registers, vm::Vm};

    use super::*;

    fn create_vm() -> Vm {
        Vm {
            registers: Registers::new(),
            memory: Memory::new(),
        }
    }

    #[test]
    fn op_ldi_positive_offset() {
        let mut vm = create_vm();
        vm.registers.pc = 0x3000;
        vm.memory.write(0x3002, 0x4000);
        vm.memory.write(0x4000, 0x1234);

        let instr: u16 = 0b1010_0000_0000_0010; // LDI R0, PC+2
        vm.op_ldi(instr).unwrap();

        assert_eq!(vm.registers.get(0).unwrap(), 0x1234);
    }

    #[test]
    fn op_ldi_negative_offset() {
        let mut vm = create_vm();

        vm.registers.pc = 0x3000;
        vm.memory.write(0x2FFE, 0x4000);
        vm.memory.write(0x4000, 0xABCD);

        let instr: u16 = 0b1010_0001_1111_1110; // LDI R0, PC-2
        vm.op_ldi(instr).unwrap();

        assert_eq!(vm.registers.get(0).unwrap(), 0xABCD);
    }

    #[test]
    fn op_ldi_zero_offset() {
        let mut vm = create_vm();

        vm.registers.pc = 0x3000;
        vm.memory.write(0x3000, 0x5000);
        vm.memory.write(0x5000, 0x5678);

        let instr: u16 = 0b1010_0000_0000_0000; // LDI R0, PC+0
        vm.op_ldi(instr).unwrap();

        assert_eq!(vm.registers.get(0).unwrap(), 0x5678);
    }

    #[test]
    fn op_ldi_update_flags() {
        let mut vm = create_vm();

        vm.registers.pc = 0x3000;
        vm.memory.write(0x3000, 0x0000);
        vm.memory.write(0x0000, 0x0000);

        let instr: u16 = 0b1010_0000_0000_0000; // LDI R0, PC+0
        vm.op_ldi(instr).unwrap();

        assert_eq!(vm.registers.get(0).unwrap(), 0x0000);
        assert_eq!(vm.registers.cond, FL_ZRO);
    }

    #[test]
    fn op_ldi_preserves_pc() {
        let mut vm = create_vm();

        vm.registers.pc = 0x3000;

        let instr: u16 = 0b1010_0000_0000_0010; // LDI R0, PC+2
        vm.op_ldi(instr).unwrap();

        assert_eq!(vm.registers.pc, 0x3000);
    }
}
