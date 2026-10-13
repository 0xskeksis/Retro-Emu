use crate::core::cpu::Register;
use crate::core::bus::Ram;
use crate::core::bus::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Stack;

impl Stack
{
    pub fn push(ram: &mut Ram, sp: &mut Register, value: u16)
    {
        let [lo, hi] = value.to_le_bytes();

        *sp = sp.wrapping_sub(1);
        ram.write(*sp, hi);

        *sp = sp.wrapping_sub(1);
        ram.write(*sp, lo);
    }

    pub fn pop(ram: &Ram, sp: &mut u16) -> u16 {
        let lo = ram.read(*sp);
        *sp = sp.wrapping_add(1);

        let hi = ram.read(*sp);
        *sp = sp.wrapping_add(1);

        u16::from_le_bytes([lo, hi])
    }
}
