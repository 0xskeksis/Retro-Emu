
pub const VRAM_SIZE: usize = 0x2000; // 8KB of VRAM
pub const OAM_SIZE: usize = 0xA0;    // 160 bytes of OAM (Object Attribute Memory)

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vram(Vec<u8>);
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Oam(Vec<u8>);

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Ppu
{
    pub vram: Vram,         // 8KB of VRAM
    pub oam: Oam,           // 160 bytes of OAM (Object Attribute Memory)
    pub lcdc: u8,           // LCD Control Register
    pub stat: u8,           // LCD Status Register
    pub scy: u8,            // Scroll Y
    pub scx: u8,            // Scroll X
    pub ly: u8,             // LY - Current Scanline
    pub lyc: u8,            // LYC - LY Compare
    pub bgp: u8,            // Background Palette Data
    pub obp0: u8,           // Object Palette 0 Data
    pub obp1: u8,           // Object Palette 1 Data
    pub wy: u8,             // Window Y Position
    pub wx: u8,             // Window X Position minus 7
}

impl Default for Vram {
    fn default() -> Self { Self(vec![0; VRAM_SIZE]) }
}
impl Default for Oam {
    fn default() -> Self { Self(vec![0; OAM_SIZE]) }
}

impl Ppu
{
    pub fn new() -> Self {
        Default::default()
    }
}
