use std::io::Read;

use crate::{
    constants::{TRAP_GETC, TRAP_IN, TRAP_OUT, TRAP_PUTS, TRAP_PUTSP},
    utils::flush_stdout,
    vm::Vm,
    vm_error::VmError,
};

impl Vm {
    /// Handles the `GETC` TRAP instruction.
    ///
    /// This function reads a single character from the standard input (stdin)
    /// and stores it in the `R0` register. The condition flags are updated
    /// based on the value of `R0`.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the operation was successful, otherwise returns a `VmError`.
    ///
    fn trap_getc(&mut self) -> Result<(), VmError> {
        let mut buffer = [0; 1];
        std::io::stdin()
            .read(&mut buffer)
            .map_err(|e| VmError::FailedToReadStdin(e.to_string()))?;
        self.registers.r0 = buffer[0] as u16;
        self.registers.update_flags(0)
    }

    /// Handles the `OUT` TRAP instruction.
    ///
    /// This function outputs a single character stored in the `R0` register to the console.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the operation was successful, otherwise returns a `VmError`.
    ///
    fn trap_out(&mut self) -> Result<(), VmError> {
        let ch = char::from((self.registers.r0 & 0xFF) as u8);
        print!("{}", ch);
        flush_stdout()
    }

    /// Handles the `PUTS` TRAP instruction.
    ///
    /// This function outputs a null-terminated string stored in memory,
    /// starting from the address in the `R0` register, to the console.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the operation was successful, otherwise returns a `VmError`.
    ///
    fn trap_puts(&mut self) -> Result<(), VmError> {
        let mut i = self.registers.r0;
        let mut c = self.memory.read(i)?;
        while c != 0 {
            print!("{}", (c as u8) as char);
            i += 1;
            c = self.memory.read(i)?;
        }
        flush_stdout()
    }

    /// Handles the `IN` TRAP instruction.
    ///
    /// This function prompts the user to enter a character and stores it in the `R0` register.
    /// The entered character is also echoed to the console.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the operation was successful, otherwise returns a `VmError`.
    ///
    fn trap_in(&mut self) -> Result<(), VmError> {
        print!("Enter a character: ");
        flush_stdout()?;

        let mut buffer = [0; 1];
        std::io::stdin()
            .read(&mut buffer)
            .map_err(|e| VmError::FailedToReadStdin(e.to_string()))?;

        let c = buffer[0] as char;

        print!("{}", c);
        flush_stdout()?;
        self.registers.r0 = c as u16;

        self.registers.update_flags(0)
    }

    /// Handles the `PUTSP` TRAP instruction.
    ///
    /// This function outputs a string stored in memory, starting from the address in the `R0` register,
    /// to the console. Each memory location contains two characters packed into one 16-bit word.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the operation was successful, otherwise returns a `VmError`.
    ///
    fn trap_putsp(&mut self) -> Result<(), VmError> {
        let mut i = self.registers.r0;
        let mut char = self.memory.read(i)?;
        while char != 0 {
            let char1 = (char & 0xFF) as u8 as char;
            if char1 == '\0' {
                break;
            }
            print!("{}", char1);

            let char2 = (char >> 8) as u8 as char;
            if char2 != '\0' {
                print!("{}", char2 as u8 as char);
            }
            i = i.wrapping_add(1);
            char = self.memory.read(i)?;
        }

        flush_stdout()
    }

    /// Handles the `HALT` TRAP instruction.
    ///
    /// This function halts the program by setting the `running` flag to `false`.
    ///
    /// # Parameters
    ///
    /// - `running`: A mutable reference to a boolean flag that indicates if the
    ///   program is running.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the operation was successful, otherwise returns a `VmError`.
    ///
    pub fn trap_halt(&self, running: &mut bool) -> Result<(), VmError> {
        println!("HALT");
        *running = false;
        flush_stdout()
    }

