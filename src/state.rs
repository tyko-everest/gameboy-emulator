
pub enum Flag {
    Carry = 0,
    Zero = 1,
    Interrupt = 2,
    Decimal = 3,
    Break = 4,
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
}

pub struct Memory {
    memory: Vec<u8>,
}

impl Memory {
    pub fn read(&self, addr: u16) -> u8 {
        return self.memory[addr as usize];
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        self.memory[addr as usize] = val;
    }

    pub fn new(size: usize) -> Memory {
        Memory {
            memory: vec![0; size]
        }
    }
}
