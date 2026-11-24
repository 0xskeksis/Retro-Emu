use thiserror::Error;

const RAM_SIZE: usize = 0x10000; // 64KB of RAM
const ROM_SIZE: usize = 0x8000;  // 32KB of ROM
const VRAM_SIZE: usize = 0x2000; // 8KB of VRAM
const OAM_SIZE: usize = 0xA0;    // 160 bytes of OAM (Object Attribute Memory)

//
// ╔═════════════════════════[ DEFINITION ]═════════════════════════╗
//

// ----- Errors -----
#[derive(Error, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum AddressError
{
    #[error("Address {0:#X} is out of bounds")]
    OutOfBounds(u16),
    #[error("Address {0:#X} is invalid")]
    Invalid(u16),
    #[error("Address {0:#X} is read-only")]
    ReadOnly(u16),
    #[error("An unknown error occurred at address {0:#X}")]
    Other(u16),
}
pub type Result<T> = std::result::Result<T, AddressError>;



#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Memory<const SIZE:usize, const READY_ONLY: bool = false>
{
    data: Vec<u8>,
}


pub type Ram = Memory<RAM_SIZE, false>;
pub type Rom = Memory<ROM_SIZE, true>;
pub type Vram = Memory<VRAM_SIZE, false>;
pub type Oam = Memory<OAM_SIZE, false>;

//
// ╔═════════════════════════[ PUBLIC API ]═════════════════════════╗
//

impl<const SIZE: usize, const READY_ONLY: bool> Memory<SIZE, READY_ONLY>
{
    pub fn new() -> Self {
        Self { data: vec![0; SIZE] }
    }

    pub fn read(&self, address: u16) -> Result<u8>
    {
        self.data.get(usize::from(address)).copied()
            .ok_or(AddressError::OutOfBounds(address))
    }
}

impl<const SIZE: usize> Memory<SIZE, false>
{
    pub fn write(&mut self, address: u16, value: u8) -> Result<()>
    {
        self.data.get_mut(usize::from(address))
            .map(|byte| *byte = value)
            .ok_or(AddressError::OutOfBounds(address))
    }
}

impl<const SIZE: usize, const READY_ONLY: bool> From<Vec<u8>> for Memory<SIZE, READY_ONLY>
{
    fn from(data: Vec<u8>) -> Self {
        Self { data }
    }
}


impl<const SIZE: usize> std::ops::IndexMut<u16> for Memory<SIZE, false>
{
    fn index_mut(&mut self, address: u16) -> &mut Self::Output
    {
        self.data.get_mut(usize::from(address))
            .ok_or(AddressError::OutOfBounds(address))
            .unwrap()
    }
} 

impl<const SIZE: usize, const READY_ONLY: bool> std::ops::Index<u16> for Memory<SIZE, READY_ONLY>
{
    type Output = u8;
    fn index(&self, address: u16) -> &Self::Output
    {
        self.data.get(usize::from(address))
            .ok_or(AddressError::OutOfBounds(address))
            .unwrap()
    }
}


