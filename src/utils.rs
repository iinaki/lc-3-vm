use byteorder::{BigEndian, ReadBytesExt};
use std::io::{BufReader, Write};
use std::{fs::File, io::Error};

use crate::constants::{FL_NEG, FL_POS, FL_ZRO};
use crate::memory::Memory;
use crate::registers::Registers;

pub fn read_image_file(path: &str, memory: &mut Memory) -> Result<(), Error> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    let mut address = reader.read_u16::<BigEndian>()?;
    while let Ok(instr) = reader.read_u16::<BigEndian>() {
        memory.write(address, instr);
        address = match address.checked_add(1) {
            Some(a) => a,
            None => {
                return Err(Error::new(std::io::ErrorKind::Other, "Address overflow"));
            }
        };
    }

    Ok(())
}

/// Flushes the stdout buffer
pub fn flush_stdout() {
    match std::io::stdout().flush() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error flushing stdout: {}", e);
        }
    };
}

pub fn update_flags(registers: &mut Registers, r: u16) {
    if registers.get(r) == 0 {
        registers.cond = FL_ZRO;
    } else if (registers.get(r) >> 15) & 1 == 1 {
        registers.cond = FL_NEG;
    } else {
        registers.cond = FL_POS;
    }
}

pub fn sign_extend(x: u16, bit_count: u16) -> i16 {
    let y = if (x >> (bit_count - 1)) & 1 != 0 {
        x | (0xFFFF << bit_count)
    } else {
        x
    };
    y as i16
}
