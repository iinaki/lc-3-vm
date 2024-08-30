use crate::constants::{FL_ZRO, PC_START};

#[derive(Debug)]
pub struct Register {
    pub r0: u16,
    pub r1: u16,
    pub r2: u16,
    pub r3: u16,
    pub r4: u16,
    pub r5: u16,
    pub r6: u16,
    pub r7: u16,
    pub pc: u16,
    pub cond: u16,
    pub count: u16,
}

impl Register {
    pub fn new() -> Register {
        Register {
            r0: 0,
            r1: 0,
            r2: 0,
            r3: 0,
            r4: 0,
            r5: 0,
            r6: 0,
            r7: 0,
            pc: PC_START,
            cond: FL_ZRO,
            count: 0,
        }
    }

    pub fn get(&self, r: u16) -> u16 {
        match r {
            0 => self.r0,
            1 => self.r1,
            2 => self.r2,
            3 => self.r3,
            4 => self.r4,
            5 => self.r5,
            6 => self.r6,
            7 => self.r7,
            8 => self.pc,
            9 => self.cond,
            _ => {
                println!("Invalid register");
                0
            }
        }
    }

    pub fn set(&mut self, r: u16, val: u16) {
        match r {
            0 => self.r0 = val,
            1 => self.r1 = val,
            2 => self.r2 = val,
            3 => self.r3 = val,
            4 => self.r4 = val,
            5 => self.r5 = val,
            6 => self.r6 = val,
            7 => self.r7 = val,
            8 => self.pc = val,
            9 => self.cond = val,
            _ => println!("Invalid register"),
        }
    }
}
