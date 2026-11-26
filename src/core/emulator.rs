use std::path::Path;
use std::time::{Duration, Instant};

use crate::core::cart::Cartridge;
use crate::core::cpu::Cpu;
use crate::core::io::Clock;
use crate::core::ppu::Ppu;
use crate::core::sys::{IrqCtrl, Scheduler};

use crate::core::mem::{BusView,
    HRAM_BASE_ADDR, Hram, RAM_BASE_ADDR, Ram, Result,
};

const FRAME_TCYCLES: u64 = 70_224;
const FRAME_TIME: Duration = Duration::from_nanos(16_742_706); // 70224 / 4194304 s


#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Emulator
{
    cpu:   Cpu,
    ppu:   Ppu,
    ram:   Ram,
    hram:  Hram,
    clock: Clock,
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
            ram:   Ram::new(RAM_BASE_ADDR),
            hram:  Hram::new(HRAM_BASE_ADDR),
            clock: Clock::new(),
            cart:  Cartridge::new(),
            sched: Scheduler::new(),
            inter: IrqCtrl::new(),
        }
    }

    pub fn load<P: AsRef<Path>>(path: &P) {
        // TODO:
    }

    pub fn run(&mut self) -> Result<()>
    {
        let mut deadline = Instant::now();
        loop {
            (0..FRAME_TCYCLES).try_for_each(|_| self.step())?;
            deadline = self.resync(deadline);
        }
    }

    pub fn step(&mut self) -> Result<()>
    {
        self.cpu.step(&mut BusView {
            ppu:   &mut self.ppu,
            ram:   &mut self.ram,
            hram:  &mut self.hram,
            cart:  &mut self.cart,
            clock: &mut self.clock,
            sched: &mut self.sched,
            inter: &mut self.inter,
        })
    }

    fn resync(&self, mut deadline:  Instant) -> Instant
    {
        deadline += FRAME_TIME;
        match deadline.checked_duration_since(Instant::now())
        {
            Some(wait) => {std::thread::sleep(wait); deadline},
            None => Instant::now(),
        }
    }
}
