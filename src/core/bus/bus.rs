use crate::core::Model;
use crate::core::bus::scheduler::Scheduler;
use crate::core::bus::{Cartridge, Ram, TimerUnit};
use crate::core::cpu::Interrupt;
use crate::core::ppu::Ppu;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Bus
{
    ram:        Ram,
    ppu:        Ppu,
    cart:       Cartridge,
    timer:      TimerUnit,
    scheduler:  Scheduler,
    interrupts: Interrupt,
}

impl Bus {
    /// Normal-speed operation; no boot-ROM state is applied.
    pub fn new(model: Model) -> Self
    {
        Self::with_timer(TimerUnit::new(model))
    }

    pub fn with_timer(timer: TimerUnit) -> Self {
        Self {
            timer,
            cart: Cartridge::new(),
            ram: Ram::new(),
            ppu: Ppu::new(),
            scheduler: Scheduler::default(),
            interrupts: Interrupt::default(),
        }
    }

    pub fn elapsed_tcycles(&self) -> u64 {
        self.scheduler.elapsed_tcycles()
    }

    pub fn elapsed_ppu_dots(&self) -> u64 {
        self.ppu.elapsed_dots()
    }

    /// A CPU read, including its time cost.
    pub fn read8(&mut self, address: u16) -> u8 {
        self.advance_mcycle();
        self.mapped_read(address)
    }

    /// A CPU write, including its time cost.
    pub fn write8(&mut self, address: u16, value: u8) {
        self.advance_mcycle();
        self.mapped_write(address, value);
    }

    /// CPU internal cycle: time passes without a memory access.
    ///
    /// Also usable for an appropriate HALT execution path.
    /// STOP requires separate handling.
    pub fn idle_mcycle(&mut self) {
        self.advance_mcycle();
    }

    fn advance_mcycle(&mut self) {
        self.scheduler.advance_mcycle(
            &mut self.timer,
            &mut self.ppu,
            &mut self.interrupts,
        );
    }

    /// Address routing only: does not advance time.
    fn mapped_read(&self, address: u16) -> u8 {
        match address {
            // TODO: MAP Cartridge, RAM, PPU registers...
            // 0xFF04..=0xFF07 => self.timer.read(TimerRegister::try_from(address))
            // 0xFF0F => self.interrupts.read_if(),
            _ => 0xFF,
        }
    }

    /// Address routing only: does not advance time.
    fn mapped_write(&mut self, address: u16, value: u8) {
        match address {
            // TODO: MAP Cartridge, RAM, PPU registers...
            // 0xFF04..=0xFF07 => self.timer.write(TimerRegister::try_from(address), value),
            // 0xFF0F => self.interrupts.write_if(value),
            // Scaffold only. Add cartridge, RAM, PPU registers, etc.
            _ => {}
        }
    }
}


#[cfg(test)]
#[path = "../../../tests/bus.rs"]
mod tests;
