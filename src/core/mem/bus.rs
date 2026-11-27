use crate::core::mem::{AddressError, Result,
    Hram, HRAM_BASE_ADDR, HRAM_END_ADDR,
    Ram,  RAM_BASE_ADDR,  RAM_END_ADDR,
    OAM_BASE_ADDR, OAM_END_ADDR,
    ROM_BASE_ADDR, ROM_END_ADDR,
    VRAM_BASE_ADDR, VRAM_END_ADDR,
    IO_BASE_ADDR, IO_END_ADDR,
    ERAM_BASE_ADDR, ERAM_END_ADDR,
    EROM_BASE_ADDR, EROM_END_ADDR,
    INTERRUPT_IE_ADDR
};
use crate::core::io::{IoDevice, IoReg, Irq, IrqCtrl, IrqReg, IrqSet};
use crate::core::ppu::Ppu;
use crate::core::cart::Cartridge;
use crate::core::sys::Scheduler;
use crate::core::io::Clock;

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct BusView<'a>
{
        pub ppu:   &'a mut Ppu,
        pub ram:   &'a mut Ram,
        pub hram:  &'a mut Hram,
        pub cart:  &'a mut Cartridge,
        pub sched: &'a mut Scheduler,
        pub inter: &'a mut IrqCtrl,
        pub clock: &'a mut Clock,
}

pub trait Bus
{
    /// Memory read: costs 1 M-cycle (4 T-cycles)
    fn read(&mut self, addr: u16) -> Result<u8>;
    /// Memory write: costs 1 M-cycle (4 T-cycles)
    fn write(&mut self, addr: u16, val: u8) -> Result<()>;
    /// Internal M-cycle with no memory access, may raise interrupts
    fn advance_mcycle(&mut self);
    /// Internal T-cycle with no memory access, may raise interrupts
    fn advance_tcycle(&mut self);
    /// Request an interrupt
    fn irq_request(&mut self, irq: impl Into<IrqSet>);
    /// Acknowledge an interrupt (clear the corresponding bit in IF)
    fn irq_acknowledge(&mut self, irq: Irq);
    /// Check if an interrupt is pending (IF & IE != 0)
    fn irq_pending(&self) -> Option<Irq>;
}


// MAN - Memory Map
// https://gbdev.io/pandocs/Memory_Map.html

impl Bus for BusView<'_>
{
    fn read(&mut self, addr: u16) -> Result<u8> {
        self.advance_mcycle();
        self.mapped_read(addr)
    }

    fn write(&mut self, addr: u16, val: u8) -> Result<()> {
        self.advance_mcycle();
        self.mapped_write(addr, val)
    }

    fn advance_mcycle(&mut self) { self.advance(4); }
    fn advance_tcycle(&mut self) { self.advance(1); }

    fn irq_request(&mut self, irq: impl Into<IrqSet>) { self.inter.request(irq); }
    fn irq_acknowledge(&mut self, irq: Irq) { self.inter.acknowledge(irq); }
    fn irq_pending(&self) -> Option<Irq> { self.inter.pending() }
}

impl BusView<'_>
{
    /// Advance the bus by a number of T-cycles, raising interrupts as needed.
    fn advance(&mut self, tcycles: u32) {
        (0..tcycles).for_each(|_| {
            let raised = self.tick();
            self.irq_request(raised);
        });
    }

    fn tick(&mut self) -> IrqSet {
        self.sched.tick()
        | self.ppu.tick()
        | self.clock.tick()
    }

    /// Unmapped registers read 0xFF on hardware.
    fn io_read(&self, addr: u16) -> u8 {
        match IoReg::decode(addr) {
            Some(IoReg::Clock(r)) => self.clock.read(r),
            Some(IoReg::Irq(r))   => self.inter.read(r),
            None                  => 0xFF,
        }
    }

    /// Unmapped writes are ignored. Side-effect interrupts are requested here,
    /// so devices never need a reference to IrqCtrl.
    fn io_write(&mut self, addr: u16, v: u8) {
        let raised = match IoReg::decode(addr) {
            Some(IoReg::Clock(r)) => self.clock.write(r, v),
            Some(IoReg::Irq(r))   => self.inter.write(r, v),
            None                  => IrqSet::EMPTY,
        };
        self.inter.request(raised);
    }

    /// Address routing only: does not advance time.
    fn mapped_read(&self, addr: u16) ->  Result<u8> {
        // TODO: 
        //  - Allow echo ram reads (E000..=FDFF) to mirror return RAM (0001..=DDFF). =>
        //      0xE000..=0xFDFF => addr - 0x2000
        //  - Ignore unusable areas
        //  - Return 0xFF for unmapped areas.
        //  - delegate rom and ram reads to the cartridge, which may have bank switching.
        match addr {
            ROM_BASE_ADDR  ..= ROM_END_ADDR  => self.cart.rom().read(addr),
            EROM_BASE_ADDR ..= EROM_END_ADDR => self.cart.read_ext_rom(addr),
            VRAM_BASE_ADDR ..= VRAM_END_ADDR => self.ppu.vram().read(addr),
            ERAM_BASE_ADDR ..= ERAM_END_ADDR => self.cart.read_ext_ram(addr),
            RAM_BASE_ADDR  ..= RAM_END_ADDR  => self.ram.read(addr),
            OAM_BASE_ADDR  ..= OAM_END_ADDR  => self.ppu.oam().read(addr),
            IO_BASE_ADDR   ..= IO_END_ADDR   => Ok(self.io_read(addr)),
            HRAM_BASE_ADDR ..= HRAM_END_ADDR => self.hram.read(addr),
            INTERRUPT_IE_ADDR                => Ok(self.io_read(addr)),
            _ => Err(AddressError::Invalid(addr)), // 0xFEA0..=0xFEFF & E000..=FDFF are unusable
        }
    }

    /// Address routing only: does not advance time.
    fn mapped_write(&mut self, addr: u16, val: u8) -> Result<()> {
        match addr {
            ROM_BASE_ADDR  ..= ROM_END_ADDR  => Err(AddressError::ReadOnly(addr)),
            EROM_BASE_ADDR ..= EROM_END_ADDR => Err(AddressError::ReadOnly(addr)),
            VRAM_BASE_ADDR ..= VRAM_END_ADDR => self.ppu.vram_mut().write(addr, val),
            ERAM_BASE_ADDR ..= ERAM_END_ADDR => self.cart.write_ext_ram(addr, val),
            RAM_BASE_ADDR  ..= RAM_END_ADDR  => self.ram.write(addr, val),
            OAM_BASE_ADDR  ..= OAM_END_ADDR  => self.ppu.oam_mut().write(addr, val),
            IO_BASE_ADDR   ..= IO_END_ADDR   => { self.io_write(addr, val); Ok(()) },
            HRAM_BASE_ADDR ..= HRAM_END_ADDR => self.hram.write(addr, val),
            INTERRUPT_IE_ADDR                => { self.io_write(addr, val); Ok(()) },
            _ => Err(AddressError::Invalid(addr)),
        }
    }
}


#[cfg(test)]
#[path = "../../../tests/bus.rs"]
mod tests;
