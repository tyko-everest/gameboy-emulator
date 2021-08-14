use core::panic;
use std::{mem, ops::{Index, IndexMut}};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;


struct CPUState {
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    flags: u8,
    a: u8,
    sp: u16,
    pc: u16,
    halted: bool
}

const FLAG_C: u8 = 1 << 0;
const FLAG_P: u8 = 1 << 2;
const FLAG_A: u8 = 1 << 4;
const FLAG_Z: u8 = 1 << 6;
const FLAG_S: u8 = 1 << 7;

impl CPUState {
    fn new() -> CPUState{
        CPUState {
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            flags: 0b00000010,
            a: 0,
            sp: 0,
            pc: 0,
            halted: false,
        }
    }

    fn m(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }

    fn carry(&self) -> bool {
        self.flags & FLAG_C == FLAG_C
    }
}

impl Index<u8> for CPUState {
    type Output = u8;
    fn index(&self, index: u8) -> &Self::Output {
        match index {
            0 => &self.b,
            1 => &self.c,
            2 => &self.d,
            3 => &self.e,
            4 => &self.h,
            5 => &self.l,
            7 => &self.a,
            _ => panic!("index {} invalid, must be 0 -> 5 or 7", index),
        }
    }
}

impl IndexMut<u8> for CPUState {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        match index {
            0 => &mut self.b,
            1 => &mut self.c,
            2 => &mut self.d,
            3 => &mut self.e,
            4 => &mut self.h,
            5 => &mut self.l,
            7 => &mut self.a,
            _ => panic!("index {} invalid, must be 0 -> 5 or 7", index),
        }
    }
}


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
    fn regd(&self) -> u8;
    fn regs(&self) -> u8;
    fn operation(&self) -> u8;
}

const REG_MASK: u8 = 0b111;
const REGD_POS: usize = 3;
const REGS_POS: usize = 0;

const REGM: u8 = 6;

const OP_ADD: u8 = 0;
const OP_ADC: u8 = 1;
const OP_SUB: u8 = 2;
const OP_SBB: u8 = 3;
const OP_ANA: u8 = 4;
const OP_XRA: u8 = 5;
const OP_ORA: u8 = 6;
const OP_CMP: u8 = 7;


impl Reg for Opcode {
    fn regd(&self) -> u8 {
        return self >> REGD_POS & REG_MASK;
    }

    fn regs(&self) -> u8 {
        return self >> REGS_POS & REG_MASK;
    }

    fn operation(&self) -> u8 {
        return self >> 3 & 0b111;
    } 
}


#[derive(FromPrimitive)]
enum Operation {
    ADD = 0,
    ADC = 1,
    SUB = 2,
    SBB = 3,
    ANA = 4,
    XRA = 5,
    ORA = 6,
    CMP = 7,
}


// // NOP
// 0x00 => {},
// // LXI B, d16
// 0x01 => {

// },
// // STAX B
// 0x02 => {

// },
// // INX B
// 0x03 => {

// },
// // INR B
// 0x04 => {

// },
// // DCR B
// 0x05 => {

// },
// // MVI B, d8
// 0x06 => {

// },
// // RLC
// 0x07 => {

// },

