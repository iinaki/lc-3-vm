use crate::{utils::sign_extend, vm::Vm, vm_error::VmError};

pub trait OpLdr {
    fn op_ldr(&mut self, instr: u16) -> Result<(), VmError>;
}

impl OpLdr for Vm {
    /// Executes the LDR operation.
    ///
    /// Performs a load operation using a base register and an offset. The final
    /// address is computed by adding a sign-extended offset to the value in the base register.
    /// The value at this computed address is loaded into the destination register. The condition
    /// flags are updated based on the loaded value.
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
    fn op_ldr(&mut self, instr: u16) -> Result<(), VmError> {
        let r0 = (instr >> 9) & 0x7;
        let r1 = (instr >> 6) & 0x7;
        let offset = sign_extend(instr & 0x3F, 6);
        let addr = (self.registers.get(r1)? as i16 + offset) as u16;
        self.registers.set(r0, self.memory.read(addr)?)?;
        self.registers.update_flags(r0)
    }
}

#[cfg(test)]
mod tests {
    use crate::{constants::FL_ZRO, memory::Memory, registers::Registers};

    use super::*;

    fn create_vm() -> Vm {
        Vm {
            registers: Registers::new(),
            memory: Memory::new(),
        }
    }

    #[test]
    fn op_ldr_positive_offset() {
        let mut vm = create_vm();

        vm.registers.set(1, 0x3000).unwrap();
        vm.memory.write(0x3002, 0xABCD);

        let instr: u16 = 0b0110_0000_0100_0010; // LDR R0, R1, #2
        vm.op_ldr(instr).unwrap();

        assert_eq!(vm.registers.get(0).unwrap(), 0xABCD);
    }

    #[test]
    fn op_ldr_negative_offset() {
        let mut vm = create_vm();

        vm.registers.set(1, 0x3002).unwrap();
        vm.memory.write(0x3000, 0x5678);

        let instr: u16 = 0b0110_0000_0111_1110; // LDR R0, R1, #-2
        vm.op_ldr(instr).unwrap();

        assert_eq!(vm.registers.get(0).unwrap(), 0x5678);
    }

    #[test]
    fn op_ldr_zero_offset() {
        let mut vm = create_vm();

        vm.registers.set(1, 0x3000).unwrap();
        vm.memory.write(0x3000, 0x9ABC);

        let instr: u16 = 0b0110_0000_0100_0000; // LDR R0, R1, #0
        vm.op_ldr(instr).unwrap();

        assert_eq!(vm.registers.get(0).unwrap(), 0x9ABC);
    }

    #[test]
    fn op_ldr_update_flags() {
        let mut vm = create_vm();

        vm.registers.set(1, 0x3000).unwrap();
        vm.memory.write(0x3000, 0x0000);

        let instr: u16 = 0b0110_0000_0100_0000; // LDR R0, R1, #0
        vm.op_ldr(instr).unwrap();

        assert_eq!(vm.registers.get(0).unwrap(), 0x0000);
        assert_eq!(vm.registers.cond, FL_ZRO);
    }

    #[test]
    fn op_ldr_preserves_pc() {
        let mut vm = create_vm();

        vm.registers.set(1, 0x3000).unwrap();

        let instr: u16 = 0b0110_0000_0100_0010; // LDR R0, R1, #2
        let initial_pc = vm.registers.pc;
        vm.op_ldr(instr).unwrap();

        assert_eq!(vm.registers.pc, initial_pc);
    }
}
