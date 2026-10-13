use crate::core::{cpu::Registers};

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Cpu
{
    pub regs: Registers,
}

impl Cpu
{
    pub fn new() -> Self {
        Self { regs: Registers::new() }
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
    pub fn step(&mut self){}

}
