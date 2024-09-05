use byteorder::{BigEndian, ReadBytesExt};
use std::fs::File;
use std::io::{BufReader, Write};

use crate::constants::{FL_NEG, FL_POS, FL_ZRO};
use crate::memory::Memory;
use crate::registers::Registers;
use crate::vm_error::VmError;

/// Reads an image file into memory. The image file is expected to start with
/// a 16-bit address indicating where in memory the data should be loaded, followed by
/// 16-bit instructions to be stored sequentially in memory.
///
/// # Returns
///
/// An `Ok` result if the image file reading was successful, otherwise a `VmError`.
pub fn read_image_file(path: &str, memory: &mut Memory) -> Result<(), VmError> {
    let file = File::open(path).map_err(|e| VmError::FailedToOpenFile(e.to_string()))?;
    let mut reader = BufReader::new(file);

    let mut address = reader
        .read_u16::<BigEndian>()
        .map_err(|e| VmError::FailedToReadBigEndian(e.to_string()))?;
    while let Ok(instr) = reader.read_u16::<BigEndian>() {
        memory.write(address, instr);
        address = match address.checked_add(1) {
            Some(a) => a,
            None => {
                return Err(VmError::FailedToReadBigEndian(
                    "Address overflow while reading image file".to_string(),
                ));
            }
        };
    }

    Ok(())
}

/// Flushes the stdout buffer
///
/// # Returns
///
/// An `Ok` result if the operation was successful, otherwise a `VmError`.
///
pub fn flush_stdout() -> Result<(), VmError> {
    std::io::stdout()
        .flush()
        .map_err(|e| VmError::FailedToFlush(e.to_string()))
}

/// Updates the condition flags in the `Registers` struct.
///
/// # Returns
///
/// An `Ok` result if the operation was successful, otherwise a `VmError`.
///
pub fn update_flags(registers: &mut Registers, r: u16) -> Result<(), VmError> {
    if registers.get(r)? == 0 {
        registers.cond = FL_ZRO;
    } else if (registers.get(r)? >> 15) & 1 == 1 {
        registers.cond = FL_NEG;
    } else {
        registers.cond = FL_POS;
    }
    Ok(())
}

/// Sign-extends a value based on a given bit count.
pub fn sign_extend(x: u16, bit_count: u16) -> i16 {
    let y = if (x >> (bit_count - 1)) & 1 != 0 {
        x | (0xFFFF << bit_count)
    } else {
        x
    };
    y as i16
}
