use crate::memory::Memory;

enum Flag {
    Carry = 1 << 0,
    Zero = 1 << 1,
    Interrupt = 1 << 2,
    Decimal = 1 << 3,
    Break = 1 << 4,
    Unused = 1 << 5,
    Overflow = 1 << 6,
    Negative = 1 << 7,
}

struct StatusReg {
    val: u8,
}

impl StatusReg {
    pub fn new() -> StatusReg {
        StatusReg {
            val: 0,
        }
    }

    pub fn get(&self, flag: Flag) -> bool {
        let mask: u8 = flag as u8;
        (self.val & mask) == mask
    }

    pub fn set(&mut self, flag: Flag, status: bool) {
        let mask: u8 = flag as u8;
        if status {
            self.val |= mask;
        } else {
            self.val &= !mask;
        }
    }
}

struct State {
    pub pc: u16,
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub sr: StatusReg,
    pub sp: u8,
    pub halted: bool,
}

impl State {
    pub fn new() -> State {
        State {
            pc: 0,
            a: 0,
            x: 0,
            y: 0,
            sr: StatusReg::new(),
            sp: 0,
            halted: false,
        }
    }

    pub fn get_flag(&self, flag: Flag) -> bool {
        self.sr.get(flag)
    }

    pub fn set_flag(&mut self, flag: Flag, val: bool) {
        self.sr.set(flag, val);
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
        return self >> 2 & 0b111;
    }

    fn c(&self) -> u8 {
        return self >> 0 & 0b11;
    } 
}

