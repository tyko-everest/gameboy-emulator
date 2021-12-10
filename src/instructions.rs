use crate::state::*;

/*
These functions will edit the CPU state according to how they're supposed to
However, they do not need read or write to memory
This must be done before calling them, and by using their return values
The only instructions that affect the pc are those with the express purpose to:
i.e. branches, jumps, and returns
The rest do not as the size of the instruction depends on the addressing mode
*/

fn will_overflow(a: u8, b: u8) -> bool {
    let c = a.wrapping_add(b);
    (((a >> 7) ^ (b >> 7)) == 0) && ((a >> 7) != (c >> 7))
}

pub fn adc(state: &mut CPUState, a: u8, b: u8) {
    let a16 = a as u16 + b as u16 + state.get_flag(Flag::Carry) as u16;
    let res = a16 as u8;
    state.set_flag(Flag::Zero, res == 0);
    state.set_flag(Flag::Negative, (res as i8) < 0);
    state.set_flag(Flag::Carry, a16 > u8::MAX as u16);
    state.set_flag(Flag::Overflow, will_overflow(a, b));
    state.a = res;
}

pub fn and(state: &mut CPUState, a: u8, b: u8) {
    let res = a & b;
    state.set_flag(Flag::Zero, res == 0);
    state.set_flag(Flag::Negative, (res as i8) < 0);
    state.a = res
}

pub fn asl(state: &mut CPUState, a: u8) -> u8 {
    state.set_flag(Flag::Carry, a >> 7 == 1);
    let res = a << 1;
    state.set_flag(Flag::Zero, res == 0);
    state.set_flag(Flag::Negative, (res as i8) < 0);
    res
}

fn add_offset(mut pc: u16, offset: i8) -> u16 {
    if offset >= 0 {
        pc += offset as u16;
    } else {
        pc -= -(offset as i16) as u16;
    }
    pc
}

pub fn bcc(state: &mut CPUState, offset: i8) {
    if state.get_flag(Flag::Carry) == false {
        state.pc = add_offset(state.pc, offset);
    }
}

pub fn bcs(state: &mut CPUState, offset: i8) {
    if state.get_flag(Flag::Carry) == true {
        state.pc = add_offset(state.pc, offset);
    }
}

pub fn beq(state: &mut CPUState, offset: i8) {
    if state.get_flag(Flag::Zero) == true {
        state.pc = add_offset(state.pc, offset);
    }
}

pub fn bit(state: &mut CPUState, m: u8) {
    state.set_flag(Flag::Negative, (m >> 7) & 1 == 1);
    state.set_flag(Flag::Overflow, (m >> 6) & 1 == 1);
    state.set_flag(Flag::Zero, state.a & m == 0);
}

pub fn bmi(state: &mut CPUState, offset: i8) {
    if state.get_flag(Flag::Negative) == true {
        state.pc = add_offset(state.pc, offset);
    }
}

pub fn bne(state: &mut CPUState, offset: i8) {
    if state.get_flag(Flag::Zero) == false {
        state.pc = add_offset(state.pc, offset);
    }
}

pub fn bpl(state: &mut CPUState, offset: i8) {
    if state.get_flag(Flag::Negative) == false {
        state.pc = add_offset(state.pc, offset);
    }
}

pub fn brk(state: &mut CPUState) {
    state.set_flag(Flag::Interrupt, true);
}

pub fn bvc(state: &mut CPUState, offset: i8) {
    if state.get_flag(Flag::Overflow) == false {
        state.pc = add_offset(state.pc, offset);
    }
}

pub fn bvs(state: &mut CPUState, offset: i8) {
    if state.get_flag(Flag::Overflow) == true {
        state.pc = add_offset(state.pc, offset);
    }
}

pub fn clc(state: &mut CPUState) {
    state.set_flag(Flag::Carry, false);
}

pub fn cld(state: &mut CPUState) {
    state.set_flag(Flag::Decimal, false);
}

pub fn cli(state: &mut CPUState) {
    state.set_flag(Flag::Interrupt, false);
}

pub fn clv(state: &mut CPUState) {
    state.set_flag(Flag::Overflow, false);
}

pub fn cmp(state: &mut CPUState, a: u8, m: u8) {
    let res = a.wrapping_sub(m);
    state.set_flag(Flag::Zero, res == 0);
    state.set_flag(Flag::Negative, (res as i8) < 0);
    state.set_flag(Flag::Carry, (res as i8) >= 0);
}

pub fn cpx(state: &mut CPUState, x: u8, m: u8) {
    cmp(state, x, m);
}

pub fn cpy(state: &mut CPUState, y: u8, m: u8) {
    cmp(state, y, m);
}

pub fn dec(state: &mut CPUState, a: u8) -> u8 {
    let res = a.wrapping_sub(1);
    state.set_flag(Flag::Zero, res == 0);
    state.set_flag(Flag::Negative, (res as i8) < 0);
    res
}

pub fn dex(state: &mut CPUState, a: u8) {
    state.x = dec(state, a);
}

pub fn dey(state: &mut CPUState, a: u8) {
    state.y = dec(state, a);
}

