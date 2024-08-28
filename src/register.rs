use crate::condition_flag::ConditionFlag;

pub struct Register {
    pub R_R0: i8,
    pub R_R1: i8,
    pub R_R2: i8,
    pub R_R3: i8,
    pub R_R4: i8,
    pub R_R5: i8,
    pub R_R6: i8,
    pub R_R7: i8,
    pub R_PC: i8,
    pub R_COND: ConditionFlag,
    pub R_COUNT: i8,
}

impl Register {
    pub fn new() -> Register {
        Register {
            R_R0: 0,
            R_R1: 0,
            R_R2: 0,
            R_R3: 0,
            R_R4: 0,
            R_R5: 0,
            R_R6: 0,
            R_R7: 0,
            R_PC: 0,
            R_COND: ConditionFlag::FL_ZRO,
            R_COUNT: 0,
        }
    }
}

