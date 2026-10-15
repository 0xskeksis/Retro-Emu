pub mod emulator;
pub use emulator::{Emulator, Model};

pub mod cpu {
    automod::dir!("src/core/cpu");
    pub use register::{Registers, Register, Flag};
    pub use interrupt::Interrupt;
    pub use cpu::Cpu;
}

pub mod bus {
    automod::dir!("src/core/bus");
    pub use bus::Bus;
    pub use memory::{Memory, Ram, Rom, Vram, Oam, Result, AddressError};
    pub use timer::TimerUnit;
    pub use cartridge::Cartridge;
}


pub mod ppu {
    automod::dir!("src/core/ppu");
    pub use ppu::Ppu;
}

