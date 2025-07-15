pub struct MemoryBus {
    pub memory: [u8; 0xFFFF],
}

impl MemoryBus {
    pub fn read_byte(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }
}
