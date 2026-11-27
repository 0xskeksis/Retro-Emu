//! Game Boy timer and divider registers.
//!
//! - `Register`: the four IO registers (DIV, TIMA, TMA, TAC)
//! - `Clock`:    the hardware block holding the registers and internal counter

use crate::core::io::{Irq, IrqSet, IoDevice};


//
// ╔═════════════════════════[ Register : Io Interface ]═════════════════════════╗
//

/// A IO register in the timer hardware block. The discriminant is its address.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum Register
{
    Div = 0xFF04,
    Tima,
    Tma,
    Tac,
}

impl TryFrom<u16> for Register
{
    type Error = ();

    fn try_from(addr: u16) -> Result<Self, ()> {
        match addr {
            0xFF04 => Ok(Self::Div),
            0xFF05 => Ok(Self::Tima),
            0xFF06 => Ok(Self::Tma),
            0xFF07 => Ok(Self::Tac),
            _ => Err(()),
        }
    }
}

//
// ╔═════════════════════════[ Clock : Timer Hardware ]═════════════════════════╗
//

/// A hardware block exposing the timer and divider registers.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Clock
{
    /// Internal 16-bit counter. DIV is its upper byte.
    counter: u16,
    tima: u8,
    tma: u8,
    tac: u8,
}

impl IoDevice for Clock
{
    type Reg = Register;

    fn read(&self, reg: Register) -> u8 {
        match reg {
            Register::Div  => (self.counter >> 8) as u8,
            Register::Tima => self.tima,
            Register::Tma  => self.tma,
            Register::Tac  => self.tac | !Self::TAC_MASK,
        }
    }

    fn write(&mut self, reg: Register, value: u8) -> IrqSet {
        match reg {
            Register::Div  => self.update(|c| c.counter = 0),
            Register::Tac  => self.update(|c| c.tac = value & Self::TAC_MASK),
            Register::Tima => { self.tima = value; IrqSet::EMPTY }
            Register::Tma  => { self.tma  = value; IrqSet::EMPTY }
        }
    }
}

impl Clock
{
    const TAC_ENABLE: u8 = 0b100;
    const TAC_MASK:   u8 = 0b111;
    const TAC_BIT: [u8; 4] = [9, 3, 5, 7]; // Counter bit watched by TIMA, indexed by TAC bits 0-1.

    /// Zero-initialized state, not a post-boot snapshot.
    pub fn new() -> Self { Self::default() }

    /// Applies `change`, then increments TIMA if the timer signal fell from 1 to 0.
    fn update(&mut self, change: impl FnOnce(&mut Self)) -> IrqSet
    {
        let was_high = self.signal();
        change(self);

        if was_high && !self.signal() {
            return self.increment_tima()
        }
        IrqSet::EMPTY
    }

    /// Timer enabled AND the selected counter bit is set.
    fn signal(&self) -> bool
    {
        let bit = Self::TAC_BIT[(self.tac & 0b11) as usize];
        self.tac & Self::TAC_ENABLE != 0 && self.counter & (1 << bit) != 0
    }
    
    /// Increments TIMA, returning a Timer interrupt if it overflows.
    fn increment_tima(&mut self) -> IrqSet {
        match self.tima.checked_add(1) {
            Some(next) => { self.tima = next;     IrqSet::EMPTY }
            None       => { self.tima = self.tma; Irq::Timer.into() }
        }
    }

    /// Advances one T-cycle.
    pub fn tick(&mut self) -> IrqSet
    {
        self.update(|c| {
            c.counter = c.counter.wrapping_add(1)
        })
    }
}
