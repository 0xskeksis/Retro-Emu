/*
* Sources:
* - https://gbdev.io/pandocs/CPU_Registers_and_Flags.html
*
* The lower 8 bits of AF are for the flags:
*  Bit     Name
*  7	    z	Zero flag
   6	    n	Subtraction flag (BCD)
   5	    h	Half Carry flag (BCD)
   4	    c	Carry flag
*/

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u16)]
pub enum Flag {
    Zero = 1 << 7,
    Subtract = 1 << 6,
    HalfCarry = 1 << 5,
    Carry = 1 << 4,
}

pub trait Register {
    fn from_u16(value: u16) -> Self;
    fn get_flag(&self, flag: Flag) -> bool;
    fn set_flag(&mut self, flag: Flag);
    fn clear_flag(&mut self, flag: Flag);

    fn get_hi(&self) -> u8;
    fn get_lo(&self) -> u8;
    fn set_hi(&mut self, value: u8);
    fn set_lo(&mut self, value: u8);
}

impl Register for u16 {
    fn from_u16(value: u16) -> Self {
        value & 0xFFF0
    }
    fn get_flag(&self, flag: Flag) -> bool {
        (self & (flag as u16)) != 0
    }
    fn set_flag(&mut self, flag: Flag) {
        *self |= flag as u16;
    }
    fn clear_flag(&mut self, flag: Flag) {
        *self &= !(flag as u16);
    }
    fn get_hi(&self) -> u8 {
        (self >> 8) as u8
    }
    fn get_lo(&self) -> u8 {
        *self as u8
    }

    fn set_hi(&mut self, value: u8) {
        *self = (*self & !(0xFF00 as u16)) | ((value as u16) << 8);
    }
    fn set_lo(&mut self, value: u8) {
        *self = (*self & !(0x00FF as u16)) | (value as u16);
    }
}
