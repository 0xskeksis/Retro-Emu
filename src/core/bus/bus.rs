use crate::core::bus::{Cartridge, Ram};
use crate::core::ppu::Ppu;

const RAM_SIZE: usize = 0x10000; // 64KB of RAM

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Bus
{
    pub cart: Cartridge,
    pub ram: Ram,
    pub ppu: Ppu,
    // pub timer: Timer,
}

impl Bus {
    pub fn new() -> Self
    {
        Self {
            cart: Cartridge::new(),
            ram: Ram::new(RAM_SIZE),
            ppu: Ppu::new(),
        }
    }
}
