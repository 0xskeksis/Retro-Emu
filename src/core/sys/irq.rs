// Official Game Boy Interrupts Documentation:
//  - https://gbdev.io/pandocs/Interrupts.html

#[repr(u8)]
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum InterruptKind
{
    VBlank = 0,
    Stat = 1,
    Timer = 2,
    Serial = 3,
    Joypad = 4,
}
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Interrupts
{
    pub ie: u8,  // 0xFFFF
    pub if_: u8, // 0xFF0F
}

impl Interrupts
{
    pub fn new() -> Self { Default::default() }

    pub fn request(&mut self, irq: InterruptKind) {
        self.if_ |= 1 << irq as u8;
    }

    pub fn pending(&self) -> u8 {
        self.ie & self.if_ & 0x1F
    }

    pub fn acknowledge(&mut self, irq: InterruptKind) {

    }
}
