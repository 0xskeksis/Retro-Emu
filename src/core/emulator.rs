use std::path::Path;

use crate::core::bus::{BusView, Cartridge, Ram, Scheduler, Timer, Result};
use crate::core::cpu::{Cpu, Interrupts};
use crate::core::ppu::Ppu;



#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub enum Model
{
    #[default]
    Dmg,
    Cgb,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Emulator
{
    cpu:   Cpu,
    ppu:   Ppu,
    ram:   Ram,
    timer: Timer,
    cart:  Cartridge,
    sched: Scheduler,
    inter: Interrupts,
}

impl Emulator
{
    pub fn new() -> Self
    {
        Self {
            cpu:   Cpu::new(),
            ppu:   Ppu::new(),
            ram:   Ram::new(),
            timer: Timer::new(),
            cart:  Cartridge::new(),
            sched: Scheduler::new(),
            inter: Interrupts::new(),
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
            ppu: &mut self.ppu,
            ram: &mut self.ram,
            cart: &mut self.cart,
            timer: &mut self.timer,
            sched: &mut self.sched,
            inter: &mut self.inter,
        };
        self.cpu.step(&mut bus)
    }
}
