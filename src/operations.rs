use std::io::{Read, Write};

use crate::{
    constants::{
        FL_NEG, FL_POS, FL_ZRO, OP_ADD, OP_AND, OP_BR, OP_JMP, OP_JSR, OP_LD, OP_LDI, OP_LDR,
        OP_LEA, OP_NOT, OP_ST, OP_STI, OP_STR, OP_TRAP, TRAP_GETC, TRAP_IN, TRAP_OUT, TRAP_PUTS,
        TRAP_PUTSP,
    },
    memory::Memory,
    register::Register,
};

// pub fn mem_read(address: u16, memory: &Memory) -> u16 {
//     memory[address as usize] as u16
// }

// uint16_t sign_extend(uint16_t x, int bit_count)
// {
//     if ((x >> (bit_count - 1)) & 1) {
//         x |= (0xFFFF << bit_count);
//     }
//     return x;
// }
fn sign_extend(mut x: u16, bit_count: u16) -> u16 {
    if (x >> (bit_count - 1)) & 1 != 0 {
        x | (0xFFFF << bit_count)
    } else {
        x
    }
}

/// Flushes the stdout buffer
fn flush_stdout() {
    match std::io::stdout().flush() {
        Ok(_) => {}
        Err(e) => {
            println!("Error flushing stdout: {}", e);
        }
    };
}

// void update_flags(uint16_t r)
// {
//     if (reg[r] == 0)
//     {
//         reg[R_COND] = FL_ZRO;
//     }
//     else if (reg[r] >> 15) /* a 1 in the left-most bit indicates negative */
//     {
//         reg[R_COND] = FL_NEG;
//     }
//     else
//     {
//         reg[R_COND] = FL_POS;
//     }
// }
fn update_flags(register: &mut Register, r: u16) {
    if register.get(r) == 0 {
        register.cond = FL_ZRO;
    } else if (register.get(r) >> 15) & 1 == 1 {
        register.cond = FL_NEG;
    } else {
        register.cond = FL_POS;
    }
}

// ADD {
//     /* destination register (DR) */
//     uint16_t r0 = (instr >> 9) & 0x7;
//     /* first operand (SR1) */
//     uint16_t r1 = (instr >> 6) & 0x7;
//     /* whether we are in immediate mode */
//     uint16_t imm_flag = (instr >> 5) & 0x1;

//     if (imm_flag)
//     {
//         uint16_t imm5 = sign_extend(instr & 0x1F, 5);
//         reg[r0] = reg[r1] + imm5;
//     }
//     else
//     {
//         uint16_t r2 = instr & 0x7;
//         reg[r0] = reg[r1] + reg[r2];
//     }

//     update_flags(r0);
// }

fn op_add(register: &mut Register, instr: u16) {
    let r0 = (instr >> 9) & 0x7;
    let r1 = (instr >> 6) & 0x7;
    let imm_flag = (instr >> 5) & 0x1;

    if imm_flag == 1 {
        let imm5 = sign_extend(instr & 0x1F, 5) as i16;
        register.set(r0, (register.get(r1) as i16 + imm5) as u16);
    } else {
        let r2 = instr & 0x7;
        register.set(r0, register.get(r1) + register.get(r2));
        println!("R0: {}", r0);
        println!("R1: {}", r1);
        println!("R2: {}", r2);
    }

    update_flags(register, r0);
}

// AND {
//     uint16_t r0 = (instr >> 9) & 0x7;
//     uint16_t r1 = (instr >> 6) & 0x7;
//     uint16_t imm_flag = (instr >> 5) & 0x1;

