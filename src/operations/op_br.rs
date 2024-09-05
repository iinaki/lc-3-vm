use crate::{utils::sign_extend, vm::Vm};

pub trait OpBr {
    fn op_br(&mut self, instr: u16);
}

impl OpBr for Vm {
    /// Executes the BRANCH operation.
    ///
    /// Conditionally updates the program counter based on the
    /// specified condition flags. If the condition is met, the program counter
    /// is adjusted by the sign-extended offset to branch to a new location.
    ///
    /// # Parameters
    ///
    /// - `registers`: A mutable reference to the `Registers` struct.
    /// - `instr`: A 16-bit instruction.
    ///
    fn op_br(&mut self, instr: u16) {
        let pc_offset = sign_extend(instr & 0x1FF, 9);
        let cond_flag = (instr >> 9) & 0x7;
        if cond_flag & self.registers.cond != 0 {
            self.registers.pc = ((self.registers.pc as i16) + pc_offset) as u16;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        constants::FL_POS, memory::Memory, operations::op_br::OpBr, registers::Registers, vm::Vm,
    };

    fn create_vm() -> Vm {
        Vm {
            registers: Registers::new(),
            memory: Memory::new(),
        }
    }

    // BR TESTS
    #[test]
    fn br_branch_taken_positive_offset() {
        let mut vm = create_vm();
        vm.registers.cond = FL_POS;

        let instr: u16 = 0b0000_0010_0000_0101;
        vm.op_br(instr);

        assert_eq!(vm.registers.pc, 0x3005);
    }

    #[test]
    fn br_branch_not_taken() {
        let mut vm = create_vm();
        vm.registers.cond = FL_POS;

        let instr: u16 = 0b0000_0100_0000_0101;
        vm.op_br(instr);

        assert_eq!(vm.registers.pc, 0x3000);
    }

    #[test]
    fn br_branch_taken_negative_offset() {
        let mut vm = create_vm();
        vm.registers.cond = FL_POS;

        let instr: u16 = 0b0000_0011_1111_1011;
        vm.op_br(instr);

        assert_eq!(vm.registers.pc, 0x2FFB);
    }

    #[test]
    fn br_branch_zero_offset() {
        let mut vm = create_vm();
        vm.registers.cond = FL_POS;

        let instr: u16 = 0b0000_0010_0000_0000;
        vm.op_br(instr);

        println!("PC: {}", vm.registers.pc);

        assert_eq!(vm.registers.pc, 0x3000);
    }
}
