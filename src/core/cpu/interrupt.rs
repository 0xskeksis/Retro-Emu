#[derive(Debug, Clone, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Interrupt
{
    flags: u8,
}

impl Interrupt {
    const VALID_BITS: u8 = 0x1F;
    const TIMER_BIT: u8 = 1 << 2;

    pub fn read_if(&self) -> u8 {
        self.flags | !Self::VALID_BITS
    }

    pub fn write_if(&mut self, value: u8) {
        self.flags = value & Self::VALID_BITS;
    }

    pub(crate) fn request_timer(&mut self) {
        self.flags |= Self::TIMER_BIT;
    }
}