//     if (imm_flag)
//     {
//         uint16_t imm5 = sign_extend(instr & 0x1F, 5);
//         reg[r0] = reg[r1] & imm5;
//     }
//     else
//     {
//         uint16_t r2 = instr & 0x7;
//         reg[r0] = reg[r1] & reg[r2];
//     }
//     update_flags(r0);
// }
fn op_and(register: &mut Register, instr: u16) {
    let r0 = (instr >> 9) & 0x7;
    let r1 = (instr >> 6) & 0x7;
    let imm_flag = (instr >> 5) & 0x1;

    if imm_flag == 1 {
        let imm5 = sign_extend(instr & 0x1F, 5);
        register.set(r0, register.get(r1) & imm5);
    } else {
        let r2 = instr & 0x7;
        register.set(r0, register.get(r1) & register.get(r2));
    }
    update_flags(register, r0);
}

// NOT{
//     uint16_t r0 = (instr >> 9) & 0x7;
//     uint16_t r1 = (instr >> 6) & 0x7;
//     reg[r0] = ~reg[r1];
//     update_flags(r0);
// }

fn op_not(register: &mut Register, instr: u16) {
    let r0 = (instr >> 9) & 0x7;
    let r1 = (instr >> 6) & 0x7;
    register.set(r0, !register.get(r1));
    update_flags(register, r0);
}

// BR {
//     uint16_t pc_offset = sign_extend(instr & 0x1FF, 9);
//     uint16_t cond_flag = (instr >> 9) & 0x7;
//     if (cond_flag & reg[R_COND])
//     {
//         reg[R_PC] += pc_offset;
//     }
// }

// fn op_br(register: &mut Register, instr: u16) {
//     let pc_offset = sign_extend(instr & 0x1FF, 9);
//     let cond_flag = (instr >> 9) & 0x7;
//     if cond_flag & register.cond as u16 != 0 {
//         register.pc += pc_offset;
//     }
// }
fn op_br(register: &mut Register, instr: u16) {
    let pc_offset = sign_extend(instr & 0x1FF, 9) as i16; 
    let cond_flag = (instr >> 9) & 0x7;
    if cond_flag & register.cond as u16 != 0 {
        register.pc = ((register.pc as i16) + pc_offset) as u16; 
    }
}

// JMP {
//     /* Also handles RET */
//     uint16_t r1 = (instr >> 6) & 0x7;
//     reg[R_PC] = reg[r1];
// }

fn op_jmp(register: &mut Register, instr: u16) {
    let r1 = (instr >> 6) & 0x7;
    register.pc = register.get(r1);
}

// {
//     uint16_t long_flag = (instr >> 11) & 1;
//     reg[R_R7] = reg[R_PC];
//     if (long_flag)
//     {
//         uint16_t long_pc_offset = sign_extend(instr & 0x7FF, 11);
//         reg[R_PC] += long_pc_offset;  /* JSR */
//     }
//     else
//     {
//         uint16_t r1 = (instr >> 6) & 0x7;
//         reg[R_PC] = reg[r1]; /* JSRR */
//     }
// }

fn op_jsr(register: &mut Register, instr: u16) {
    let long_flag = (instr >> 11) & 1;
    register.r7 = register.pc;
    if long_flag == 1 {
        let long_pc_offset = sign_extend(instr & 0x7FF, 11);
        register.pc += long_pc_offset; /* JSR */
    } else {
        let r1 = (instr >> 6) & 0x7;
        register.pc = register.get(r1); /* JSRR */
    }
}

// LD {
//     uint16_t r0 = (instr >> 9) & 0x7;
//     uint16_t pc_offset = sign_extend(instr & 0x1FF, 9);
//     reg[r0] = mem_read(reg[R_PC] + pc_offset);
//     update_flags(r0);
// }

fn op_ld(register: &mut Register, instr: u16, memory: &mut Memory) {
    let r0 = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1FF, 9);
    register.set(r0, memory.read(r0 + pc_offset));
    update_flags(register, r0);
}

// LDI {
//     /* destination register (DR) */
//     uint16_t r0 = (instr >> 9) & 0x7;
//     /* PCoffset 9*/
//     uint16_t pc_offset = sign_extend(instr & 0x1FF, 9);
//     /* add pc_offset to the current PC, look at that memory location to get the final address */
//     reg[r0] = mem_read(mem_read(reg[R_PC] + pc_offset));
//     update_flags(r0);
// }

