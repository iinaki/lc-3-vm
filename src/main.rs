use lc_3_vm::register::Register;
use lc_3_vm::opcode::Opcode;
use lc_3_vm::condition_flag::ConditionFlag;

const MEMORY_MAX: usize = 1 << 16;

fn main() {
    let memory = [0; MEMORY_MAX];
    let register = Register::new();

    // @{Load Arguments}
    // @{Setup}

    /* since exactly one condition flag should be set at any given time, set the Z flag */
    register.R_COND = ConditionFlag::FL_ZRO;

    /* set the PC to starting position */
    /* 0x3000 is the default */
    enum { PC_START = 0x3000 };
    reg[R_PC] = PC_START;

    int running = 1;
    while (running)
    {
        /* FETCH */
        uint16_t instr = mem_read(reg[R_PC]++);
        uint16_t op = instr >> 12;

        switch (op)
        {
            case OP_ADD:
                @{ADD}
                break;
            case OP_AND:
                @{AND}
                break;
            case OP_NOT:
                @{NOT}
                break;
            case OP_BR:
                @{BR}
                break;
            case OP_JMP:
                @{JMP}
                break;
            case OP_JSR:
                @{JSR}
                break;
            case OP_LD:
                @{LD}
                break;
            case OP_LDI:
                @{LDI}
                break;
            case OP_LDR:
                @{LDR}
                break;
            case OP_LEA:
                @{LEA}
                break;
            case OP_ST:
                @{ST}
                break;
            case OP_STI:
                @{STI}
                break;
            case OP_STR:
                @{STR}
                break;
            case OP_TRAP:
                @{TRAP}
                break;
            case OP_RES:
            case OP_RTI:
            default:
                @{BAD OPCODE}
                break;
        }
    }
    @{Shutdown}
}
