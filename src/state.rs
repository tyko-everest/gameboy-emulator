
pub enum Flag {
    Carry = 0,
    Zero = 1,
    Interrupt = 2,
    Decimal = 3,
    Break = 4,
    Unused = 5,
    Overflow = 6,
    Negative = 7,
}

impl Flag {
    pub fn get(flag: Flag, reg: u8) -> bool {
        let mask: u8 = 1 << flag as u8;
        (reg & mask) == mask
    }

    pub fn set(flag: Flag, reg: &mut u8, cond: bool) {
        let mask: u8 = 1 << flag as u8;
        if cond {
            *reg |= mask;
        } else {
            *reg &= !mask;
        }
    }
}

pub struct CPUState {
    pub pc: u16,
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub sr: u8,
    pub sp: u8,
    pub halted: bool,
}

impl CPUState {
    pub fn new() -> CPUState {
        CPUState {
            pc: 0,
            a: 0,
            x: 0,
            y: 0,
            sr: 0,
            sp: 0,
            halted: false,
        }
    }

    pub fn get_flag(&self, flag: Flag) -> bool {
        Flag::get(flag, self.sr)
    }

    pub fn set_flag(&mut self, flag: Flag, val: bool) {
        Flag::set(flag, &mut self.sr, val);
    }
}
