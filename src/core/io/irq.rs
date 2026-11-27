//! Game Boy interrupt system.
//!
//! - `Irq`:     a single interrupt source (VBlank, STAT, Timer, Serial, Joypad)
//! - `IrqSet`:  a set of sources, the value stored in IF / IE
//! - `IrqCtrl`: the hardware block holding IF (FF0F) and IE (FFFF)

use std::ops::{BitAnd, BitOr, BitOrAssign};

//
// ╔═════════════════════════[ Irq : Single interrupt ]═════════════════════════╗
//

/// A single interrupt source. The discriminant is its bit index in IF/IE,
/// which is also its priority (0 = highest).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum Irq
{
    VBlank = 0,
    Stat   = 1,
    Timer  = 2,
    Serial = 3,
    Joypad = 4,
}

impl Irq
{
    /// Bit mask of this source in IF/IE.
    pub const fn bit(self) -> u8 {
        1 << self as u8
    }

    /// Address the CPU jumps to when servicing this source:
    /// 0x40, 0x48, 0x50, 0x58, 0x60.
    pub const fn handler_address(self) -> u16 {
        0x40 + 8 * self as u16
    }
}

/// `Irq::Timer | Irq::VBlank` produces an `IrqSet`.
impl BitOr for Irq
{
    type Output = IrqSet;

    fn bitor(self, o: Self) -> IrqSet {
        IrqSet(self.bit() | o.bit())
    }
}

/// Tryfrom<u32> for Irq: converts a bit index to an `Irq` if valid.
impl TryFrom<u32> for Irq
{
    type Error = ();

    fn try_from(i: u32) -> Result<Self, Self::Error> {
        match i {
            0 => Ok(Irq::VBlank),
            1 => Ok(Irq::Stat),
            2 => Ok(Irq::Timer),
            3 => Ok(Irq::Serial),
            4 => Ok(Irq::Joypad),
            _ => Err(()),
        }
    }
}

//
// ╔═════════════════════════[ IrqSet : Interrupts Set ]═════════════════════════╗
//

/// A set of interrupt sources, as stored in IF or IE.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
pub struct IrqSet(u8);

impl IrqSet
{
    pub const EMPTY: Self = Self(0);
    pub const ALL: [Irq; 5] = [
        Irq::VBlank,
        Irq::Stat,
        Irq::Timer,
        Irq::Serial,
        Irq::Joypad
    ];

    /// Bits 0-4: the only bits that correspond to real sources.
    const MASK: u8 = 0x1F;

    pub const fn bits(self) -> u8 { self.0 }

    /// Keeps all 8 bits unmasked. Only for IE, which stores bits 5-7 on hardware.
    /// Use `From<u8>` everywhere else.
    pub const fn from_raw(bits: u8) -> Self { Self(bits) }

    pub fn insert(&mut self, irq: Irq) { self.0 |= irq.bit(); }
    pub fn remove(&mut self, irq: Irq) { self.0 &= !irq.bit(); }

    pub const fn contains(self, irq: Irq) -> bool { self.0 & irq.bit() != 0 }
    pub const fn is_empty(self)           -> bool { self.0 & Self::MASK == 0 }

    /// Highest-priority source in the set (lowest set bit among bits 0-4).
    pub fn highest_priority(self) -> Option<Irq> {
        let valid = self.0 & Self::MASK;
        match valid {
            0 => None,
            _ => Irq::try_from(valid.trailing_zeros()).ok(),
        }
    }

    /// Iterates sources from highest to lowest priority.
    pub fn iter(self) -> impl Iterator<Item = Irq> {
        Self::ALL.into_iter().filter(move |&irq| self.contains(irq))
    }
}

/// Default construction from a byte: masks bits 5-7.
impl From<u8> for IrqSet {
    fn from(bits: u8) -> Self { Self(bits & Self::MASK) }
}

impl From<Irq> for IrqSet {
    fn from(irq: Irq) -> Self { Self(irq.bit()) }
}

impl BitOr for IrqSet {
    type Output = Self;
    fn bitor(self, o: Self) -> Self { Self(self.0 | o.0) }
}

impl BitOr<Irq> for IrqSet {
    type Output = Self;
    fn bitor(self, irq: Irq) -> Self { Self(self.0 | irq.bit()) }
}

