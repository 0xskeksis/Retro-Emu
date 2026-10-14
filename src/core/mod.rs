pub mod emulator;

pub mod cpu {
    automod::dir!("src/core/cpu");
    pub use register::Register;
    pub use register::Flag;
    pub use register::Registers;
    pub use cpu::Cpu;
    pub use stack::Stack;
}

pub mod bus {
    automod::dir!("src/core/bus");
    pub use bus::Bus;
    pub use memory::Memory;
    pub use memory::Ram;
    pub use memory::Rom;
    pub use timer::Timer;
    pub use error::AddressError;
    pub use error::BusResult;
    pub use cartridge::Cartridge;
}


pub mod ppu {
    automod::dir!("src/core/ppu");
    pub use ppu::Ppu;
}

