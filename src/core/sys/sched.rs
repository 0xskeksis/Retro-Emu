#[derive(Debug, Clone, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Scheduler
{
    elapsed_tcycles: u64,
}

impl Scheduler
{
    pub fn new() -> Self { Default::default() }

    pub fn elapsed_tcycles(&self) -> u64 {
        self.elapsed_tcycles
    }

    /// Normal-speed scheduling only.
    pub fn advance_tcycle(&mut self)
    {
        self.elapsed_tcycles = self
            .elapsed_tcycles
            .wrapping_add(1);
    }

    pub fn advance_mcycle(&mut self)
    {
        self.elapsed_tcycles = self
            .elapsed_tcycles
            .wrapping_add(4);
    }
}