impl BitOrAssign for IrqSet {
    fn bitor_assign(&mut self, o: Self) { self.0 |= o.0; }
}

impl BitOrAssign<Irq> for IrqSet {
    fn bitor_assign(&mut self, irq: Irq) { self.0 |= irq.bit(); }
}

impl BitAnd for IrqSet {
    type Output = Self;
    fn bitand(self, o: Self) -> Self { Self(self.0 & o.0) }
}

//
// ╔═════════════════════════[ IrqCtrl : IF / IE hardware ]═════════════════════════╗
//

/// Interrupt controller: IF (FF0F) and IE (FFFF).
/// IME belongs to the CPU and is not stored here.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct IrqCtrl
{
    if_: IrqSet, // FF0F, requested sources (bits 0-4 only)
    ie:  IrqSet, // FFFF, enabled sources (all 8 bits stored)
}

impl IrqCtrl
{
    pub fn new() -> Self { Default::default() }

    // ---- Peripherals (via emulator.rs): OR in, never overwrite

    /// Raises one or more sources. Accepts an `Irq` or an `IrqSet`
    pub fn request(&mut self, sources: impl Into<IrqSet>) {
        self.if_ |= sources.into();
    }

    // ---- CPU servicing

    /// Highest-priority source that is both requested and enabled.
    /// The CPU checks IME itself. Note that HALT wakes up on this
    /// even when IME is off.
    pub fn pending(&self) -> Option<Irq> {
        (self.if_ & self.ie).highest_priority()
    }

    /// Clears the IF bit of a serviced source
    pub fn acknowledge(&mut self, irq: Irq) {
        self.if_.remove(irq);
    }

    // ---- Bus register access (game code): plain register semantics

    /// FF0F read: bits 5-7 always read as 1
    pub fn read_reg_if(&self) -> u8 { self.if_.bits() | 0xE0 }

    /// FF0F write: replaces IF, so the game can request *or discard* interrupts
    pub fn write_reg_if(&mut self, v: u8) { self.if_ = v.into(); }

    /// FFFF read: all 8 bits read back as written
    pub fn read_reg_ie(&self) -> u8 { self.ie.bits() }

    /// FFFF write: replaces IE
    pub fn write_reg_ie(&mut self, v: u8) { self.ie = IrqSet::from_raw(v); }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handler_addresses() {
        let addrs: Vec<u16> = IrqSet::ALL.iter().map(|i| i.handler_address()).collect();
        assert_eq!(addrs, [0x40, 0x48, 0x50, 0x58, 0x60]);
    }

    #[test]
    fn priority_is_lowest_bit() {
        let set = Irq::Joypad | Irq::Stat;
        assert_eq!(set.highest_priority(), Some(Irq::Stat));
        assert_eq!(IrqSet::EMPTY.highest_priority(), None);
    }

    #[test]
    fn irqset_or() {
        let set = IrqSet::EMPTY | Irq::VBlank | Irq::Timer;
        assert_eq!(set.bits(), 0b00000101);
    }

    #[test]
    fn pending_requires_enable() {
        let mut ic = IrqCtrl::default();
        ic.request(Irq::Timer);
        assert_eq!(ic.pending(), None);

        ic.write_reg_ie(Irq::Timer.bit());
        assert_eq!(ic.pending(), Some(Irq::Timer));

        ic.acknowledge(Irq::Timer);
        assert_eq!(ic.pending(), None);
    }

    #[test]
    fn register_quirks() {
        let mut ic = IrqCtrl::default();
        ic.write_reg_if(0xFF);
        assert_eq!(ic.read_reg_if(), 0xFF); // bits 0-4 set, 5-7 forced to 1
        ic.write_reg_if(0x00);
        assert_eq!(ic.read_reg_if(), 0xE0);

        ic.write_reg_ie(0xFF);
        assert_eq!(ic.read_reg_ie(), 0xFF); // IE keeps all 8 bits
    }

    #[test]
    fn upper_ie_bits_never_trigger() {
        let mut ic = IrqCtrl::default();
        ic.write_reg_ie(0xE0);
        ic.write_reg_if(0xFF);
        assert_eq!(ic.pending(), None);
    }
}
