use crate::core::io::{Irq, IrqSet};
use crate::core::mem::{
    OAM_BASE_ADDR, Oam,
    VRAM_BASE_ADDR, Vram,
};

#[derive(Debug, Clone, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Ppu
{
    vram: Vram,
    oam:  Oam,
    elapsed_dots: u64,
}

impl Ppu
{
    pub fn new() -> Self {
        Self {
            vram: Vram::new(VRAM_BASE_ADDR),
            oam:  Oam::new(OAM_BASE_ADDR),
            ..Default::default()
        }
    }

    pub fn vram(&self) -> &Vram { &self.vram }
    pub fn vram_mut(&mut self) -> &mut Vram { &mut self.vram }

    pub fn oam(&self) -> &Oam { &self.oam }
    pub fn oam_mut(&mut self) -> &mut Oam { &mut self.oam }

    pub fn elapsed_dots(&self) -> u64 { self.elapsed_dots }

    pub(crate) fn tick(&mut self) -> IrqSet
    {
        self.elapsed_dots = self
            .elapsed_dots
            .wrapping_add(1);

        IrqSet::EMPTY
    }
}
