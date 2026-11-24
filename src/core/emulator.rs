use std::path::Path;

use crate::core::cart::Cartridge;
use crate::core::cpu::Cpu;
use crate::core::io::Clock;
use crate::core::mem::{BusView, Hram, Ram, Result};
use crate::core::ppu::Ppu;
use crate::core::sys::{IrqCtrl, Scheduler};


#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Emulator
{
    cpu:   Cpu,
    ppu:   Ppu,
    ram:   Ram,
    hram:  Hram,
    timer: Clock,
    cart:  Cartridge,
    sched: Scheduler,
    inter: IrqCtrl,
}

impl Emulator
{
    pub fn new() -> Self
    {
        Self {
            cpu:   Cpu::new(),
            ppu:   Ppu::new(),
            ram:   Ram::new(),
            hram:  Hram::new(),
            timer: Clock::new(),
            cart:  Cartridge::new(),
            sched: Scheduler::new(),
            inter: IrqCtrl::new(),
        }
    }

    pub fn load<P: AsRef<Path>>(path: &P) {
        // TODO:
    }

    pub fn run() {
        // TODO:
    }

    fn step(&mut self) -> Result<()>
    {
        let mut bus = BusView {
            ppu:   &mut self.ppu,
            ram:   &mut self.ram,
            hram:  &mut self.hram,
            cart:  &mut self.cart,
            timer: &mut self.timer,
            sched: &mut self.sched,
            inter: &mut self.inter,
        };
        self.cpu.step(&mut bus)
    }
}
