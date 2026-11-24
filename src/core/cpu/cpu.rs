use crate::core::cpu::Register;
use crate::core::mem::{Bus, Result};

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
        self.pc.get_hi();
        byte
    }
}
