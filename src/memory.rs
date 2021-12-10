
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