    /// Handles the correct trap routine based on the instruction.
    ///
    /// # Parameters
    ///
    /// - `instr`: The 16-bit instruction containing the TRAP opcode.
    /// - `running`: A mutable reference to a boolean flag that indicates if the
    ///   program is running.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the handling was successful, otherwise returns a `VmError`.
    ///
    pub fn handle_trap(&mut self, instr: u16, running: &mut bool) -> Result<(), VmError> {
        self.registers.r7 = self.registers.pc;
        let trap_instr = instr & 0xFF;
        match trap_instr {
            TRAP_GETC => self.trap_getc(),
            TRAP_OUT => self.trap_out(),
            TRAP_PUTS => self.trap_puts(),
            TRAP_IN => self.trap_in(),
            TRAP_PUTSP => self.trap_putsp(),
            _ => self.trap_halt(running),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{memory::Memory, registers::Registers};

    use super::*;
    use std::io::Cursor;

    fn create_vm() -> Vm {
        Vm {
            registers: Registers::new(),
            memory: Memory::new(),
        }
    }

    // TRAP GETC
    fn trap_getc_with_input(registers: &mut Registers, input: &mut dyn std::io::Read) {
        let mut buffer = [0; 1];
        registers.r0 = match input.read_exact(&mut buffer) {
            Ok(_) => buffer[0] as u16,
            Err(e) => {
                println!("Error reading from input: {}", e);
                0
            }
        };
        registers.update_flags(0).unwrap();
    }

    #[test]
    fn trap_getc_valid_input() {
        let mut vm = create_vm();
        let mut input = Cursor::new(vec![b'A']);

        trap_getc_with_input(&mut vm.registers, &mut input);
        assert_eq!(vm.registers.r0, b'A' as u16);
    }

    #[test]
    fn trap_getc_invalid_input() {
        let mut vm = create_vm();
        let mut input = Cursor::new(vec![]);

        trap_getc_with_input(&mut vm.registers, &mut input);
        assert_eq!(vm.registers.r0, 0);
    }

    // TRAP OUT
    #[test]
    fn trap_out_prints_a() {
        let mut vm = create_vm();
        vm.registers.r0 = 'A' as u16;

        vm.trap_out().unwrap();
        // prints 'A' in stdout
    }

    // TRAP PUTS
    #[test]
    fn trap_puts_prints_string() {
        let mut vm = create_vm();

        let message = "Hello";
        for (i, &byte) in message.as_bytes().iter().enumerate() {
            vm.memory.write(i as u16, byte as u16);
        }
        vm.memory.write(message.len() as u16, 0);

        vm.registers.r0 = 0;

        vm.trap_puts().unwrap();
        // prints 'Hello' in stdout
    }

    // TRAP IN
    fn trap_in_with_input(registers: &mut Registers, input: &mut dyn std::io::Read) {
        print!("Enter a character: ");
        let mut buffer = [0; 1];
        let c = match input.read_exact(&mut buffer) {
            Ok(_) => buffer[0] as char,
            Err(e) => {
                println!("Error reading from stdin: {}", e);
                ' '
            }
        };
        print!("{}", c);
        flush_stdout().unwrap();
        registers.set(0, c as u16).unwrap();

        registers.update_flags(0).unwrap();
    }

    // TRAP IN
    #[test]
    fn trap_in() {
        let mut vm = create_vm();
        let mut input = Cursor::new(vec![b'F']);

        trap_in_with_input(&mut vm.registers, &mut input);
        assert_eq!(vm.registers.r0, b'F' as u16);
        // And prints correctly
    }

    // TRAP PUTSP
    #[test]
    fn trap_putsp_prints_ab() {
        let mut vm = create_vm();

        vm.memory.write(0x3000, 0x4241); // "AB" -> 0x4241
        vm.memory.write(0x3001, 0x0000); // null terminator

        vm.registers.r0 = 0x3000;

        vm.trap_putsp().unwrap();
        // output: "AB"
    }
}
