use crate::core::bus::{Cartridge, Ram};
use crate::core::ppu::Ppu;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Bus
{
    pub cartridge: Cartridge,
    pub ram: Ram,
    pub ppu: Ppu,
    // pub timer: Timer,
}

impl Bus {
    pub fn new() -> Self
    {
        Self {
            cartridge: Cartridge::new(),
            ram: Ram::new(),
            ppu: Ppu::new(),
        }
    }
}
