use crate::constants::FL_ZRO;

pub struct Register {
    pub R_R0: u16,
    pub R_R1: u16,
    pub R_R2: u16,
    pub R_R3: u16,
    pub R_R4: u16,
    pub R_R5: u16,
    pub R_R6: u16,
    pub R_R7: u16,
    pub R_PC: u16,
    pub R_COND: u16,
    pub R_COUNT: u16,
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
            R_COND: FL_ZRO,
            R_COUNT: 0,
        }
    }

    pub fn get(&self, r: u16) -> u16 {
        match r {
            0 => self.R_R0,
            1 => self.R_R1,
            2 => self.R_R2,
            3 => self.R_R3,
            4 => self.R_R4,
            5 => self.R_R5,
            6 => self.R_R6,
            7 => self.R_R7,
            _ => {
                println!("Invalid register");
                0
            }
        }
    }

    pub fn set(&mut self, r: u16, val: u16) {
        match r {
            0 => self.R_R0 = val,
            1 => self.R_R1 = val,
            2 => self.R_R2 = val,
            3 => self.R_R3 = val,
            4 => self.R_R4 = val,
            5 => self.R_R5 = val,
            6 => self.R_R6 = val,
            7 => self.R_R7 = val,
            _ => println!("Invalid register"),
        }
    }
}
