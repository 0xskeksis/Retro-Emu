use std::path::Path;

use crate::core::bus::Bus;
use crate::core::cpu::Cpu;



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
    pub cpu: Cpu,
    pub bus: Bus,
}



impl Emulator
{
    pub fn new() -> Self
    {
        Self {
            cpu: Cpu::new(),
            bus: Bus::new(Model::default()),
        }
    }

    pub fn load<P: AsRef<Path>>(path: &P)
    {
        // TODO:
    }

    fn run()
    {
        // TODO:
    }
}
