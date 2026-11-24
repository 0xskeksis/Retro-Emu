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
    pub use mem::{Memory, Ram, Rom, Vram, Oam, Result, AddressError};
    pub use bus::{Bus, BusView};
}
pub mod sys
{
    automod::dir!("src/core/sys");
    pub use irq::Interrupts;
    pub use sched::Scheduler;
}
pub mod ppu {
    automod::dir!("src/core/ppu");
    pub use ppu::Ppu;
}

