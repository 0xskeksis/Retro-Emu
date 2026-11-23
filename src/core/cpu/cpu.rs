use crate::core::bus::{Bus, Result};
use crate::core::cpu::{Interrupts, Register};

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Cpu
{
    pub af: Register,
    pub bc: Register,
    pub de: Register,
    pub hl: Register,
    pub pc: Register,
    pub sp: Register,
}

impl Cpu
{
    pub fn new() -> Self { Default::default() }

    /*
     * A step represent these actions:
     * - fetch
     * - decode
     * - execute
     *
     * Exemple idea: while(!interrupt) {cpu.step()}
     *
     */
    pub fn step<B: Bus>(&mut self, bus: &mut B) -> Result<()>
    {
        // if self.ime && bus.pending_interrupts() != 0 {
        //     self.service_interrupt(bus);
        //     return;
        // }
        let opcode = self.fetch(bus)?;
        match opcode {
            0x00 => {} // NOP: 1 M-cycle (the fetch)
            0x77 => {} // LD (HL), A: 2 M-cycles (fetch + write)
            0xC3 => {
                // JP nn: 4 M-cycles (fetch, lo, hi, internal)
                let lo = self.fetch(bus)?;
                let hi = self.fetch(bus)?;
                bus.advance_mcycle();
                self.pc = u16::from_le_bytes([lo, hi]);
            }
            _ => unimplemented!("opcode {opcode:#04X}"),
        };
        Ok(())
    }

    fn fetch<B: Bus>(&mut self, bus: &mut B) -> Result<u8> {
        let byte = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        byte
    }
}
