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

pub struct Registers {
    pub af: u16,
    pub bc: u16,
    pub de: u16,
    pub hl: u16,
    pub pc: u16,
    pub sp: u16,
}

impl Registers{
    pub fn new() -> Registers {
        af: 0;
        bc: 0;
        de: 0;
        hl: 0;
        pc: 0x100; // cartridge start
        sp: 0;
    }
}
