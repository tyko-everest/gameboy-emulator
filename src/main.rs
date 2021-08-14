use core::panic;
use std::{mem, ops::{Index, IndexMut}};
use num::abs;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use ux::*;

struct CPUState {
    pc: u16,
    a: u8,
    x: u8,
    y: u8,
    sr: u8,
    sp: u8,
    halted: bool,
}

impl CPUState {
    fn new() -> CPUState {
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

enum Flag {
    Carry = 0,
    Zero = 1,
    Interrupt = 2,
    Decimal = 3,
    Break = 4,
    Oveflow = 6,
    Negative = 7,
}

impl Flag {
    fn get(reg: u8, flag: Flag) -> bool {
        let mask: u8 = 1 << flag as u8;
        (reg & mask) == mask
    }

    fn set(reg: &mut u8, flag: Flag, cond: bool) {
        let mask: u8 = 1 << flag as u8;
        if cond {
            *reg |= mask;
        } else {
            *reg &= !mask;
        }
    }
}

type SR = u8;

trait SRTrait {
    const FLAG_CARRY: u8 =      1 << 0;
    const FLAG_ZERO: u8 =       1 << 1;
    const FLAG_INTERRUPT: u8 =  1 << 2;
    const FLAG_DECIMAL: u8 =    1 << 3;
    const FLAG_BREAK: u8 =      1 << 4;
    const FLAG_OVERFLOW: u8 =   1 << 6;
    const FLAG_NEGATIVE: u8 =   1 << 7;

    fn get_carry(&mut self) -> bool;
    fn set_carry(&mut self, set: bool);

    fn get_zero(&mut self) -> bool;
    fn set_zero(&mut self, set: bool);

    fn get_overflow(&mut self) -> bool;
    fn set_overflow(&mut self, set: bool);

    fn get_negative(&mut self) -> bool;
    fn set_negative(&mut self, set: bool);
}

// impl SRTrait for SR {
//     fn get_carry(&mut self) -> bool {
//         *self & SR::FLAG_ZERO == SR::FLAG_ZERO
//     }

//     fn get_zero(&mut self) -> bool {
//         *self & SR::FLAG_ZERO == SR::FLAG_ZERO
//     }

//     // sets or unsets the zero bit of the status register
//     fn set_zero(&mut self, set: bool) {
//         *self = update_flag(*self, SR::FLAG_ZERO, set);
//     }
// }

const MEM_SIZE: usize = 2 << 16;

struct Memory {
    memory: [u8; MEM_SIZE],
}

impl Memory {
    fn read(&self, addr: u16) -> u8 {
        return self.memory[addr as usize];
    }

    fn write(&mut self, addr: u16, val: u8) {
        self.memory[addr as usize] = val;
    }

    fn new() -> Memory {
        Memory {
            memory: [0; MEM_SIZE]
        }
    }
}


type Opcode = u8;

trait Reg {
    fn a(&self) -> u8;
    fn b(&self) -> u8;
    fn c(&self) -> u8;
}

impl Reg for Opcode {
    fn a(&self) -> u8 {
        return self >> 5 & 0b111;
    }

    fn b(&self) -> u8 {
        return self >> 2 & 0b11;
    }

    fn c(&self) -> u8 {
        return self >> 0 & 0b11;
    } 
}

fn get_addr_from_opcode(opcode: u8, state: &CPUState, memory: &mut Memory) -> u16 {
    let low_byte = memory.read(state.pc + 1);
    let high_byte = memory.read(state.pc + 2);
    let abs_adr = low_byte as u16 | (high_byte as u16) << 8;
    match opcode.b() {
        // X,ind
        0 => {
            memory.read((low_byte + state.x) as u16 | ((low_byte + state.x + 1) as u16) << 8) as u16
        },
        // zpg
        1 => {
            low_byte as u16
        },
        // #
        2 => {
            state.pc + 1
        },
        // abs
        3 => {
            (low_byte as u16) | ((high_byte as u16) << 8)
        },
        // ind,Y
        4 => {
            memory.read(low_byte as u16 | ((low_byte + 1) as u16) << 8) as u16 + state.y as u16
        },
        // zpg,X
        5 => {
            low_byte as u16 + state.x as u16
        },
        // abs,Y
        6 => {
            abs_adr + state.y as u16
        },
        // abs,X
        7 => {
            abs_adr + state.x as u16
        },
        // opcode.b() is guaranteed to be 3 bits, this is not possible
        _ => panic!("invalid opcode")
    }
}

fn step_instr(state: &mut CPUState, memory: &mut Memory) {

    let opcode: Opcode = memory.read(state.pc);
    match opcode.c() {
        0 => {

        },
        1 => {
            let addr = get_addr_from_opcode(opcode, state, memory);
            let old_a = state.a;
            match opcode.a() {
                // ORA
                0 => {
                    state.a |= memory.read(addr);
                    Flag::set(&mut state.sr, Flag::Zero, state.a == 0);

                },
                // AND
                1 => {
                    state.a &= memory.read(addr);
                },
                // EOR
                2 => {
                    state.a ^= memory.read(addr);
                },
                // ADC
                3 => {
                    state.a += memory.read(addr);
                },
                // STA
                4 => {
                    memory.write(addr, state.a);
                },
                // LDA
                5 => {
                    state.a = memory.read(addr);
                },
                // CMP
                6 => {

                },
                // SBC
                7 => {
                    state.a -= memory.read(addr);
                },
                _ => panic!("invalid part b of opcode")
            }
        },
        2 => {

        },
        _ => panic!("invalid part c of opcode")

    }
}

fn main() {
    
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adc() {
        let mut state = CPUState::new();
        state.a = 1;

        let mut memory = Memory::new();
        memory.write(0, 0x65);
        memory.write(1, 100);
        memory.write(100, 3);

        step_instr(&mut state, &mut memory);
        assert_eq!(state.a, 4);
    }
}
