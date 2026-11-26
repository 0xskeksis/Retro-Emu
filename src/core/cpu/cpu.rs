use crate::core::cpu::{Register, Flag};
use crate::core::mem::{Bus, Result};
use crate::core::cpu::ops::Opcodes;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Cpu {
    pub af: u16,
    pub bc: u16,
    pub de: u16,
    pub hl: u16,
    pub pc: u16,
    pub sp: u16,
    pub ime: u16,
}

impl Cpu {
    pub fn new() -> Self {
        Default::default()
    }

    /*
     * A step represent these actions:
     * - fetch
     * - decode
     * - execute
     *
     * Exemple idea: while(!interrupt) {cpu.step()}
     *
     */
    pub fn step<B: Bus>(&mut self, bus: &mut B) -> Result<()> {
        // if self.ime && bus.pending_interrupts() != 0 {
        //     self.service_interrupt(bus);
        //     return;
        // }
        let opcode = self.fetch(bus)?;
        Opcodes::<B>::execute(self, bus, opcode)?;
        bus.advance_mcycle();
        Ok(())
    }

    pub fn fetch<B: Bus>(&mut self, bus: &mut B) -> Result<u8> {
        let byte = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        self.pc.get_hi();
        byte
    }

    #[inline(always)]
    pub fn flag(&self, flag: Flag) -> bool {
        self.af.get_flag(flag)
    }

    /// Set or clear one flag, leaving the others untouched.
    #[inline(always)]
    pub fn put_flag(&mut self, flag: Flag, value: bool) {
        if value {
            self.af.set_flag(flag);
        } else {
            self.af.clear_flag(flag);
        }
    }

    /// Overwrite all four flags at once (low nibble of F stays 0).
    #[inline(always)]
    pub fn set_flags(&mut self, z: bool, n: bool, h: bool, c: bool) {
        let f = ((z as u8) << 7) | ((n as u8) << 6) | ((h as u8) << 5) | ((c as u8) << 4);
        self.af.set_lo(f);
    }

    #[inline(always)] pub fn zf(&self) -> bool { self.flag(Flag::Zero) }
    #[inline(always)] pub fn nf(&self) -> bool { self.flag(Flag::Subtract) }
    #[inline(always)] pub fn hf(&self) -> bool { self.flag(Flag::HalfCarry) }
    #[inline(always)] pub fn cf(&self) -> bool { self.flag(Flag::Carry) }

    pub fn a(&self) -> u8 {
        self.af.get_hi()
    }

    pub fn f(&self) -> u8 {
        self.af.get_lo()
    }

    pub fn b(&self) -> u8 {
        self.bc.get_hi()
    }

    pub fn c(&self) -> u8 {
        self.bc.get_lo()
    }
    
    pub fn d(&self) -> u8 {
        self.de.get_hi()
    }

    pub fn e(&self) -> u8 {
        self.de.get_lo()
    }

    pub fn h(&self) -> u8 {
        self.hl.get_hi()
    }

    pub fn l(&self) -> u8 {
        self.hl.get_lo()
    }
}
