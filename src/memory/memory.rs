#[derive(Clone, Default)]
pub struct Memory {
    pub memory: Vec<u8>,
}

impl Memory {
    pub fn new() -> Self {
        Memory { memory: vec![0u8; 0x1_0000 * 64] }
    }

    pub fn read_u8(&self, addr: u32) -> u8 {
        self.memory[addr as usize]
    }

    pub fn read_u16(&self, addr: u32) -> u16 {
        u16::from(self.read_u8(addr + 1)) << 8 | u16::from(self.read_u8(addr))
    }

    pub fn write_u8(&mut self, addr: u32, data: u8) {
        self.memory[addr as usize] = data;
    }

    pub fn write_u16(&mut self, addr: u32, data: u16) {
        self.write_u8(addr, data as u8);
        self.write_u8(addr + 1, (data >> 8) as u8);
    }

    pub fn write_u32(&mut self, addr: u32, data: u32) {
        self.write_u16(addr, data as u16);
        self.write_u16(addr + 2, (data >> 16) as u16);
    }

    pub fn read(&self, addr: u32, length: usize) -> &[u8] {
        let addr = addr as usize;
        &self.memory[addr..addr+length]
    }

    pub fn write(&mut self, addr: u32, data: &[u8]) {
        let addr = addr as usize;
        self.memory[addr..addr+data.len()].copy_from_slice(data);
    }
}
