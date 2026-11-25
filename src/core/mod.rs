pub mod emulator;
pub use emulator::Emulator;

pub mod cpu
{
    automod::dir!("src/core/cpu");
    pub use reg::{Register, Flag};
    pub use cpu::Cpu;
}
pub mod cart
{
    automod::dir!("src/core/cart");
    pub use cart::{Cartridge, Model};
}

pub mod io
{
    automod::dir!("src/core/io");
    pub use clock::Clock;
}
pub mod mem
{
    automod::dir!("src/core/mem");
    pub use mem::{Memory, Result, AddressError,
        Ram,  RAM_BASE_ADDR,  RAM_END_ADDR,
        Rom,  ROM_BASE_ADDR,  ROM_END_ADDR,
        Vram, VRAM_BASE_ADDR, VRAM_END_ADDR,
        Oam,  OAM_BASE_ADDR,  OAM_END_ADDR,
        Hram, HRAM_BASE_ADDR, HRAM_END_ADDR,
        ERAM_BASE_ADDR, ERAM_END_ADDR,
        EROM_BASE_ADDR, EROM_END_ADDR,
        IO_BASE_ADDR, IO_END_ADDR,
        INTERRUPT_IE_ADDR};
    pub use bus::{Bus, BusView};
}
pub mod sys
{
    automod::dir!("src/core/sys");
    pub use irq::{Irq, IrqSet, IrqCtrl};
    pub use sched::Scheduler;
}
pub mod ppu {
    automod::dir!("src/core/ppu");
    pub use ppu::Ppu;
}

