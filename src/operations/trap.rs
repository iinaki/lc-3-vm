use std::io::Read;

use crate::{
    constants::{TRAP_GETC, TRAP_IN, TRAP_OUT, TRAP_PUTS, TRAP_PUTSP},
    memory::Memory,
    register::Register,
};

use super::{flush_stdout, update_flags};

// TRAP PUTS {
//     /* one char per word */
//     uint16_t* c = memory + reg[R_R0];
//     while (*c)
//     {
//         putc((char)*c, stdout);
//         ++c;
//     }
//     fflush(stdout);
// }

fn trap_puts(register: &mut Register, memory: &mut Memory) {
    let mut c = memory.read(register.r0);
    while memory.read(c) != 0 {
        print!("{}", memory.read(c) as u8 as char);
        c += 1;
    }
    flush_stdout();
}

// /* read a single ASCII char */
// reg[R_R0] = (uint16_t)getchar();
// update_flags(R_R0);

pub fn trap_getc(register: &mut Register) {
    let mut buffer = [0; 1];
    register.r0 = match std::io::stdin().read_exact(&mut buffer) {
        Ok(_) => buffer[0] as u16,
        Err(e) => {
            println!("Error reading from stdin: {}", e);
            0
        }
    };
    update_flags(register, register.r0);
}

// TRAP OUT
// putc((char)reg[R_R0], stdout);
// fflush(stdout);

fn trap_out(register: &mut Register) {
    print!("{}", register.r0 as u8 as char);
    flush_stdout();
}

// TRAP IN {
//     printf("Enter a character: ");
//     char c = getchar();
//     putc(c, stdout);
//     fflush(stdout);
//     reg[R_R0] = (uint16_t)c;
//     update_flags(R_R0);
// }

fn trap_in(register: &mut Register) {
    print!("Enter a character: ");
    let mut buffer = [0; 1];
    let c = match std::io::stdin().read_exact(&mut buffer) {
        Ok(_) => buffer[0] as char,
        Err(e) => {
            println!("Error reading from stdin: {}", e);
            ' '
        }
    };
    print!("{}", c);
    flush_stdout();
    register.r0 = c as u16;

    update_flags(register, register.r0);
}

// TRAP PUTSP
// {
//     /* one char per byte (two bytes per word)
//        here we need to swap back to
//        big endian format */
//     uint16_t* c = memory + reg[R_R0];
//     while (*c)
//     {
//         char char1 = (*c) & 0xFF;
//         putc(char1, stdout);
//         char char2 = (*c) >> 8;
//         if (char2) putc(char2, stdout);
//         ++c;
//     }
//     fflush(stdout);
// }

fn trap_putsp(register: &mut Register, memory: &mut Memory) {
    let mut c = memory.read(register.r0);
    while memory.read(c) != 0 {
        let char1 = memory.read(c) & 0xFF;
        print!("{}", char1 as u8 as char);
        let char2 = memory.read(c) >> 8;
        if char2 != 0 {
            print!("{}", char2 as u8 as char);
        }
        c += 1;
    }
    flush_stdout();
}

// TRAP HALt
// puts("HALT");
// fflush(stdout);
// running = 0;

pub fn trap_halt(running: &mut bool) {
    println!("HALT");
    flush_stdout();
    *running = false;
}

pub fn handle_trap(register: &mut Register, instr: u16, memory: &mut Memory, running: &mut bool) {
    register.r7 = register.pc;

    let trap_instr = instr & 0xFF;
    match trap_instr {
        TRAP_GETC => {
            // @{TRAP GETC}
            trap_getc(register);
        }
        TRAP_OUT => {
            // @{TRAP OUT}
            trap_out(register);
        }
        TRAP_PUTS => {
            // @{TRAP PUTS}
            trap_puts(register, memory);
        }
        TRAP_IN => {
            // @{TRAP IN}
            trap_in(register);
        }
        TRAP_PUTSP => {
            // @{TRAP PUTSP}
            trap_putsp(register, memory);
        }
        _ => {
            // @{TRAP HALT} or @{BAD TRAP}
            trap_halt(running);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn trap_getc_with_input(register: &mut Register, input: &mut dyn std::io::Read) {
        let mut buffer = [0; 1];
        register.r0 = match input.read_exact(&mut buffer) {
            Ok(_) => buffer[0] as u16,
            Err(e) => {
                println!("Error reading from input: {}", e);
                0
            }
        };
        update_flags(register, register.r0);
    }

    #[test]
    fn test_trap_getc_valid_input() {
        let mut register = Register::new();
        let mut input = Cursor::new(vec![b'A']);

        trap_getc_with_input(&mut register, &mut input);
        assert_eq!(register.r0, b'A' as u16);
    }

    #[test]
    fn test_trap_getc_invalid_input() {
        let mut register = Register::new();
        let mut input = Cursor::new(vec![]); 

        trap_getc_with_input(&mut register, &mut input);
        assert_eq!(register.r0, 0); 
    }
}
