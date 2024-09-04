use byteorder::{BigEndian, ReadBytesExt};
use std::io::{BufReader, Write};
use std::{fs::File, io::Error};

use crate::memory::Memory;

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
