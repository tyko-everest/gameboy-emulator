use core::panic;
use crate::state::*;
use crate::instructions::*;

mod state;
mod instructions;

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
            let m = memory.read(addr);
            match opcode.a() {
                // ORA
                0 => {
                    state.a = ora(state.a, m, &mut state.sr);
                },
                // AND
                1 => {
                    state.a = and(state.a, m, &mut state.sr);
                },
                // EOR
                2 => {
                    state.a = eor(state.a, m, &mut state.sr);
                },
                // ADC
                3 => {
                    state.a = adc(state.a, m, &mut state.sr);
                },
                // STA
                4 => {
                    if opcode.b() != 2 {
                        memory.write(addr, state.a);
                    }
                },
                // LDA
                5 => {
                    state.a = memory.read(addr);
                    Flag::set(Flag::Zero, &mut state.sr, state.a == 0);
                    Flag::set(Flag::Negative, &mut state.sr, (state.a as i8) < 0);
                },
                // CMP
                6 => {
                    cmp(state.a, m, &mut state.sr);
                },
                // SmC
                7 => {
                    state.a = sbc(state.a, m, &mut state.sr);
                },
                _ => panic!("not possible")
            }

        },
        2 => {
            match opcode.a() {
                // ASL
                0 => {
                    if opcode.b() & 0b1 == 1 {
                        let addr = get_addr_from_opcode(opcode, state, memory);
                        let mut a = memory.read(addr);
                        a = asl(a, &mut state.sr);
                        memory.write(addr, a);

                    } else if opcode.b() == 2 {
                        state.a = asl(state.a, &mut state.sr);

                    } else {
                        panic!("illegal instruction");
                    }

                },
                // ROL
                1 => {
                    if opcode.b() & 0b1 == 1 {
                        let addr = get_addr_from_opcode(opcode, state, memory);
                        let mut a = memory.read(addr);
                        a = asl(a, &mut state.sr);
                        memory.write(addr, a);

                    } else if opcode.b() == 2 {
                        state.a = rol(state.a, &mut state.sr)

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
