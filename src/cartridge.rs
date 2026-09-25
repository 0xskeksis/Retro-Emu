/*
 * All of the table needed for the cartrige parsing header
 * Sources: https://gbdev.io/pandocs/The_Cartridge_Header.html
 */

/*
 * Notes:
 * Every end offset is +1 to get the good size
 */

struct CartridgeHeader {
    entry_point:        [u8; 4],
    nintendo_logo:      [u8; 0x30],
    title:              &str,
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

pub struct Cartridge {
   rom:     Vec<u8>,
   header:  CartridgeHeader,
}

const ENTRY_POINT_OFFSET:   u8 = 0x100;
const ENTRY_POINT_SIZE:     u8 = (0x103 - 0x100);

const LOGO_OFFSET:  u8 = 0x104;
const LOGO_END:     u8 = 0x134;
const LOGO_SIZE:    usize = (LOGO_END - LOGO_OFFSET);

const logo: [u8; LOGO_SIZE] = [
0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D,
0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99,
0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E
];

/* TODO: 
 * Find the version of the cartridge to deduce the title size
 */

const TITLE_OFFSET: u8 = 0x134;

// TODO:
// const TITLE_END:    u8 = 0x143;
// const TITLE_SIZE:   usize = (TITLE_SIZE - TITLE_OFFSET);

const MANUFACT_CODE_OFFSET: u8 = 0x13F;
const MANUFACT_CODE_END:    u8 = 0x143;
const MANUFACT_CODE_SIZE:   usize = (MANUFACT_CODE_END - MANUFACT_CODE_OFFSET);

const CGB_FLAG_OFFSET:      u8 = 0x143;
const DMG_ONLY_VALUE:       u8 = 0x00;  // DMG only
const DMG_CGB__VALUE:       u8 = 0x80;  // CGB + DMG
const CGB_ONLY_VALUE:       u8 = 0xC0;  // CGB only


const LICENSE_CODE_OFFSET:  u8 = 0x144;

const license_code_table: [(&str, &str)] = [
    ("00", "None"),
    ("01", "Nintendo Research & Development 1"),
    ("08", "Capcom"),
    ("13", "EA (Electronic Arts)"),
    ("18", "Hudson Soft"),
    ("19", "B-AI"),
    ("20", "KSS"),
    ("22", "Planning Office WADA"),
    ("24", "PCM Complete"),
    ("25", "San-X"),
    ("28", "Kemco"),
    ("29", "SETA Corporation"),
    ("30", "Viacom"),
    ("31", "Nintendo"),
    ("32", "Bandai"),
    ("33", "Ocean Software/Acclaim Entertainment"),
    ("34", "Konami"),
    ("35", "HectorSoft"),
    ("37", "Taito"),
    ("38", "Hudson Soft"),
    ("39", "Banpresto"),
    ("41", "Ubi bite"),
    ("42", "Atlus"),
    ("44", "Malibu Interactive"),
    ("46", "Angel"),
    ("47", "Bullet-Proof Software2"),
    ("49", "Irem"),
    ("50", "Absolute"),
    ("51", "Acclaim Entertainment"),
    ("52", "Activision"),
    ("53", "Sammy USA Corporation"),
    ("54", "Konami"),
    ("55", "Hi Tech Expressions"),
    ("56", "LJN"),
    ("57", "Matchbox"),
    ("58", "Mattel"),
    ("59", "Milton Bradley Company"),
    ("60", "Titus Interactive"),
    ("61", "Virgin Games Ltd.3"),
    ("64", "Lucasfilm Games4"),
    ("67", "Ocean Software"),
    ("69", "EA (Electronic Arts)"),
    ("70", "Infogrames5"),
    ("71", "Interplay Entertainment"),
    ("72", "Broderbund"),
    ("73", "Sculptured Software6"),
    ("75", "The Sales Curve Limited7"),
    ("78", "THQ"),
    ("79", "Accolade8"),
    ("80", "Misawa Entertainment"),
    ("83", "LOZC G."),
    ("86", "Tokuma Shoten"),
    ("87", "Tsukuda Original"),
    ("91", "Chunsoft"),
    ("92", "Video System"),
    ("93", "Ocean Software/Acclaim Entertainment"),
    ("95", "Varie"),
    ("96", "Yonezawa10/S’Pal"),
    ("97", "Kaneko"),
    ("99", "Pack-In-Video"),
    ("9H", "Bottom up"),
    ("A4", "Konami (Yu-Gi-Oh!)"),
    ("BL", "MTO"),
    ("DK", "Kodansha"),
];

// s/\v(\S+)\s+(.+)/("\1", "\2")/  
// Merci infiniment

const old_license_code_table: [(&str, &str)] = [
    ("00", "None"),
    ("01", "Nintendo"),
    ("08", "Capcom"),
    ("09", "HOT-B"),
    ("0A", "Jaleco"),
    ("0B", "Coconuts Japan"),
    ("0C", "Elite Systems"),
    ("13", "EA (Electronic Arts)"),
    ("18", "Hudson Soft"),
    ("19", "ITC Entertainment"),
    ("1A", "Yanoman"),
    ("1D", "Japan Clary"),
    ("1F", "Virgin Games Ltd.3"),
    ("24", "PCM Complete"),
    ("25", "San-X"),
    ("28", "Kemco"),
    ("29", "SETA Corporation"),
    ("30", "Infogrames5"),
    ("31", "Nintendo"),
    ("32", "Bandai"),
    ("33", "Indicates that the New licensee code should be used instead."),
    ("34", "Konami"),
    ("35", "HectorSoft"),
    ("38", "Capcom"),
    ("39", "Banpresto"),
    ("3C", "Entertainment Interactive (stub)"),
    ("3E", "Gremlin"),
    ("41", "Ubi Soft1"),
    ("42", "Atlus"),
    ("44", "Malibu Interactive"),
    ("46", "Angel"),
    ("47", "Spectrum HoloByte"),
    ("49", "Irem"),
    ("4A", "Virgin Games Ltd.3"),
    ("4D", "Malibu Interactive"),
    ("4F", "U.S. Gold"),
    ("50", "Absolute"),
    ("51", "Acclaim Entertainment"),
    ("52", "Activision"),
    ("53", "Sammy USA Corporation"),
    ("54", "GameTek"),
    ("55", "Park Place15"),
    ("56", "LJN"),
    ("57", "Matchbox"),
    ("59", "Milton Bradley Company"),
    ("5A", "Mindscape"),
    ("5B", "Romstar"),
    ("5C", "Naxat Soft16"),
    ("5D", "Tradewest"),
    ("60", "Titus Interactive"),
    ("61", "Virgin Games Ltd.3"),
    ("67", "Ocean Software"),
    ("69", "EA (Electronic Arts)"),
    ("6E", "Elite Systems"),
    ("6F", "Electro Brain"),
    ("70", "Infogrames5"),
    ("71", "Interplay Entertainment"),
    ("72", "Broderbund"),
    ("73", "Sculptured Software6"),
    ("75", "The Sales Curve Limited7"),
    ("78", "THQ"),
    ("79", "Accolade8"),
    ("7A", "Triffix Entertainment"),
    ("7C", "MicroProse"),
    ("7F", "Kemco"),
    ("80", "Misawa Entertainment"),
    ("83", "LOZC G."),
    ("86", "Tokuma Shoten"),
    ("8B", "Bullet-Proof Software2"),
    ("8C", "Vic Tokai Corp.17"),
    ("8E", "Ape Inc.18"),
    ("8F", "I’Max19"),
    ("91", "Chunsoft Co.9"),
    ("92", "Video System"),
    ("93", "Tsubaraya Productions"),
    ("95", "Varie"),
    ("96", "Yonezawa10/S’Pal"),
    ("97", "Kemco"),
    ("99", "Arc"),
    ("9A", "Nihon Bussan"),
    ("9B", "Tecmo"),
    ("9C", "Imagineer"),
    ("9D", "Banpresto"),
    ("9F", "Nova"),
    ("A1", "Hori Electric"),
    ("A2", "Bandai"),
    ("A4", "Konami"),
    ("A6", "Kawada"),
    ("A7", "Takara"),
    ("A9", "Technos Japan"),
    ("AA", "Broderbund"),
    ("AC", "Toei Animation"),
    ("AD", "Toho"),
    ("AF", "Namco"),
    ("B0", "Acclaim Entertainment"),
    ("B1", "ASCII Corporation or Nexsoft"),
    ("B2", "Bandai"),
    ("B4", "Square Enix"),
    ("B6", "HAL Laboratory"),
    ("B7", "SNK"),
    ("B9", "Pony Canyon"),
    ("BA", "Culture Brain"),
    ("BB", "Sunsoft"),
    ("BD", "Sony Imagesoft"),
    ("BF", "Sammy Corporation"),
    ("C0", "Taito"),
    ("C2", "Kemco"),
    ("C3", "Square"),
    ("C4", "Tokuma Shoten"),
    ("C5", "Data East"),
    ("C6", "Tonkin House"),
    ("C8", "Koei"),
    ("C9", "UFL"),
    ("CA", "Ultra Games"),
    ("CB", "VAP, Inc."),
    ("CC", "Use Corporation"),
    ("CD", "Meldac"),
    ("CE", "Pony Canyon"),
    ("CF", "Angel"),
    ("D0", "Taito"),
    ("D1", "SOFEL (Software Engineering Lab)"),
    ("D2", "Quest"),
    ("D3", "Sigma Enterprises"),
    ("D4", "ASK Kodansha Co."),
    ("D6", "Naxat Soft16"),
    ("D7", "Copya System"),
    ("D9", "Banpresto"),
    ("DA", "Tomy"),
    ("DB", "LJN"),
    ("DD", "Nippon Computer Systems"),
    ("DE", "Human Ent."),
    ("DF", "Altron"),
    ("E0", "Jaleco"),
    ("E1", "Towa Chiki"),
    ("E2", "Yutaka # Needs more info"),
    ("E3", "Varie"),
    ("E5", "Epoch"),
    ("E7", "Athena"),
    ("E8", "Asmik Ace Entertainment"),
    ("E9", "Natsume"),
    ("EA", "King Records"),
    ("EB", "Atlus"),
    ("EC", "Epic/Sony Records"),
    ("EE", "IGS"),
    ("F0", "A Wave"),
    ("F3", "Extreme Entertainment"),
    ("FF", "LJN"),
];

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

// This byte indicates how much ROM is present on the cartridge. In most cases, the ROM size is given by:
// 32 KiB × (1 << <value>):

const ROM_SIZE_OFFSET: u8           = 0x148;

const RAM_SIZE_OFFSET: u8           = 0x149;

const DESTRINATION_CODE_OFFSET: u8  = 0x14A;

const GAME_START: u8                = 0x150;
