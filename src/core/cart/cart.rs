use thiserror::Error;

use crate::core::mem::{AddressError, Ram, Rom, ROM_BASE_ADDR, self};

#[derive(Debug, Error)]
pub enum LoadError {
    #[error("invalid ROM: expected at least {expected} bytes, got {actual}")]
    TooShort { expected: usize, actual: usize },
}

#[derive(Debug, Error)]
pub enum CartridgeError {
    #[error(transparent)]
    Address(#[from] AddressError),

    #[error(transparent)]
    Load(#[from] LoadError),
}

pub type Result<T> = std::result::Result<T, CartridgeError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub enum Model
{
    #[default]
    Dmg,
    Cgb,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct CartridgeHeader
{
    entry_point:        [u8; 4],
    nintendo_logo:      [u8; 0x30],
    title:              &'static str,
    manufacturer_code:  [u8; 4],
    cgb_flags:          u8,
    license_code:       [u8; 2],
    sgb_flags:          u8,
    cartridge_type:     Model,
    rom_size:           usize,
    ram_size:           usize,
    destination_code:   u8,
    version:            u8,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Cartridge
{
    rom:     Rom,
    ext_rom: Option<Rom>,
    ext_ram: Option<Ram>,
    header:  Option<CartridgeHeader>,
}

const ENTRY_POINT_OFFSET: usize = 0x100;
const ENTRY_POINT_SIZE:   usize = 0x104 - ENTRY_POINT_OFFSET; // 4 bytes

const LOGO_OFFSET: usize = 0x104;
const LOGO_END:    usize = 0x134;
const LOGO_SIZE:   usize = LOGO_END - LOGO_OFFSET; // 0x30

const LOGO: [u8; LOGO_SIZE] = [
    0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D,
    0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99,
    0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
];

const TITLE_OFFSET: usize = 0x134;

const MANUFACT_CODE_OFFSET: usize = 0x13F;
const MANUFACT_CODE_END:    usize = 0x143;
const MANUFACT_CODE_SIZE:   usize = MANUFACT_CODE_END - MANUFACT_CODE_OFFSET;

const CGB_FLAG_OFFSET: usize = 0x143;
const DMG_ONLY_VALUE:  u8 = 0x00;
const DMG_CGB_VALUE:   u8 = 0x80;
const CGB_ONLY_VALUE:  u8 = 0xC0;

const LICENSE_CODE_OFFSET:      usize = 0x144;
const ROM_SIZE_OFFSET:          usize = 0x148;
const RAM_SIZE_OFFSET:          usize = 0x149;
const DESTINATION_CODE_OFFSET:  usize = 0x14A;

const GAME_START: usize = 0x150;

impl Cartridge {
    pub fn new() -> Self {
        Self {
            rom: Rom::new(ROM_BASE_ADDR),
            ext_rom: None,
            ext_ram: None,
            header: None,
        }
    }

    pub fn load(&mut self, data: Vec<u8>) -> Result<()> {
        if data.len() < GAME_START {
            return Err(LoadError::TooShort {
                expected: GAME_START,
                actual: data.len(),
            }
            .into());
        }
        //TODO: fill th header
        self.rom = Rom::from(data);
        Ok(())
    }

    pub fn rom(&self) -> &Rom { &self.rom }

    pub fn read_ext_rom(&self, addr: u16) -> mem::Result<u8> {
        let rom = self.ext_rom.as_ref().ok_or(AddressError::OutOfBounds(addr))?;
        rom.read(addr)
    }

    pub fn read_ext_ram(&self, addr: u16) -> mem::Result<u8> {
        let ram = self.ext_ram.as_ref().ok_or(AddressError::OutOfBounds(addr))?;
        ram.read(addr)
    }

    pub fn write_ext_ram(&mut self, addr: u16, value: u8) -> mem::Result<()> {
        let ram = self.ext_ram.as_mut().ok_or(AddressError::OutOfBounds(addr))?;
        ram.write(addr, value)
    }
}