fn step_instr(state: &mut CPUState, memory: &mut Memory) {
    let opcode: Opcode = memory.read(state.pc);
    match opcode {
        // 16 bit math + misc
        0x00..=0x3F => {

        }
        // moves and halt
        0x40..=0x7F => {
            if opcode == 0x76 {
                state.halted = true;

            } else {
                // memory as destination
                if opcode.regd() == REGM {
                    memory.write(state.m(), state[opcode.regs()]);

                // memory as source
                } else if opcode.regs() == REGM {
                    state[opcode.regd()] = memory.read(state.m());

                // register to register
                } else {
                    state[opcode.regd()] = state[opcode.regs()];

                }
            }
        }
        // 8 bit math
        0x80..=0xBF => {

            let rhs;
            if opcode.regs() == REGM || opcode.operation() == Operation::CMP as u8 {
                rhs = memory.read(state.m());
            } else {
                rhs = state[opcode.regs()];
            }

            let a;
            match FromPrimitive::from_u8(opcode.operation()) {
                Some(Operation::ADD) => {
                    a = state.a as u16 + rhs as u16;
                    if a > u8::MAX as u16 {
                        state.flags |= FLAG_C;
                    } else {
                        state.flags &= !FLAG_C;
                    }
                },
                Some(Operation::ADC) => {
                    a = state.a as u16 + rhs as u16 + state.carry() as u16;
                    if a > u8::MAX as u16 {
                        state.flags |= FLAG_C;
                    } else {
                        state.flags &= !FLAG_C;
                    }
                },
                Some(Operation::SUB) | Some(Operation::CMP) => {
                    // check for proper two's complement behaviour
                    a = state.a as u16 + (rhs as i8).wrapping_neg() as u16;
                    if (a as u8) < rhs {
                        state.flags |= FLAG_C;
                    } else {
                        state.flags &= !FLAG_C;
                    }
                },
                Some(Operation::SBB) => {
                    // check for proper two's complement behaviour
                    a = state.a as u16 + (rhs + state.carry() as u8).wrapping_neg() as u16;
                    if (a as u8) < rhs {
                        state.flags |= FLAG_C;
                    } else {
                        state.flags &= !FLAG_C;
                    }
                },
                Some(Operation::ANA) => {
                    a = state.a as u16 & rhs as u16;
                    state.flags &= !FLAG_C;
                },
                Some(Operation::XRA) => {
                    a = state.a as u16 ^ rhs as u16;
                    state.flags &= !FLAG_C;
                },
                Some(Operation::ORA) => {
                    a = state.a as u16 | rhs as u16;
                    state.flags &= !FLAG_C;
                },
                None => {
                    panic!("what?")
                }
            }

            // update rest of state flags
            if a == 0 {
                state.flags |= FLAG_Z;
            } else {
                state.flags &= !FLAG_Z;
            }

            if (a as i8) < 0 {
                state.flags |= FLAG_S;
            } else {
                state.flags &= !FLAG_S;
            }

            // not updated parity or alt. carry flags currently

            if opcode.operation() != Operation::CMP as u8 {
                state.a = a as u8;
            }

        }
        // jumps, calls, stack, IO
        0xC0..=0xFF => {

        }
    }


}

fn main() {

    let mut memory = Memory::new();
    let mut state = CPUState::new();

    state.a = 1;
    state.b = 254;
    let test = state[0];

    memory.write(0, 0x80);

    step_instr(&mut state, &mut memory);

    println!("done")
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let mut memory = Memory::new();
        let mut state = CPUState::new();

        state.a = 1;
        state.b = 254;
        // opcode for add b
        memory.write(0, 0x80);
        step_instr(&mut state, &mut memory);
        assert_eq!(255, state.a);

        state.a = 1;
        state.b = 255;
        state.pc = 0;
        step_instr(&mut state, &mut memory);
        assert_eq!(0, state.a);
        assert_eq!(state.flags & FLAG_C, FLAG_C);
    }

    #[test]
    fn sub() {
        let mut memory = Memory::new();
        let mut state = CPUState::new();

        state.a = 5;
        state.b = 10;
        // opcode for sub b
        memory.write(0, 0x90);
        step_instr(&mut state, &mut memory);
        assert!(state.a as i8 == -5);
    }

    #[test]
    fn sbb() {
        let mut memory = Memory::new();
        let mut state = CPUState::new();

        state.a = 5;
        state.b = 170;
        // opcode for sub b
        memory.write(0, 0x98);
        step_instr(&mut state, &mut memory);
        assert_eq!(state.flags & FLAG_C, FLAG_C);
    }

}