fn op_ldi(register: &mut Register, instr: u16, memory: &mut Memory) {
    let r0 = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1FF, 9);
    let addr = memory.read(r0 + pc_offset);

    register.set(r0, memory.read(addr));
    update_flags(register, r0);
}

// LDR {
//     uint16_t r0 = (instr >> 9) & 0x7;
//     uint16_t r1 = (instr >> 6) & 0x7;
//     uint16_t offset = sign_extend(instr & 0x3F, 6);
//     reg[r0] = mem_read(reg[r1] + offset);
//     update_flags(r0);
// }

fn op_ldr(register: &mut Register, instr: u16, memory: &mut Memory) {
    let r0 = (instr >> 9) & 0x7;
    let r1 = (instr >> 6) & 0x7;
    let offset = sign_extend(instr & 0x3F, 6);
    register.set(r0, memory.read(register.get(r1) + offset));
    update_flags(register, r0);
}

// LEA {
//     uint16_t r0 = (instr >> 9) & 0x7;
//     uint16_t pc_offset = sign_extend(instr & 0x1FF, 9);
//     reg[r0] = reg[R_PC] + pc_offset;
//     update_flags(r0);
// }

fn op_lea(register: &mut Register, instr: u16) {
    let r0 = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1FF, 9);
    register.set(r0, register.pc + pc_offset);
    update_flags(register, r0);
}

// ST {
//     uint16_t r0 = (instr >> 9) & 0x7;
//     uint16_t pc_offset = sign_extend(instr & 0x1FF, 9);
//     mem_write(reg[R_PC] + pc_offset, reg[r0]);
// }

fn op_st(register: &mut Register, instr: u16, memory: &mut Memory) {
    let r0 = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1FF, 9);
    //memory[(r0 + pc_offset) as usize] = register.get(r0);
    memory.write(r0 + pc_offset, register.get(r0));
}

// STI {
//     uint16_t r0 = (instr >> 9) & 0x7;
//     uint16_t pc_offset = sign_extend(instr & 0x1FF, 9);
//     mem_write(mem_read(reg[R_PC] + pc_offset), reg[r0]);
// }

fn op_sti(register: &mut Register, instr: u16, memory: &mut Memory) {
    let r0 = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1FF, 9);
    let addr = memory.read(r0 + pc_offset);
    memory.write(addr, register.get(r0));
}

// STR {
//     uint16_t r0 = (instr >> 9) & 0x7;
//     uint16_t r1 = (instr >> 6) & 0x7;
//     uint16_t offset = sign_extend(instr & 0x3F, 6);
//     mem_write(reg[r1] + offset, reg[r0]);
// }

fn op_str(register: &mut Register, instr: u16, memory: &mut Memory) {
    let r0 = (instr >> 9) & 0x7;
    let r1 = (instr >> 6) & 0x7;
    let offset = sign_extend(instr & 0x3F, 6);
    memory.write(register.get(r1) + offset, register.get(r0));
}

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

fn trap_halt(running: &mut bool) {
    println!("HALT");
    flush_stdout();
    *running = false;
}

