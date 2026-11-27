pub use crate::core::io::{Clock, Register as ClockReg};
pub use crate::core::io::IoDevice;
pub use crate::core::io::{Irq, IrqSet, IrqCtrl, IrqReg};

/// Any register reachable through the IO space (plus IE at FFFF).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IoReg
{
    Clock(ClockReg),
    Irq(IrqReg),
    // Ppu(PpuReg),
    // Joypad(JoypadReg),
    // Serial(SerialReg),
    // Dma(DmaReg),
}

impl IoReg
{
    pub fn decode(addr: u16) -> Option<Self> {
        None.or_else(|| Clock::decode(addr).map(Self::Clock))
            .or_else(|| IrqCtrl::decode(addr).map(Self::Irq))
            // .or_else(|| Ppu::decode(addr).map(Self::Ppu))
    }
}
