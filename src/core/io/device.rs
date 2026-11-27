use crate::core::io::IrqSet;

/// A hardware block exposing memory-mapped registers.
///
/// Each device declares its own register enum. Address decoding goes through
/// `Reg: TryFrom<u16>`, so the device is the only place that knows its addresses.
pub trait IoDevice
{
    type Reg: Copy + TryFrom<u16>;

    /// Register read. Unused bits must read as 1 (device's responsibility).
    fn read(&self, reg: Self::Reg) -> u8;

    /// Register write. Returns interrupts raised as a side effect
    /// (e.g. DIV/TAC writes glitching TIMA). Most devices return EMPTY.
    fn write(&mut self, reg: Self::Reg, value: u8) -> IrqSet;

    /// Decodes an address into this device's register, if it owns it.
    fn decode(addr: u16) -> Option<Self::Reg> {
        Self::Reg::try_from(addr).ok()
    }
}
