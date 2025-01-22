pub struct Memory {
    memory: [u8; 65536],
}

impl Memory {
    pub fn new() -> Memory {
        Memory { memory: [0; 65536] }
    }

    pub fn check_write(&self, addr: u16) {
        if addr > 32767 && addr < 62216 { return }
        if addr > 65532 { return }
        if addr < 32768 {
            panic!("Attempted to write to ROM");
        }
        if addr > 62215 && addr < 65533 {
            panic!("Attempted to write to unused memory");
        }
    }

    pub fn check_read(&self, addr: u16) {
        if addr > 62215 && addr < 65533 {
            panic!("Attempted to read from unused memory");
        }
    }

    pub fn poke(&mut self, address: u16, value: u8) {
        self.check_write(address);
        self.memory[address as usize] = value;
    }

    pub fn peek(&self, address: u16) -> u8 {
        self.check_read(address);
        self.memory[address as usize]
    }

    pub fn peek2(&self, address: u16) -> u16 {
        ((self.peek(address) as u16) << 8) + self.peek(address + 1) as u16
    }

    pub fn write(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }

    pub fn write_vec(&mut self, address: u16, values: Vec<u8>) {
        let mut offset: u16 = 0;
        for value in values {
            self.write(address + offset, value);
            offset += 1;
        }
    }
}