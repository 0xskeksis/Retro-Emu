#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Model {
    Dmg,
    Cgb,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Register {
    Div,
    Tima,
    Tma,
    Tac,
}

impl TryFrom<u16> for Register {
    type Error = InvalidRegister;

    fn try_from(address: u16) -> Result<Self, Self::Error> {
        match address {
            0xFF04 => Ok(Self::Div),
            0xFF05 => Ok(Self::Tima),
            0xFF06 => Ok(Self::Tma),
            0xFF07 => Ok(Self::Tac),
            _ => Err(InvalidRegister(address)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvalidRegister(pub u16);

impl std::fmt::Display for InvalidRegister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid timer register: {:#06X}", self.0)
    }
}

impl std::error::Error for InvalidRegister {}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct Control(u8);

impl Control {
    fn new(value: u8) -> Self {
        Self(value & 0b111)
    }

    fn read(self) -> u8 {
        self.0 | 0xF8
    }

    fn enabled(self) -> bool {
        self.0 & 0b100 != 0
    }

    fn divider_mask(self) -> u16 {
        let bit = match self.0 & 0b11 {
            0b00 => 9,
            0b01 => 3,
            0b10 => 5,
            0b11 => 7,
            _ => unreachable!(),
        };

        1 << bit
    }

    fn clock_high(self, divider: u16) -> bool {
        divider & self.divider_mask() != 0
    }

    fn input_high(self, divider: u16) -> bool {
        self.enabled() && self.clock_high(divider)
    }
}

/// Counts T-cycles remaining in each overflow phase.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum Reload {
    #[default]
    Idle,
    Pending(u8),
    Active(u8),
}

impl Reload {
    /// Returns true exactly when TIMA must reload and request an interrupt.
    fn tick(&mut self) -> bool {
        let (next, reload) = match *self {
            Self::Idle => (Self::Idle, false),
            Self::Pending(1) => (Self::Active(4), true),
            Self::Pending(left) => (Self::Pending(left - 1), false),
            Self::Active(1) => (Self::Idle, false),
            Self::Active(left) => (Self::Active(left - 1), false),
        };

        *self = next;
        reload
    }

    fn is_idle(self) -> bool {
        matches!(self, Self::Idle)
    }

    fn is_active(self) -> bool {
        matches!(self, Self::Active(_))
    }
}

#[derive(Debug, Clone)]
pub struct Timer {
    model: Model,
    cgb_enable_glitch: bool,

    elapsed_t_cycles: u64,
    divider: u16,

    tima: u8,
    tma: u8,
    control: Control,

    reload: Reload,
    interrupt_requested: bool,
}

impl Timer {
    /// Zero-initialized hardware state, not a post-boot snapshot.
    pub fn new(model: Model) -> Self {
        Self {
            model,
            cgb_enable_glitch: false,
            elapsed_t_cycles: 0,
            divider: 0,
            tima: 0,
            tma: 0,
            control: Control::default(),
            reload: Reload::Idle,
            interrupt_requested: false,
        }
    }

    /// Selects the unit-dependent CGB timer-enable behavior.
    pub fn with_cgb_enable_glitch(mut self, enabled: bool) -> Self {
        self.cgb_enable_glitch = enabled;
        self
    }

    pub fn elapsed_t_cycles(&self) -> u64 {
        self.elapsed_t_cycles
    }

    pub fn internal_divider(&self) -> u16 {
        self.divider
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
            Register::Tac => self.write_control(Control::new(value)),
        }
    }

    pub fn advance_m_cycles(&mut self, cycles: u32) {
        self.advance_t_cycles(u64::from(cycles) * 4);
    }

    pub fn advance_t_cycles(&mut self, cycles: u64) {
        self.elapsed_t_cycles = self
            .elapsed_t_cycles
            .checked_add(cycles)
            .expect("elapsed timer cycle counter overflow");

        for _ in 0..cycles {
            self.tick();
        }
    }

    /// Resets DIV without resetting elapsed time.
    pub fn reset_divider(&mut self) {
        self.update_divider(0);
    }

    /// Consumes the request to set the timer interrupt bit in IF.
    ///
    /// Does not clear IF; this timer does not own that register.
    pub fn take_interrupt_request(&mut self) -> bool {
        std::mem::take(&mut self.interrupt_requested)
    }

    fn tick(&mut self) {
        self.advance_reload();
        self.update_divider(self.divider.wrapping_add(1));
    }

    fn advance_reload(&mut self) {
        if !self.reload.tick() {
            return;
        }

        self.tima = self.tma;
        self.interrupt_requested = true;
    }

    fn update_divider(&mut self, divider: u16) {
        let was_high = self.control.input_high(self.divider);
        self.divider = divider;
        let is_high = self.control.input_high(self.divider);

        if was_high && !is_high {
            self.increment_tima();
        }
    }

    fn increment_tima(&mut self) {
        if !self.reload.is_idle() {
            return;
        }

        let (value, overflowed) = self.tima.overflowing_add(1);
        self.tima = value;

        if overflowed {
            self.reload = Reload::Pending(4);
        }
    }

    fn write_tima(&mut self, value: u8) {
        if self.reload.is_active() {
            return;
        }

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
        let old_high = previous.clock_high(self.divider);
        let new_high = next.clock_high(self.divider);

        match self.model {
            Model::Dmg => {
                previous.input_high(self.divider)
                    && !next.input_high(self.divider)
            }

            Model::Cgb => {
                let falling_edge = old_high && !new_high;
                let enabled_before =
                    previous.enabled() || self.cgb_enable_glitch;

                next.enabled() && enabled_before && falling_edge
            }
        }
    }
}
