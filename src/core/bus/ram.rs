
pub const RAM_SIZE: usize = 0x10000; // 64KB of RAM

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Ram
{
    data: Vec<u8>,
}

impl Default for Ram {
    fn default() -> Self { 
        Self { data: vec![0; RAM_SIZE] } 
    }
}

impl Ram
{
    pub fn new() -> Self { Default::default() }

    pub fn read(&self, address: u16) -> u8 {
        self.data[address as usize]
    }

    pub fn write(&mut self, address: u16, value: u8) {
        self.data[address as usize] = value;
    }
}