fn handle_trap(register: &mut Register, instr: u16, memory: &mut Memory, running: &mut bool) {
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

pub fn handle_operations(
    register: &mut Register,
    instr: u16,
    op: u16,
    memory: &mut Memory,
    running: &mut bool,
) {
    println!("PERFORMING OP: {}", op);
    match op {
        OP_ADD => {
            // @{ADD}
            op_add(register, instr);
        }
        OP_AND => {
            // @{AND}
            op_and(register, instr);
        }
        OP_NOT => {
            // @{NOT}
            op_not(register, instr)
        }
        OP_BR => {
            // @{BR}
            op_br(register, instr);
        }
        OP_JMP => {
            // @{JMP}
            op_jmp(register, instr);
        }
        OP_JSR => {
            // @{JSR}
            op_jsr(register, instr);
        }
        OP_LD => {
            // @{LD}
            op_ld(register, instr, memory);
        }
        OP_LDI => {
            // @{LDI}
            op_ldi(register, instr, memory);
        }
        OP_LDR => {
            // @{LDR}
            op_ldr(register, instr, memory)
        }
        OP_LEA => {
            // @{LEA}
            op_lea(register, instr);
        }
        OP_ST => {
            // @{ST}
            op_st(register, instr, memory);
        }
        OP_STI => {
            // @{STI}
            op_sti(register, instr, memory);
        }
        OP_STR => {
            // @{STR}
            op_str(register, instr, memory);
        }
        OP_TRAP => {
            // @{TRAP}
            handle_trap(register, instr, memory, running);
        }
        _ => {
            // @{BAD OPCODE}
            println!("Bad opcode: {}", op);
            trap_halt(running);
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::{constants::FL_POS, register::Register};
    use super::*;

    // BR TESTS
    #[test]
    fn br_branch_taken_positive_offset() {
        let mut register = Register::new(); 
        register.cond = FL_POS;

        let instr: u16 = 0b0000_001_000000101; 
        op_br(&mut register, instr);

        assert_eq!(register.pc, 0x3005);
    }

    #[test]
    fn br_branch_not_taken() {
        let mut register = Register::new(); 
        register.cond = FL_POS;

        let instr: u16 = 0b0000_010_000000101;
        op_br(&mut register, instr);

        assert_eq!(register.pc, 0x3000); 
    }

    #[test]
    fn br_branch_taken_negative_offset() {
        let mut register = Register::new(); 
        register.cond = FL_POS;

        let instr: u16 = 0b0000_001_111111011; 
        op_br(&mut register, instr);

        assert_eq!(register.pc, 0x2FFB); 
    }

    #[test]
    fn br_branch_zero_offset() {
        let mut register = Register::new(); 
        register.cond = FL_POS;

        let instr: u16 = 0b0000_001_000000000; 
        op_br(&mut register, instr);

        println!("PC: {}", register.pc);

        assert_eq!(register.pc, 0x3000); 
    }

    // ADD TESTS
    #[test]
    fn test_op_add_with_registers() {
        let mut register = Register::new();
        register.set(1, 10);  
        register.set(2, 15);  

        let instr: u16 = 0b0001_000_001_000_010;
        op_add(&mut register, instr);
        println!("REGISTERS: {:?}", register);

        assert_eq!(register.get(0), 25); 
    }

    #[test]
    fn test_op_add_with_immediate_positive() {
        let mut register = Register::new();
        register.set(1, 10);

        let instr: u16 = 0b0001_000_001_1_00001; 
        op_add(&mut register, instr);

        assert_eq!(register.get(0), 11); 
    }

    #[test]
    fn test_op_add_with_immediate_negative() {
        let mut register = Register::new();
        register.set(1, 10); 

        let instr: u16 = 0b0001_000_001_1_11111;
        op_add(&mut register, instr);

        assert_eq!(register.get(0), 9);
    }

    #[test]
    fn test_op_add_with_negative_result() {
        let mut register = Register::new();
        register.set(1, 0);

        let instr: u16 = 0b0001_000_001_1_11111;
        op_add(&mut register, instr);

        assert_eq!(register.get(0), 0xFFFF); 
        assert_eq!(register.cond, FL_NEG); 
    }

    #[test]
    fn test_op_add_with_zero_result() {
        let mut register = Register::new();
        register.set(1, 1); 

        let instr: u16 = 0b0001_000_001_1_11111; 
        op_add(&mut register, instr);

        assert_eq!(register.get(0), 0); 
        assert_eq!(register.cond, FL_ZRO); 
    }

    #[test]
    fn test_op_add_with_positive_result() {
        let mut register = Register::new();
        register.set(1, 1); 

        let instr: u16 = 0b0001_000_001_1_00001;
        op_add(&mut register, instr);

        assert_eq!(register.get(0), 2); 
        assert_eq!(register.cond, FL_POS); 
    }

}
