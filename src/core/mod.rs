pub mod emulator;
pub use emulator::{Emulator, Model};

pub mod cpu {
    automod::dir!("src/core/cpu");
    pub use register::{Register, Flag};
    pub use interrupt::{Interrupts, InterruptKind};
    pub use cpu::Cpu;
}

pub mod bus {
    automod::dir!("src/core/bus");
    pub use bus::{Bus, BusView};
    pub use memory::{Memory, Ram, Rom, Vram, Oam, Result, AddressError};
    pub use timer::Timer;
    pub use scheduler::Scheduler;
    pub use cartridge::Cartridge;
}


pub mod ppu {
    automod::dir!("src/core/ppu");
    pub use ppu::Ppu;
}

