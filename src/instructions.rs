use crate::state::*;

fn will_overflow(a: u8, b: u8) -> bool {
    let c = a.wrapping_add(b);
    (((a >> 7) ^ (b >> 7)) == 0) && ((a >> 7) != (c >> 7))
}

pub fn adc(a: u8, b: u8, sr: &mut u8) -> u8 {
    let a16 = a as u16 + b as u16 + Flag::get(Flag::Carry, *sr) as u16;
    let res = a16 as u8;
    Flag::set(Flag::Zero, sr,  res == 0);
    Flag::set(Flag::Negative, sr,  (res as i8) < 0);
    Flag::set(Flag::Carry, sr,  a16 > u8::MAX as u16);
    Flag::set(Flag::Overflow, sr,  will_overflow(a, b));
    res
}

pub fn and(a: u8, b: u8, sr: &mut u8) -> u8 {
    let res = a & b;
    Flag::set(Flag::Zero, sr, res == 0);
    Flag::set(Flag::Negative, sr, (res as i8) < 0);
    res
}

pub fn asl(a: u8, sr: &mut u8) -> u8 {
    Flag::set(Flag::Carry, sr, a >> 7 == 1);
    let res = a << 1;
    Flag::set(Flag::Zero, sr, res == 0);
    Flag::set(Flag::Negative, sr, (res as i8) < 0);
    res
}

// bcc, bcs, beq
// bit
// bmi, bne, bpl
// brk
// bvc
// bvs

pub fn clc(sr: &mut u8) {
    Flag::set(Flag::Carry, sr, false);
}

pub fn cld(sr: &mut u8) {
    Flag::set(Flag::Decimal, sr, false);
}

pub fn cli(sr: &mut u8) {
    Flag::set(Flag::Interrupt, sr, false);
}

pub fn clv(sr: &mut u8) {
    Flag::set(Flag::Overflow, sr, false);
}

// also used for cpx and cpy
pub fn cmp(a: u8, b: u8, sr: &mut u8) {
    let res = a.wrapping_sub(b);
    Flag::set(Flag::Zero, sr, res == 0);
    Flag::set(Flag::Negative, sr, (res as i8) < 0);
    Flag::set(Flag::Carry, sr, (res as i8) >= 0);
}

// also used for dex and dey
pub fn dec(a: u8, sr: &mut u8) -> u8 {
    let res = a.wrapping_sub(1);
    Flag::set(Flag::Zero, sr, res == 0);
    Flag::set(Flag::Negative, sr, (res as i8) < 0);
    res
}

pub fn eor(a: u8, b: u8, sr: &mut u8) -> u8 {
    let res = a ^ b;
    Flag::set(Flag::Zero, sr, res == 0);
    Flag::set(Flag::Negative, sr, (res as i8) < 0);
    res
}

// also used for inx and iny
pub fn inc(a: u8, sr: &mut u8) -> u8 {
    let res = a.wrapping_add(1);
    Flag::set(Flag::Zero, sr, res == 0);
    Flag::set(Flag::Negative, sr, (res as i8) < 0);
    res
}

// jmp, jsr

// lda, ldx, ldy

pub fn lsr(a: u8, sr: &mut u8) -> u8 {
    Flag::set(Flag::Carry, sr, a & 1 == 1);
    Flag::set(Flag::Negative, sr, false);
    let res = a >> 1;
    Flag::set(Flag::Zero, sr, res == 0);
    res
}

// nop

pub fn ora(a: u8, b: u8, sr: &mut u8) -> u8 {
    let res = a | b;
    Flag::set(Flag::Zero, sr, res == 0);
    Flag::set(Flag::Negative, sr, (res as i8) < 0);
    res
}

// pha, php, pla, plp

pub fn rol(a: u8, sr: &mut u8) -> u8 {
    let old_carry = Flag::get(Flag::Carry, *sr);
    Flag::set(Flag::Carry, sr, (a >> 7) & 1 == 1);
    let res = (a << 1) | (old_carry as u8);
    Flag::set(Flag::Zero, sr, a == 0);
    Flag::set(Flag::Negative, sr, (a as i8) < 0);
    res
}

pub fn ror(a: u8, sr: &mut u8) -> u8 {
    let old_carry = Flag::get(Flag::Carry, *sr);
    Flag::set(Flag::Carry, sr, a & 1 == 1);
    let res = (a >> 1) | ((old_carry as u8) << 7);
    Flag::set(Flag::Zero, sr, a == 0);
    Flag::set(Flag::Negative, sr, (a as i8) < 0);
    res
}

// rti, rts

pub fn sbc(a: u8, b: u8, sr: &mut u8) -> u8 {
    let b = (-(b as i8)) as u8;
    let a16 = a as u16 + b.wrapping_sub(Flag::get(Flag::Carry, *sr) as u8) as u16;
    let res = a16 as u8;
    Flag::set(Flag::Zero, sr, res == 0);
    Flag::set(Flag::Negative, sr, (res as i8) < 0);
    Flag::set(Flag::Carry, sr, a16 > u8::MAX as u16);
    Flag::set(Flag::Overflow, sr, will_overflow(a, b));
    res
}

pub fn sec(sr: &mut u8) {
    Flag::set(Flag::Carry, sr, true);
}

pub fn sed(sr: &mut u8) {
    Flag::set(Flag::Decimal, sr, true);
}

pub fn sei(sr: &mut u8) {
    Flag::set(Flag::Interrupt, sr, true);
}

// sta, stx, sty

// tax, tay, tsx, txa, txs, tya

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adc() {
        let mut sr: u8 = 0;
        let a: u8 = 1;
        let b: u8 = 255;

        let res = adc(a, b, &mut sr);
        assert_eq!(res, 0);
        assert_eq!(res, 0x03);
    }
}
