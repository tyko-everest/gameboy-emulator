mod cpu;
mod memory;

use core::panic;
use crate::cpu::Cpu;
use crate::memory::Memory;



fn main() {
    let mut cpu = Cpu::new();
    let mut mem = Memory::new(1 << 16);

    cpu.step_instr(&mut mem);
    
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(0, 0);
    }
}
