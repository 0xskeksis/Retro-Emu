use crate::core::{bus::TimerUnit, cpu::Interrupt, ppu::Ppu};

#[derive(Debug, Clone, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Scheduler
{
    elapsed_tcycles: u64,
}

impl Scheduler
{
    pub fn elapsed_tcycles(&self) -> u64 {
        self.elapsed_tcycles
    }

    /// Normal-speed scheduling only.
    pub fn advance_tcycle(&mut self,
        timer: &mut TimerUnit,
        ppu: &mut Ppu,
        interrupts: &mut Interrupt)
    {
        self.elapsed_tcycles = self
            .elapsed_tcycles
            .checked_add(1)
            .expect("scheduler cycle counter overflow");

        if timer.tick_tcycle() {
            interrupts.request_timer();
        }
        ppu.tick_dot();
    }

    pub fn advance_mcycle(&mut self,
        timer: &mut TimerUnit,
        ppu: &mut Ppu,
        interrupts: &mut Interrupt)
    {
        for _ in 0..4 {
            self.advance_tcycle(timer, ppu, interrupts);
        }
    }
}