pub struct Cpu {
    state: State
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            state: State::new(),
        }
    }

    pub fn step_instr(&mut self, mem: &mut Memory) {
        let opcode: Opcode = mem.read(self.state.pc);
        match opcode.c() {
            0 => {

            },
            1 => {
                match opcode.a() {
                    0 => self.ora(mem, opcode.b()),
                    1 => self.and(mem, opcode.b()),
                    2 => self.eor(mem, opcode.b()),
                    3 => self.adc(mem, opcode.b()),
                    4 => self.sta(mem, opcode.b()),
                    5 => self.lda(mem, opcode.b()),
                    6 => self.cmp(mem, opcode.b()),
                    7 => self.sbc(mem, opcode.b()),
                    _ => panic!("only here because I don't know how to match exhaustively on a 3-bit num"),
                }
            },
            2 => {

            },
            _ => panic!("illegal c value in opcode"),
        }
    }

    // helper function for c = 1 type instructions
    // gets the address of arg, the cycles the instr will take, and the size of the opcode
    fn c1_helper(&mut self, memory: &Memory, mode: u8) -> (u16, u8, u16) {
        let low_byte = memory.read(self.state.pc + 1);
        let high_byte = memory.read(self.state.pc + 2);
        let abs_addr = low_byte as u16 | (high_byte as u16) << 8;

        match mode {
            // X,ind
            0 => {
                let addr = (low_byte + self.state.x) as u16 | ((low_byte + self.state.x + 1) as u16) << 8;
                (addr, 6, 2)
            },
            // zpg
            1 => {
                let addr = low_byte as u16;
                (addr, 3, 2)
            },
            // #
            2 => {
                (self.state.pc + 1, 2, 2)
            },
            // abs
            3 => {
                let addr = abs_addr;
                (addr, 4, 3)
            },
            // ind,Y
            4 => {
                let addr = (memory.read(low_byte as u16) as u16 | (memory.read(low_byte as u16 + 1) as u16) << 8) as u16 + self.state.y as u16;
                let cycles;
                if (addr >> 8) as u8 > high_byte {
                    cycles = 5;
                } else {
                    cycles = 6;
                }
                (addr, cycles, 2)
            },
            // zpg,X
            5 => {
                let addr = low_byte.wrapping_add(self.state.x) as u16;
                (addr, 4, 2)
            },
            // abs,Y
            6 => {
                let addr = abs_addr + self.state.y as u16;
                let cycles;
                if (addr >> 8) as u8 > high_byte {
                    cycles = 5;
                } else {
                    cycles = 6;
                }
                (addr, cycles, 3)
            },
            // abs,X
            7 => {
                let addr = abs_addr + self.state.x as u16;
                let cycles;
                if (addr >> 8) as u8 > high_byte {
                    cycles = 5;
                } else {
                    cycles = 6;
                }
                (addr, cycles, 3)
            },
            // opcode.b() is guaranteed to be 3 bits, this is not possible
            _ => panic!("not possible"),
        }
    }

    // helper for the first half of the c=2 instructions
    // gets the address of arg, the cycles the instr will take, and the size of the opcode
    fn c2_helper(&mut self, memory: &Memory, mode: u8) -> (u16, u8, u16) {
        
    }

    // c=1 type instructions in order of opcode

    fn ora(&mut self, mem: &Memory, mode: u8) {
        let (addr, cycles, size) = self.c1_helper(mem, mode);
        let b = mem.read(addr);
        let res = self.state.a | b;
        self.state.set_flag(Flag::Zero, res == 0);
        self.state.set_flag(Flag::Negative, (res as i8) < 0);
        self.state.a = res;
        self.state.pc += size;
    }

    fn and(&mut self, mem: &Memory, mode: u8) {
        let (addr, cycles, size) = self.c1_helper(mem, mode);
        let b = mem.read(addr);
        let res = self.state.a & b;
        self.state.set_flag(Flag::Zero, res == 0);
        self.state.set_flag(Flag::Negative, (res as i8) < 0);
        self.state.a = res;
        self.state.pc += size;
    }

    fn eor(&mut self, mem: &Memory, mode: u8) {
        let (addr, cycles, size) = self.c1_helper(mem, mode);
        let b = mem.read(addr);
        let res = self.state.a ^ b;
        self.state.set_flag(Flag::Zero, res == 0);
        self.state.set_flag(Flag::Negative, (res as i8) < 0);
        self.state.a = res;
        self.state.pc += size;
    }

    fn will_overflow(a: u8, b: u8) -> bool {
        let c = a.wrapping_add(b);
        (((a >> 7) ^ (b >> 7)) == 0) && ((a >> 7) != (c >> 7))
    }

    fn adc(&mut self, mem: &Memory, mode: u8) {
        let (addr, cycles, size) = self.c1_helper(mem, mode);
        let b = mem.read(addr);
        let a16 = self.state.a as u16 + b as u16 + self.state.get_flag(Flag::Carry) as u16;
        let res = a16 as u8;
        self.state.set_flag(Flag::Zero, res == 0);
        self.state.set_flag(Flag::Negative, (res as i8) < 0);
        self.state.set_flag(Flag::Carry, a16 > u8::MAX as u16);
        self.state.set_flag(Flag::Overflow, Cpu::will_overflow(self.state.a, b));
        self.state.a = res;
        self.state.pc += size;
    }

    fn sta(&mut self, mem: &mut Memory, mode: u8) {
        let (addr, cycles, size) = self.c1_helper(mem, mode);
        mem.write(addr, self.state.a);
        self.state.pc += size;
    }

    fn lda(&mut self, mem: &Memory, mode: u8) {
        let (addr, cycles, size) = self.c1_helper(mem, mode);
        let b = mem.read(addr);
        self.state.set_flag(Flag::Zero, self.state.a == 0);
        self.state.set_flag(Flag::Negative, (self.state.a as i8) < 0);
        self.state.a = b;
        self.state.pc += size;
    }

    fn cmp(&mut self, mem: &Memory, mode: u8) {
        let (addr, cycles, size) = self.c1_helper(mem, mode);
        let m = mem.read(addr);
        let res = self.state.a.wrapping_sub(m);
        self.state.set_flag(Flag::Zero, res == 0);
        self.state.set_flag(Flag::Negative, (res as i8) < 0);
        self.state.set_flag(Flag::Carry, (res as i8) >= 0);
        self.state.pc += size;
    }

    fn sbc(&mut self, mem: &Memory, mode: u8) {
        let (addr, cycles, size) = self.c1_helper(mem, mode);
        let b = (-(mem.read(addr) as i8)) as u8;
        let a16 = self.state.a as u16 + b.wrapping_sub(self.state.get_flag(Flag::Carry) as u8) as u16;
        let res = a16 as u8;
        self.state.set_flag(Flag::Zero, res == 0);
        self.state.set_flag(Flag::Negative, (res as i8) < 0);
        self.state.set_flag(Flag::Carry, a16 > u8::MAX as u16);
        self.state.set_flag(Flag::Overflow, Cpu::will_overflow(self.state.a, b));
        self.state.a = res;
        self.state.pc += size;
    }

    // c=2 type instructions

    pub fn asl(&mut self, mem: &Memory, mode: u8) {
        self.state.set_flag(Flag::Carry, a >> 7 == 1);
        let res = a << 1;
        self.state.set_flag(Flag::Zero, res == 0);
        self.state.set_flag(Flag::Negative, (res as i8) < 0);
        res
    }
}
