use thiserror::Error;

use crate::core::Model;

//
// ╔═════════════════════════[ DEFINITION ]═════════════════════════╗
//

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u16)]
pub enum Register
{
    Div = 0xFF04,
    Tima = 0xFF05,
    Tma = 0xFF06,
    Tac = 0xFF07,
}

#[derive(Error, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TimerError
{
    #[error("Invalid timer register: {0:#06X}")]
    InvalidRegister(u16),
}

/// Private state: countdowns are always in 1..=4.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum StateMachine
{
    #[default]
    Idle,
    Pending(u8),
    Active(u8),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Timer
{
    div: u16,
    tima: u8,
    tma: u8,
    tac: u8,
    reload: StateMachine,
    glitch: bool,
}

//
// ╔═════════════════════════[ PUBLIC API ]═════════════════════════╗
//

impl Timer
{
    /// Zero-initialized state, not a post-boot snapshot.
    pub fn new() -> Self { Default::default() }

    pub fn read(&self, register: Register) -> u8 {
        match register {
            Register::Div => (self.div >> 8) as u8,
            Register::Tima => self.tima,
            Register::Tma => self.tma,
            Register::Tac => self.tac | 0xF8,
        }
    }

    pub fn write(&mut self, register: Register, value: u8) {
        match register {
            Register::Div => self.write_div(0),
            Register::Tima => self.write_tima(value),
            Register::Tma => self.write_tma(value),
            Register::Tac => self.write_tac(value)
        }
    }

    /// Advances one CPU T-cycle.
    ///
    /// Returns true if this tick requests the timer interrupt.
    /// The scheduler must forward that request to IF.
    pub fn tick_tcycle(&mut self) -> bool
    {
        let request_interrupt = self.reload.tick();
        if request_interrupt { self.tima = self.tma; }
        self.write_div(self.div.wrapping_add(1));
        request_interrupt
    }
}


//
// ╔═════════════════════════[ INTERNALS ]═════════════════════════╗
//

impl TryFrom<u16> for Register {
    type Error = TimerError;

    fn try_from(address: u16) -> Result<Self, Self::Error> {
        match address {
            a if a == Self::Div as u16 => Ok(Self::Div),
            a if a == Self::Tima as u16 => Ok(Self::Tima),
            a if a == Self::Tma as u16 => Ok(Self::Tma),
            a if a == Self::Tac as u16 => Ok(Self::Tac),
            _ => Err(TimerError::InvalidRegister(address)),
        }
    }
}

impl StateMachine
{
    fn is_idle(self)   -> bool { matches!(self, Self::Idle) }
    fn is_active(self) -> bool { matches!(self, Self::Active(_)) }

    /// Returns true when TIMA should reload and request an interrupt.
    fn tick(&mut self) -> bool 
    {
        let (next, reload) = match *self {
            Self::Idle                => (Self::Idle, false),
            Self::Pending(1)          => (Self::Active(4), true),
            Self::Active(1)           => (Self::Idle, false),
            Self::Pending(left @ 2..) => (Self::Pending(left - 1), false),
            Self::Active(left @ 2..)  => (Self::Active(left - 1), false),
            _ => unreachable!("StateMachine countdown should always be in 1..=4"),
        };
        *self = next;
        reload
    }
}


impl Timer
{
    fn write_div(&mut self, div: u16) {
        let enabled = self.tac & 0b100 != 0;
        let was_high = enabled && (self.div & self.divider_mask()) != 0;
        self.div = div;
        let is_high = enabled && (self.div & self.divider_mask()) != 0;

        if was_high && !is_high {
            self.incr_tima();
        }
    }

    fn incr_tima(&mut self) {
        if !self.reload.is_idle() { return; }
        let (value, overflowed) = self.tima.overflowing_add(1);
        self.tima = value;
        if overflowed { self.reload = StateMachine::Pending(4); }
    }

    fn write_tima(&mut self, value: u8) {
        if self.reload.is_active() { return; }
        self.tima = value;
        self.reload = StateMachine::Idle;
    }

    fn write_tma(&mut self, value: u8) {
        self.tma = value;
        if self.reload.is_active() {
            self.tima = value;
        }
    }

    fn write_tac(&mut self, value: u8) {
        let new:u8 = value & 0b111;
        // TODO:
        // let increment = self.control_write_causes_tick(next);
        // self.control = next;
        // if increment {
        //     self.incr_tima();
        // }
    }

    fn divider_mask(&self) -> u16 {
        let bit = match self.tac & 0b11 {
            0b00 => 9,
            0b01 => 3,
            0b10 => 5,
            0b11 => 7,
            _ => unreachable!(),
        };
        1u16 << bit
    }
}


#[cfg(test)]
#[path = "../../../tests/timer.rs"]
mod tests;
