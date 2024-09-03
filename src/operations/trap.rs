use std::io::Read;

use crate::{
    constants::{TRAP_GETC, TRAP_IN, TRAP_OUT, TRAP_PUTS, TRAP_PUTSP},
    memory::Memory,
    registers::Registers,
    utils::{flush_stdout, update_flags},
};

pub fn trap_getc(registers: &mut Registers) {
    let mut buffer = [0; 1];
    registers.r0 = match std::io::stdin().read(&mut buffer) {
        Ok(_) => buffer[0] as u16,
        Err(e) => {
            println!("Error reading from stdin: {}", e);
            flush_stdout();
            0
        }
    };
    update_flags(registers, 0);
}

fn trap_out(registers: &mut Registers) {
    let ch = char::from((registers.r0 & 0xFF) as u8);
    print!("{}", ch);
    flush_stdout();
}

fn trap_puts(registers: &mut Registers, memory: &mut Memory) {
    let mut i = registers.r0;
    let mut c = memory.read(i);
    while c != 0 {
        print!("{}", (c as u8) as char);
        i += 1;
        c = memory.read(i);
    }
    flush_stdout();
}

fn trap_in(registers: &mut Registers) {
    print!("Enter a character: ");
    flush_stdout();

    let mut buffer = [0; 1];
    let c = match std::io::stdin().read(&mut buffer) {
        Ok(_) => buffer[0] as char,
        Err(e) => {
            println!("Error reading from stdin: {}", e);
            flush_stdout();
            ' '
        }
    };
    print!("{}", c);
    flush_stdout();
    registers.r0 = c as u16;

    update_flags(registers, 0);
}

fn trap_putsp(registers: &mut Registers, memory: &mut Memory) {
    let mut i = registers.r0;
    let mut char = memory.read(i);
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
        char = memory.read(i);
    }

    flush_stdout();
}

pub fn trap_halt(running: &mut bool) {
    println!("HALT");
    flush_stdout();
    *running = false;
}

pub fn handle_trap(registers: &mut Registers, instr: u16, memory: &mut Memory, running: &mut bool) {
    registers.r7 = registers.pc;

    let trap_instr = instr & 0xFF;
    match trap_instr {
        TRAP_GETC => {
            trap_getc(registers);
        }
        TRAP_OUT => {
            trap_out(registers);
        }
        TRAP_PUTS => {
            trap_puts(registers, memory);
        }
        TRAP_IN => {
            trap_in(registers);
        }
        TRAP_PUTSP => {
            trap_putsp(registers, memory);
        }
        _ => {
            trap_halt(running);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

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
        update_flags(registers, 0);
    }

    #[test]
    fn trap_getc_valid_input() {
        let mut registers = Registers::new();
        let mut input = Cursor::new(vec![b'A']);

        trap_getc_with_input(&mut registers, &mut input);
        assert_eq!(registers.r0, b'A' as u16);
    }

    #[test]
    fn trap_getc_invalid_input() {
        let mut registers = Registers::new();
        let mut input = Cursor::new(vec![]);

        trap_getc_with_input(&mut registers, &mut input);
        assert_eq!(registers.r0, 0);
    }

    // TRAP OUT
    #[test]
    fn trap_out_prints_a() {
        let mut registers = Registers::new();
        registers.r0 = 'A' as u16;

        trap_out(&mut registers);
        // prints 'A' in stdout
    }

    // TRAP PUTS
    #[test]
    fn trap_puts_prints_string() {
        let mut registers = Registers::new();
        let mut memory = Memory::new();

        let message = "Hello";
        for (i, &byte) in message.as_bytes().iter().enumerate() {
            memory.write(i as u16, byte as u16);
        }
        memory.write(message.len() as u16, 0);

        registers.r0 = 0;

        trap_puts(&mut registers, &mut memory);
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
        flush_stdout();
        registers.set(0, c as u16);

        update_flags(registers, 0);
    }

    // TRAP IN
    #[test]
    fn trap_in() {
        let mut registers = Registers::new();
        let mut input = Cursor::new(vec![b'F']);

        trap_in_with_input(&mut registers, &mut input);
        assert_eq!(registers.r0, b'F' as u16);
        // And prints correctly
    }

    // TRAP PUTSP
    #[test]
    fn trap_putsp_prints_ab() {
        let mut registers = Registers::new();
        let mut memory = Memory::new();

        memory.write(0x3000, 0x4241); // "AB" -> 0x4241
        memory.write(0x3001, 0x0000); // null terminator

        registers.r0 = 0x3000;

        trap_putsp(&mut registers, &mut memory);
        // output: "AB"
    }
}
