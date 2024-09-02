use std::{fs::File, io::Error, io::Read};

use crate::constants::MEMORY_SIZE;
use crate::memory::Memory;

fn swap(x: u16) -> u16 {
    (x << 8) | (x >> 8)
}

// pub fn read_image_file(path: &str, memory: &mut Memory) -> Result<(), Error> {
//     let mut file = File::open(path)?;

//     let mut origin_buffer = [0u8; 2];
//     file.read_exact(&mut origin_buffer)?;

//     let origin = swap(u16::from_be_bytes(origin_buffer));

//     let max_read = MEMORY_SIZE - origin as usize;

//     let mut instruction_buffer = vec![0u8; max_read * 2];

//     let read_bytes = file.read(&mut instruction_buffer)?;

//     for (i, chunk) in instruction_buffer.chunks(2).enumerate() {
//         if i >= read_bytes / 2 {
//             break;
//         }
//         let instruction = swap(u16::from_be_bytes([chunk[0], chunk[1]]));
//         memory.write((origin as usize + i) as u16, instruction);
//     }

//     Ok(())
// }

// void read_image_file(FILE* file)
// {
//     /* the origin tells us where in memory to place the image */
//     uint16_t origin;
//     fread(&origin, sizeof(origin), 1, file);
//     origin = swap16(origin);

//     /* we know the maximum file size so we only need one fread */
//     uint16_t max_read = MEMORY_MAX - origin;
//     uint16_t* p = memory + origin;
//     size_t read = fread(p, sizeof(uint16_t), max_read, file);

//     /* swap to little endian */
//     while (read-- > 0)
//     {
//         *p = swap16(*p);
//         ++p;
//     }
// }

pub fn read_image_file(path: &str, memory: &mut Memory) -> Result<(), Error> {
    let mut file = File::open(path)?;

    let mut origin_buf = [0u8; 2];
    file.read_exact(&mut origin_buf)?;
    let origin = u16::from_be_bytes(origin_buf) as usize;

    let max_read = MEMORY_SIZE - origin;

    // when reading from a file, you read bytes (u8), which then need to be converted back into u16 to be stored in memory
    let mut buffer = vec![0u8; max_read * 2];
    
    let bytes_read = file.read(&mut buffer)?;

    for i in 0..(bytes_read / 2) {
        let byte_pair = [buffer[2 * i], buffer[2 * i + 1]];
        memory.memory[origin as usize + i] = u16::from_be_bytes(byte_pair);
    }

    Ok(())
}
