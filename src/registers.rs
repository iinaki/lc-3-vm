use crate::{
    constants::{FL_NEG, FL_POS, FL_ZRO, PC_START},
    vm_error::VmError,
};

/// Represents the registers of the LC-3 virtual machine.
///
/// # Fields
///
/// * `r0`, `r1`, `r2`, `r3`, `r4`, `r5`, `r6`, `r7` - General-purpose registers.
/// * `pc` - The program counter, which holds the address of the next instruction to execute.
/// * `cond` - The condition register, which holds flags indicating the result of the last operation.
///
#[derive(Debug)]
pub struct Registers {
    pub r0: u16,
    pub r1: u16,
    pub r2: u16,
    pub r3: u16,
    pub r4: u16,
    pub r5: u16,
    pub r6: u16,
    pub r7: u16,
    pub pc: u16,
    pub cond: u16,
}

impl Default for Registers {
    fn default() -> Self {
        Self::new()
    }
}

impl Registers {
    /// Creates a new `Registers` instance with initial values. Sets all general-purpose registers to `0`, the program counter to `PC_START`, the condition register to `FL_ZRO`.
    ///
    /// # Returns
    ///
    /// A `Registers` instance initialized to default values.
    ///
    pub fn new() -> Self {
        Registers {
            r0: 0,
            r1: 0,
            r2: 0,
            r3: 0,
            r4: 0,
            r5: 0,
            r6: 0,
            r7: 0,
            pc: PC_START,
            cond: FL_ZRO,
        }
    }

    /// Gets the value of the specified register.
    ///
    /// # Arguments
    ///
    /// * `r` - A `u16` representing the register identifier (0-9).
    ///
    /// # Returns
    ///
    /// The value of the specified register. If the identifier is invalid, it returns a VmError.
    ///
    pub fn get(&self, r: u16) -> Result<u16, VmError> {
        let res = match r {
            0 => self.r0,
            1 => self.r1,
            2 => self.r2,
            3 => self.r3,
            4 => self.r4,
            5 => self.r5,
            6 => self.r6,
            7 => self.r7,
            8 => self.pc,
            9 => self.cond,
            _ => {
                return Err(VmError::InvalidRegister(
                    "Invalid registers at get".to_string(),
                ));
            }
        };
        Ok(res)
    }

    /// Sets the value of the specified register.
    ///
    /// # Arguments
    ///
    /// * `r` - A `u16` representing the register identifier (0-9).
    /// * `val` - The value to be stored in the register.
    ///
    /// # Returns
    ///
    /// An Ok if the operation is successful. If the identifier is invalid, it returns a VmError.
    ///
    pub fn set(&mut self, r: u16, val: u16) -> Result<(), VmError> {
        match r {
            0 => self.r0 = val,
            1 => self.r1 = val,
            2 => self.r2 = val,
            3 => self.r3 = val,
            4 => self.r4 = val,
            5 => self.r5 = val,
            6 => self.r6 = val,
            7 => self.r7 = val,
            8 => self.pc = val,
            9 => self.cond = val,
            _ => {
                return Err(VmError::InvalidRegister(
                    "Invalid registers at set".to_string(),
                ));
            }
        }
        Ok(())
    }

    /// Updates the condition flags in the `Registers` struct.
    ///
    /// # Returns
    ///
    /// An `Ok` result if the operation was successful, otherwise a `VmError` if it fails to get de value of `r`.
    ///
    pub fn update_flags(&mut self, r: u16) -> Result<(), VmError> {
        let r_value = self.get(r)?;

        if r_value == 0 {
            self.cond = FL_ZRO;
        } else if (r_value >> 15) & 1 == 1 {
            self.cond = FL_NEG;
        } else {
            self.cond = FL_POS;
        }

        Ok(())
    }
}
