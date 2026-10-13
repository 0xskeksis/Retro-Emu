/*
 * All of the table needed for the cartrige parsing header
 * Sources: https://gbdev.io/pandocs/The_Cartridge_Header.html
 */

/*
 * Notes:
 * Every end offset is +1 to get the good size
 */


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
    cartridge_type:     CartridgeType,
    rom_size:           usize,
    ram_size:           usize,
    destination_code:   u8,
    version:            u8,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Cartridge
{
    rom:     Vec<u8>,
    header:  Option<CartridgeHeader>,
}

const ENTRY_POINT_OFFSET:  u16 = 0x100;
const ENTRY_POINT_SIZE: usize = (0x103 - 0x100) as usize;

const LOGO_OFFSET:  u16 = 0x104;
const LOGO_END:     u16 = 0x134;
const LOGO_SIZE:    usize = (LOGO_END - LOGO_OFFSET) as usize;

const LOGO: [u16; LOGO_SIZE] = [
0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D,
0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99,
0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E
];

const TITLE_OFFSET: u16 = 0x134;
// TODO: : Find the version of the cartridge to deduce the title size
// const TITLE_END:    u16 = 0x143;
// const TITLE_SIZE:   usize = (TITLE_SIZE - TITLE_OFFSET);

const MANUFACT_CODE_OFFSET:     u16 = 0x13F;
const MANUFACT_CODE_END:        u16 = 0x143;
const MANUFACT_CODE_SIZE:    usize = (MANUFACT_CODE_END - MANUFACT_CODE_OFFSET) as usize;

const CGB_FLAG_OFFSET:          u16 = 0x143;
const DMG_ONLY_VALUE:           u16 = 0x00; // DMG only
const DMG_CGB_VALUE:            u16 = 0x80;  // CGB + DMG
const CGB_ONLY_VALUE:           u16 = 0xC0;  // CGB only

const LICENSE_CODE_OFFSET:      u16 = 0x144;
const ROM_SIZE_OFFSET:          u16 = 0x148;
const RAM_SIZE_OFFSET:          u16 = 0x149;
const DESTRINATION_CODE_OFFSET: u16 = 0x14A;

const GAME_START:               u16 = 0x150;


#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum CartridgeType {
    Romonly                     = 0x00,
    Mbc1                        = 0x01,
    Mbc1Ram                     = 0x02,
    Mbc1RamBattery              = 0x03,
    Mbc2                        = 0x05,
    Mbc2Battery                 = 0x06,
    RomRam                      = 0x08,
    RomRamBattery               = 0x09,
    Mmm01                       = 0x0B,
    Mmm01Ram                    = 0x0C,
    Mmm01RamBattery             = 0x0D,
    Mbc3TimerBattery            = 0x0F,
    Mbc3TimerRamBattery         = 0x10,
    Mbc3                        = 0x11,
    Mbc3Ram                     = 0x12,
    Mbc3RamBattery              = 0x13,
    Mbc5                        = 0x19,
    Mbc5Ram                     = 0x1A,
    Mbc5RamBattery              = 0x1B,
    Mbc5Rumble                  = 0x1C,
    Mbc5RumbleRam               = 0x1D,
    Mbc5RumbleRamBattery        = 0x1E,
    Mbc6                        = 0x20,
    Mbc7SensorRumbleRamBattery  = 0x22,
    PocketCamera                = 0xFC,
    BandaiTama5                 = 0xFD,
    Huc3                        = 0xFE,
    Huc1RamBattery              = 0xFF,
}

impl Cartridge
{
    pub fn new() -> Self { Default::default() }
}