pub fn eor(state: &mut CPUState, a: u8, b: u8) {
    let res = a ^ b;
    state.set_flag(Flag::Zero, res == 0);
    state.set_flag(Flag::Negative, (res as i8) < 0);
    state.a = res
}

pub fn inc(state: &mut CPUState, a: u8) -> u8 {
    let res = a.wrapping_add(1);
    state.set_flag(Flag::Zero, res == 0);
    state.set_flag(Flag::Negative, (res as i8) < 0);
    res
}

pub fn inx(state: &mut CPUState, a: u8) {
    state.x = inc(state, a);
}

pub fn iny(state: &mut CPUState, a: u8) {
    state.y = inc(state, a);
}

// used for jmp and jsr
pub fn jmp(state: &mut CPUState, addr: u16) {
    state.pc = addr;
}

// used for lda, ldx, ldy to update the status flags
pub fn ldn(state: &mut CPUState, a: u8) {
    state.set_flag(Flag::Zero, a == 0);
    state.set_flag(Flag::Negative, (a as i8) < 0);
}

pub fn lsr(state: &mut CPUState, a: u8) -> u8 {
    state.set_flag(Flag::Carry, a & 1 == 1);
    state.set_flag(Flag::Negative, false);
    let res = a >> 1;
    state.set_flag(Flag::Zero, res == 0);
    res
}

// nop

pub fn ora(state: &mut CPUState, a: u8, b: u8) {
    let res = a | b;
    state.set_flag(Flag::Zero, res == 0);
    state.set_flag(Flag::Negative, (res as i8) < 0);
    state.a = res;
}

// pha, php don't affect the cpu state

pub fn pla(state: &mut CPUState, a: u8) {
    state.set_flag(Flag::Zero, a == 0);
    state.set_flag(Flag::Negative, (a as i8) < 0);
}

pub fn plp(state: &mut CPUState, status: u8) {
    let mask: u8 = 1 << Flag::Break as u8 | 1 << Flag::Unused as u8;
    state.sr &= mask;
    state.sr |= status & !mask;
}

pub fn rol(state: &mut CPUState, a: u8) -> u8 {
    let old_carry = state.get_flag(Flag::Carry);
    state.set_flag(Flag::Carry, (a >> 7) & 1 == 1);
    let res = (a << 1) | (old_carry as u8);
    state.set_flag(Flag::Zero, a == 0);
    state.set_flag(Flag::Negative, (a as i8) < 0);
    res
}

pub fn ror(state: &mut CPUState, a: u8) -> u8 {
    let old_carry = state.get_flag(Flag::Carry);
    state.set_flag(Flag::Carry, a & 1 == 1);
    let res = (a >> 1) | ((old_carry as u8) << 7);
    state.set_flag(Flag::Zero, a == 0);
    state.set_flag(Flag::Negative, (a as i8) < 0);
    res
}

// rti, rts

pub fn sbc(state: &mut CPUState, a: u8, b: u8) {
    let b = (-(b as i8)) as u8;
    let a16 = a as u16 + b.wrapping_sub(state.get_flag(Flag::Carry) as u8) as u16;
    let res = a16 as u8;
    state.set_flag(Flag::Zero, res == 0);
    state.set_flag(Flag::Negative, (res as i8) < 0);
    state.set_flag(Flag::Carry, a16 > u8::MAX as u16);
    state.set_flag(Flag::Overflow, will_overflow(a, b));
    state.a = res;
}

pub fn sec(state: &mut CPUState) {
    state.set_flag(Flag::Carry, true);
}

pub fn sed(state: &mut CPUState) {
    state.set_flag(Flag::Decimal, true);
}

pub fn sei(state: &mut CPUState) {
    state.set_flag(Flag::Interrupt, true);
}

// sta, stx, sty dont't affect the cpu state

pub fn tax(state: &mut CPUState) {
    state.set_flag(Flag::Negative, (state.a as i8) < 0);
    state.set_flag(Flag::Zero, state.a == 0);
    state.x = state.a;
}

pub fn tay(state: &mut CPUState) {
    state.set_flag(Flag::Negative, (state.a as i8) < 0);
    state.set_flag(Flag::Zero, state.a == 0);
    state.y = state.a;
}

pub fn tsx(state: &mut CPUState) {
    state.set_flag(Flag::Negative, (state.sp as i8) < 0);
    state.set_flag(Flag::Zero, state.sp == 0);
    state.x = state.sp;
}

pub fn txa(state: &mut CPUState) {
    state.set_flag(Flag::Negative, (state.x as i8) < 0);
    state.set_flag(Flag::Zero, state.x == 0);
    state.a = state.x;
}

pub fn txs(state: &mut CPUState) {
    state.set_flag(Flag::Negative, (state.x as i8) < 0);
    state.set_flag(Flag::Zero, state.x == 0);
    state.sp = state.x;
}

pub fn tya(state: &mut CPUState) {
    state.set_flag(Flag::Negative, (state.y as i8) < 0);
    state.set_flag(Flag::Zero, state.y == 0);
    state.a = state.y;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adc() {
        let mut state = CPUState::new();
        let a = 1;
        let b = 255;

        adc(&mut state, a, b);
        assert_eq!(state.a, 0);
        assert_eq!(state.sr, 0x03);
    }
}
