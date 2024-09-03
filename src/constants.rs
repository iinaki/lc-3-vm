/// The size of the memory in the virtual machine.
pub const MEMORY_SIZE: usize = 65536;

/// The default starting address for the program counter (PC).
pub const PC_START: u16 = 0x3000;

// CONDITION FLAGS

/// The result of an operation is positive.
pub const FL_POS: u16 = 1 << 0;

/// The result of an operation is zero.
pub const FL_ZRO: u16 = 1 << 1;

/// The result of an operation is negative.
pub const FL_NEG: u16 = 1 << 2;

// OP CODES

/// Operation code for branch instructions.
pub const OP_BR: u16 = 0;

/// Operation code for add instructions.
pub const OP_ADD: u16 = 1;

/// Operation code for load instructions.
pub const OP_LD: u16 = 2;

/// Operation code for store instructions.
pub const OP_ST: u16 = 3;

/// Operation code for jump register instructions.
pub const OP_JSR: u16 = 4;

/// Operation code for bitwise AND instructions.
pub const OP_AND: u16 = 5;

/// Operation code for load register instructions.
pub const OP_LDR: u16 = 6;

/// Operation code for store register instructions.
pub const OP_STR: u16 = 7;

/// Operation code for return from interrupt instructions (unused).
pub const OP_RTI: u16 = 8;

/// Operation code for bitwise NOT instructions.
pub const OP_NOT: u16 = 9;

/// Operation code for load indirect instructions.
pub const OP_LDI: u16 = 10;

/// Operation code for store indirect instructions.
pub const OP_STI: u16 = 11;

/// Operation code for jump instructions.
pub const OP_JMP: u16 = 12;

/// Operation code for reserved (unused) instructions.
pub const OP_RES: u16 = 13;

/// Operation code for load effective address instructions.
pub const OP_LEA: u16 = 14;

/// Operation code for trap instructions.
pub const OP_TRAP: u16 = 15;

// TRAP CODES

/// Trap code for getting a character from the keyboard (not echoed onto the terminal).
pub const TRAP_GETC: u16 = 0x20;

/// Trap code for outputting a character.
pub const TRAP_OUT: u16 = 0x21;

/// Trap code for outputting a word string.
pub const TRAP_PUTS: u16 = 0x22;

/// Trap code for getting a character from the keyboard (echoed onto the terminal).
pub const TRAP_IN: u16 = 0x23;

/// Trap code for outputting a byte string.
pub const TRAP_PUTSP: u16 = 0x24;

/// Trap code for halting the program.
pub const TRAP_HALT: u16 = 0x25;

// MEMORY MAPPED REGISTERS

/// Memory-mapped register for the keyboard status.
pub const MR_KBSR: u16 = 0xFE00;

/// Memory-mapped register for keyboard data.
pub const MR_KBDR: u16 = 0xFE02;
