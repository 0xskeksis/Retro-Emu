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

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct InvalidTimerRegister(pub u16);

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Control(u8);


/// Private state: countdowns are always in 1..=4.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Reload
{
    #[default]
    Idle,
    Pending(u8),
    Active(u8),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct TimerUnit
{
    model: Model,
    cgb_enable_glitch: bool,

    divider: u16,
    tima: u8,
    tma: u8,
    control: Control,
    reload: Reload,
}

//
// ╔═════════════════════════[ PUBLIC API ]═════════════════════════╗
//

impl TimerUnit
{
    /// Zero-initialized state, not a post-boot snapshot.
    pub fn new(model: Model) -> Self {
        Self { model, ..Default::default() }
    }

    pub fn internal_divider(&self) -> u16 { self.divider }

    /// Chooses one deterministic variant of unit-dependent CGB behavior.
    pub fn with_cgb_enable_glitch(mut self, enabled: bool) -> Self {
        self.cgb_enable_glitch = enabled;
        self
    }

    /// Resets the hardware divider, not scheduler elapsed time.
    pub fn reset_divider(&mut self) {
        self.update_divider(0);
    }

    pub fn read(&self, register: Register) -> u8 {
        match register {
            Register::Div => (self.divider >> 8) as u8,
            Register::Tima => self.tima,
            Register::Tma => self.tma,
            Register::Tac => self.control.read(),
        }
    }

    pub fn write(&mut self, register: Register, value: u8) {
        match register {
            Register::Div => self.reset_divider(),
            Register::Tima => self.write_tima(value),
            Register::Tma => self.write_tma(value),
            Register::Tac => {
                self.write_control(Control::new(value));
            }
        }
    }
    /// Advances one CPU T-cycle.
    ///
    /// Returns true if this tick requests the timer interrupt.
    /// The scheduler must forward that request to IF.
    pub fn tick_tcycle(&mut self) -> bool {
        let request_interrupt = self.reload.tick();
        if request_interrupt { self.tima = self.tma; }
        self.update_divider(self.divider.wrapping_add(1));
        request_interrupt
    }
}


//
// ╔═════════════════════════[ INTERNALS ]═════════════════════════╗
//

impl TryFrom<u16> for Register {
    type Error = InvalidTimerRegister;

    fn try_from(address: u16) -> Result<Self, Self::Error> {
        match address {
            a if a == Self::Div as u16 => Ok(Self::Div),
            a if a == Self::Tima as u16 => Ok(Self::Tima),
            a if a == Self::Tma as u16 => Ok(Self::Tma),
            a if a == Self::Tac as u16 => Ok(Self::Tac),
            _ => Err(InvalidTimerRegister(address)),
        }
    }
}

impl Control
{
    fn new(value: u8) -> Self { Self(value & 0b111) }
    fn read(self)     -> u8   { self.0 | 0xF8 }
    fn enabled(self)  -> bool { self.0 & 0b100 != 0 }

    fn divider_mask(self) -> u16 {
        let bit = match self.0 & 0b11 {
            0b00 => 9,
            0b01 => 3,
            0b10 => 5,
            0b11 => 7,
            _ => unreachable!(),
        };
        1u16 << bit
    }
    fn clock_high(self, divider: u16) -> bool {
        divider & self.divider_mask() != 0
    }
    fn input_high(self, divider: u16) -> bool {
        self.enabled() && self.clock_high(divider)
    }
}

impl Reload
{
    fn is_idle(self)   -> bool { matches!(self, Self::Idle) }
    fn is_active(self) -> bool { matches!(self, Self::Active(_)) }

    /// Returns true when TIMA should reload and request an interrupt.
    fn tick(&mut self) -> bool {
        let (next, reload) = match *self {
            Self::Idle       => (Self::Idle, false),
            Self::Pending(1) => (Self::Active(4), true),
            Self::Active(1)  => (Self::Idle, false),
            Self::Pending(left @ 2..) => (Self::Pending(left - 1), false),
            Self::Active(left @ 2..)  => (Self::Active(left - 1), false),
            _ => unreachable!("Reload countdown should always be in 1..=4"),
        };
        *self = next;
        reload
    }
}


impl TimerUnit
{
    fn update_divider(&mut self, divider: u16) {
        let was_high = self.control.input_high(self.divider);
        self.divider = divider;
        let is_high = self.control.input_high(self.divider);

        if was_high && !is_high {
            self.increment_tima();
        }
    }

    fn increment_tima(&mut self) {
        if !self.reload.is_idle() { return; }
        let (value, overflowed) = self.tima.overflowing_add(1);
        self.tima = value;
        if overflowed { self.reload = Reload::Pending(4); }
    }

    fn write_tima(&mut self, value: u8) {
        if self.reload.is_active() { return; }
        self.tima = value;
        self.reload = Reload::Idle;
    }

    fn write_tma(&mut self, value: u8) {
        self.tma = value;
        if self.reload.is_active() {
            self.tima = value;
        }
    }

    fn write_control(&mut self, next: Control) {
        let increment = self.control_write_causes_tick(next);
        self.control = next;
        if increment {
            self.increment_tima();
        }
    }

    fn control_write_causes_tick(&self, next: Control) -> bool {
        let previous = self.control;
        match self.model {
            Model::Dmg => previous.input_high(self.divider)
                            && !next.input_high(self.divider),
            Model::Cgb => {
                let falling_edge = previous.clock_high(self.divider)
                    && !next.clock_high(self.divider);

                let enabled_before = previous.enabled()
                    || self.cgb_enable_glitch;
                next.enabled() && enabled_before && falling_edge
            }
        }
    }
}


impl std::fmt::Display for InvalidTimerRegister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid timer register: {:#06X}", self.0)
    }
}
impl std::error::Error for InvalidTimerRegister {}


#[cfg(test)]
#[path = "../../../tests/timer.rs"]
mod tests;
