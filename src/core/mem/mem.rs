use thiserror::Error;

use std::ops::{Deref, DerefMut, Index, IndexMut};

pub const ROM_SIZE:  usize = 0x4000; // 16KB of ROM
pub const RAM_SIZE:  usize = 0x2000; // 8KB of RAM
pub const VRAM_SIZE: usize = 0x2000; // 8KB of VRAM
pub const OAM_SIZE:  usize = 0xA0;   // 160 bytes of OAM
pub const HRAM_SIZE: usize = 0x7F;   // 127 bytes of High RAM (HRAM)
pub const IO_SIZE:   usize = 0x80;   // 128 bytes of I/O registers

pub const ROM_BASE_ADDR:  u16 = 0x0000;
pub const EROM_BASE_ADDR: u16 = 0x4000;
pub const VRAM_BASE_ADDR: u16 = 0x8000;
pub const ERAM_BASE_ADDR: u16 = 0xA000;
pub const RAM_BASE_ADDR:  u16 = 0xC000;
pub const OAM_BASE_ADDR:  u16 = 0xFE00;
pub const IO_BASE_ADDR:   u16 = 0xFF00;
pub const HRAM_BASE_ADDR: u16 = 0xFF80;
pub const INTERRUPT_IE_ADDR: u16 = 0xFFFF;

pub const EROM_END_ADDR: u16 = EROM_BASE_ADDR + (ROM_SIZE as u16) - 1;
pub const ROM_END_ADDR:  u16 = ROM_BASE_ADDR  + (ROM_SIZE as u16) - 1;
pub const RAM_END_ADDR:  u16 = RAM_BASE_ADDR  + (RAM_SIZE as u16) - 1;
pub const ERAM_END_ADDR: u16 = ERAM_BASE_ADDR + (RAM_SIZE as u16) - 1;
pub const VRAM_END_ADDR: u16 = VRAM_BASE_ADDR + (VRAM_SIZE as u16) - 1;
pub const OAM_END_ADDR:  u16 = OAM_BASE_ADDR  + (OAM_SIZE as u16) - 1;
pub const HRAM_END_ADDR: u16 = HRAM_BASE_ADDR + (HRAM_SIZE as u16) - 1;
pub const IO_END_ADDR:   u16 = IO_BASE_ADDR   + (IO_SIZE as u16) - 1;

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
    data:      Vec<u8>,
    base_addr: u16,
}


pub type Ram = Memory<RAM_SIZE, false>;
pub type Rom = Memory<ROM_SIZE, true>;
pub type Vram = Memory<VRAM_SIZE, false>;
pub type Oam = Memory<OAM_SIZE, false>;
pub type Hram = Memory<HRAM_SIZE, false>;

//
// ╔═════════════════════════[ PUBLIC API ]═════════════════════════╗
//

impl<const SIZE: usize, const READY_ONLY: bool> Memory<SIZE, READY_ONLY>
{
    pub fn new(base_addr: u16) -> Self {
        Self { data: vec![0; SIZE], base_addr}
    }

    pub fn base_addr(&self) -> u16 { self.base_addr }

    pub fn read(&self, address: u16) -> Result<u8> {
        self.data.get(self.index_of(address)?)
            .copied()
            .ok_or(AddressError::OutOfBounds(address))
    }

    fn index_of(&self, address: u16) -> Result<usize> {
        address.checked_sub(self.base_addr)
            .map(usize::from)
            .ok_or(AddressError::OutOfBounds(address))
    }
}

impl<const SIZE: usize> Memory<SIZE, false>
{
    pub fn write(&mut self, address: u16, value: u8) -> Result<()>
    {
        let index = self.index_of(address)?;
        self.data.get_mut(index)
            .map(|byte| *byte = value)
            .ok_or(AddressError::OutOfBounds(address))
    }
}

// ------- Index<u16> / IndexMut<u16>
//
// `mem[address]` (read-only)
impl<const SIZE: usize, const READY_ONLY: bool> Index<u16> for Memory<SIZE, READY_ONLY>
{
    type Output = u8;
    fn index(&self, address: u16) -> &Self::Output
    {
        self.data.get(self.index_of(address).unwrap())
            .ok_or(AddressError::OutOfBounds(address))
            .unwrap()
    }
}
// `mem[address]` (read-write)
impl<const SIZE: usize> IndexMut<u16> for Memory<SIZE, false>
{
    fn index_mut(&mut self, address: u16) -> &mut Self::Output
    {
        let index = self.index_of(address).unwrap();
        self.data.get_mut(index)
            .ok_or(AddressError::OutOfBounds(address))
            .unwrap()
    }
}

// ------- Deref / DerefMut
//
// Expose Vec<u8> methods (e.g., `len()`, `is_empty()`, `iter()`, etc.)
impl<const SIZE:usize, const READY_ONLY: bool> Deref for Memory<SIZE, READY_ONLY>
{
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
// Expose Vec<u8> mutable methods (e.g., `push()`, `pop()`, `clear()`, etc.)
impl<const SIZE:usize> DerefMut for Memory<SIZE, false>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}


// ------ IntoIterator: Memory, &Memory, &mut Memory
//
// `for item in Memory` (consumes Memory, yields u8)
impl<const SIZE: usize, const READY_ONLY: bool> IntoIterator for Memory<SIZE, READY_ONLY>
{
    type Item = u8;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}
// `for item in &Memory` (borrows Memory, yields &u8)
impl<'a, const SIZE: usize, const READY_ONLY:bool> IntoIterator for &'a Memory<SIZE, READY_ONLY>
{
    type Item = &'a u8;
    type IntoIter = std::slice::Iter<'a, u8>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}
// `for item in &mut Memory` (mutably borrows Memory, yields &mut u8)
impl<'a, const SIZE: usize> IntoIterator for &'a mut Memory<SIZE, false>
{
    type Item = &'a mut u8;
    type IntoIter = std::slice::IterMut<'a, u8>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter_mut()
    }
}


// ------ From<Vec<u8>>
//
// Construct Memory from a Vec<u8> (e.g., for loading ROM data)
impl<const SIZE: usize, const READY_ONLY: bool> From<Vec<u8>> for Memory<SIZE, READY_ONLY>
{
    fn from(data: Vec<u8>) -> Self {
        Self { data, base_addr: 0 }
    }
}
