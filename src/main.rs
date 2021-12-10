use core::panic;
use crate::state::*;
use crate::instructions::*;
use crate::memory::*;

mod state;
mod instructions;
mod memory;

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

// returns the address to use for the operand, and the number of cycles it takes
fn get_addr_from_opcode(opcode: u8, state: &CPUState, memory: &mut Memory) -> (u16, usize) {
    let low_byte = memory.read(state.pc + 1);
    let high_byte = memory.read(state.pc + 2);
    let abs_adr = low_byte as u16 | (high_byte as u16) << 8;
    match opcode.b() {
        // X,ind
        0 => {
            (memory.read((low_byte + state.x) as u16 | ((low_byte + state.x + 1) as u16) << 8) as u16, 6)
        },
        // zpg
        1 => {
            (low_byte as u16, 3)
        },
        // #
        2 => {
            (state.pc + 1, 2)
        },
        // abs
        3 => {
            ((low_byte as u16) | ((high_byte as u16) << 8), 4)
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
        _ => panic!("not possible")
    }
}

fn step_instr(state: &mut CPUState, memory: &mut Memory) {

    let opcode: Opcode = memory.read(state.pc);
    match opcode.c() {
        0 => {

        },
        1 => {
            let (addr, cycles) = get_addr_from_opcode(opcode, state, memory);
            let m = memory.read(addr);
            match opcode.a() {
                // ORA
                0 => {
                    ora(state, state.a, m);
                },
                // AND
                1 => {
                    and(state, state.a, m);
                },
                // EOR
                2 => {
                    eor(state, state.a, m);
                },
                // ADC
                3 => {
                    adc(state, state.a, m);
                },
                // STA
                4 => {
                    if opcode.b() != 2 {
                        memory.write(addr, state.a);
                    } else {
                        panic!("illegal opcode");
                    }
                },
                // LDA
                5 => {
                    state.a = memory.read(addr);
                    ldn(state, state.a);
                },
                // CMP
                6 => {
                    cmp(state, state.a, m);
                },
                // SmC
                7 => {
                    sbc(state, state.a, m);
                },
                _ => panic!("not possible")
            }

        },
        2 => {
            match opcode.a() {
                // ASL
                0 => {
                    if opcode.b() & 0b1 == 1 {
                        let (addr, cycles) = get_addr_from_opcode(opcode, state, memory);
                        let mut a = memory.read(addr);
                        a = asl(state, a);
                        memory.write(addr, a);

                    } else if opcode.b() == 2 {
                        state.a = asl(state, state.a);

                    } else {
                        panic!("illegal instruction");
                    }

                },
                // ROL
                1 => {
                    if opcode.b() & 0b1 == 1 {
                        let (addr, cycles) = get_addr_from_opcode(opcode, state, memory);
                        let mut a = memory.read(addr);
                        a = asl(state, a);
                        memory.write(addr, a);

                    } else if opcode.b() == 2 {
                        state.a = rol(state, state.a)

                    } else {
                        panic!("illegal instruction");
                    }

                },
                // LSR
                2 => {

                },
                // ROR
                3 => {

                },
                // STX, TXA, TXS
                4 => {

                },
                // LDX, TAX, TSX
                5 => {

                },
                // DEC
                6 => {

                },
                // INC
                7 => {

                },
                _ => panic!("not possible")
            }

        },
        _ => panic!("illegal instruction")
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

        let mut memory = Memory::new(2 << 16);
        memory.write(0, 0x65);
        memory.write(1, 100);
        memory.write(100, 255);

        step_instr(&mut state, &mut memory);
        assert_eq!(state.a, 0);
        assert_eq!(state.sr, 0x03);
    }
}
