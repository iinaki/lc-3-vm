use std::io::Read;

use crate::{
    constants::{MEMORY_SIZE, MR_KBDR, MR_KBSR},
    vm_error::VmError,
};

/// Represents the memory of the LC-3 virtual machine.
///
/// Contains an array representing the memory of the virtual machine,
/// allowing for reading and writing operations at specific memory addresses.
///
#[derive(Debug)]
pub struct Memory {
    pub memory: [u16; MEMORY_SIZE],
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}

impl Memory {
    /// Creates a new `Memory` instance.
    ///
    /// # Returns
    ///
    /// A `Memory` instance with all memory cells set to `0`.
    ///
    pub fn new() -> Memory {
        Memory {
            memory: [0; MEMORY_SIZE],
        }
    }

    /// Reads the value stored at the specified memory address.
    ///
    /// If the address corresponds to the keyboard status register (`MR_KBSR`), this method
    /// checks for input from the standard input. If a character is available, it updates
    /// the keyboard status register and the keyboard data register (`MR_KBDR`) accordingly.
    ///
    /// # Arguments
    ///
    /// * `address` - A `u16` value representing the memory address to read from.
    ///
    /// # Returns
    ///
    /// The value stored at the specified memory address. If the address corresponds to
    /// `MR_KBSR` and an error occurs while reading from standard input, the function
    /// will return a `VmError`.
    ///
    pub fn read(&mut self, address: u16) -> Result<u16, VmError> {
        if address == MR_KBSR {
            let mut buffer = [0; 1];
            std::io::stdin()
                .read_exact(&mut buffer)
                .map_err(|e| VmError::FailedToReadStdin(e.to_string()))?;
            let char = buffer[0] as u16;

            if char == 0 {
                self.memory[MR_KBSR as usize] = 0;
            } else {
                self.memory[MR_KBSR as usize] = 1 << 15;
                self.memory[MR_KBDR as usize] = char;
            }
        }
        Ok(self.memory[address as usize])
    }

    /// Writes a value to the specified memory address.
    ///
    /// # Arguments
    ///
    /// * `address` - A `u16` value representing the memory address to write to.
    /// * `val` - The value to store at the specified memory address.
    ///
    pub fn write(&mut self, address: u16, val: u16) {
        self.memory[address as usize] = val;
    }
}
