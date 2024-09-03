use std::{fs::File, io::Error, io::Read};

use crate::constants::MEMORY_SIZE;
use crate::memory::Memory;

fn swap(x: u16) -> u16 {
    (x << 8) | (x >> 8)
}

pub fn read_image_file(path: &str, memory: &mut Memory) -> Result<(), Error> {
    let mut file = File::open(path)?;

    let mut origin_buffer = [0u8; 2];
    file.read_exact(&mut origin_buffer)?;

    let origin = swap(u16::from_be_bytes(origin_buffer));

    let max_read = MEMORY_SIZE - origin as usize;

    let mut instruction_buffer = vec![0u8; max_read * 2];

    let read_bytes = file.read(&mut instruction_buffer)?;

    for (i, chunk) in instruction_buffer.chunks(2).enumerate() {
        if i >= read_bytes / 2 {
            break;
        }
        let instruction = swap(u16::from_be_bytes([chunk[0], chunk[1]]));
        memory.write((origin as usize + i) as u16, instruction);
    }

    Ok(())
}
