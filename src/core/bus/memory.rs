use crate::core::bus::{AddressError, BusResult};


#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Memory<const READY_ONLY: bool = false>
{
    data: Vec<u8>,
}

pub type Ram = Memory<false>;
pub type Rom = Memory<true>;

impl<const READY_ONLY: bool> Memory<READY_ONLY>
{
    pub fn new(size: usize) -> Self {
        Self { data: vec![0; size] }
    }

    pub fn from_bytes(data: Vec<u8>) -> Self {
        Self { data }
    }

    pub fn read(&self, address: u16) -> BusResult<u8> 
    {
        self.data.get(usize::from(address)).copied()
            .ok_or(AddressError::OutOfBounds(address))
    }
}

impl Memory<false>
{
    pub fn write(&mut self, address: u16, value: u8) -> BusResult<()> 
    {
        self.data.get_mut(usize::from(address))
            .map(|byte| *byte = value)
            .ok_or(AddressError::OutOfBounds(address))
    }
}
