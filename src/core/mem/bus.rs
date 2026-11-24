use crate::core::mem::{AddressError, Hram, Ram, Result};
use crate::core::sys::Scheduler;
use crate::core::ppu::Ppu;
use crate::core::cart::Cartridge;
use crate::core::sys::Interrupts;
use crate::core::io::Clock;

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct BusView<'a>
{
        pub ppu:   &'a mut Ppu,
        pub ram:   &'a mut Ram,
        pub hram:  &'a mut Hram,
        pub cart:  &'a mut Cartridge,
        pub sched: &'a mut Scheduler,
        pub inter: &'a mut Interrupts,
        pub timer: &'a mut Clock,
}

pub trait Bus
{
    /// Memory read: costs 1 M-cycle (4 T-cycles)
    fn read(&mut self, addr: u16) -> Result<u8>;
    /// Memory write: costs 1 M-cycle (4 T-cycles)
    fn write(&mut self, addr: u16, val: u8) -> Result<()>;
    /// Internal M-cycle with no memory access (e.g. the extra cycle in JP nn)
    fn advance_mcycle(&mut self);
    /// Internal T-cycle with no memory access (e.g. the extra cycle in JP nn)
    fn advance_tcycle(&mut self);
}


// MAN - Memory Map
// https://gbdev.io/pandocs/Memory_Map.html

impl Bus for BusView<'_>
{
    fn read(&mut self, addr: u16) -> Result<u8> {
        self.advance_mcycle();
        self.mapped_read(addr)
    }

    fn write(&mut self, addr: u16, val: u8) -> Result<()> {
        self.advance_mcycle();
        self.mapped_write(addr, val)
    }

    fn advance_mcycle(&mut self) {
        self.advance(4);
    }

    fn advance_tcycle(&mut self) {
        self.advance(1);
    }

}

impl BusView<'_>
{
    fn advance(&mut self, tcycles: u32) {
        for _ in 0..tcycles {
            self.sched.advance_tcycle();
            // TODO: self.timer.tick_tcycle(tcycles, self.inter);
            // TODO: self.ppu.tick_tcycle(tcycles, self.inter);
        }
    }

    /// Address routing only: does not advance time.
    fn mapped_read(&self, addr: u16) ->  Result<u8> {
        // TODO: : I/O, Interrupts
        match addr {
            0x0000..=0x3FFF => { self.cart.rom().read(addr)     }, // 16 KiB ROM, Cartridge fixed bank
            0x4000..=0x7FFF => { self.cart.read_ext_rom(addr)   }, // 16 KiB ROM, Cartridge switchable bank (if any)
            0x8000..=0x9FFF => { self.ppu.vram().read(addr)     }, // 8 KiB VRAM, In CGB, switchable bank
            0xA000..=0xBFFF => { self.cart.read_ext_ram(addr)   }, // 8 KiB RAM, In CGB, Cartridge extern switchable bank (if any)
            0xC000..=0xCFFF => { self.ram.read(addr)            }, // 4 KiB WRAM
            0xD000..=0xDFFF => { self.ram.read(addr)            }, // 4 KiB WRAM, In CGB, switchable bank
            0xFE00..=0xFE9F => { self.ppu.oam().read(addr)      }, // 160B Sprite attribute table (OAM)
            0xFF00..=0xFF7F => { Err(AddressError::Other(addr)) }, // I/O Ports
            0xFF80..=0xFFFE => { self.hram.read(addr)           }, // High RAM (HRAM)
            0xFFFF          => { Err(AddressError::Other(addr)) }, // Interrupt Enable Register
            _ => Err(AddressError::Invalid(addr)),                 // 0xFEA0..=0xFEFF & E000..=FDFF are unusable
        }
    }

    /// Address routing only: does not advance time.
    fn mapped_write(&mut self, addr: u16, val: u8) -> Result<()> {
        // TODO: : I/O, Interrupts
        match addr {
            0x0000..=0x7FFF => { Err(AddressError::ReadOnly(addr))    },
            0x8000..=0x9FFF => { self.ppu.vram_mut().write(addr, val) },
            0xA000..=0xBFFF => { self.cart.write_ext_ram(addr, val)   },
            0xC000..=0xCFFF => { self.ram.write(addr, val)            },
            0xD000..=0xDFFF => { self.ram.write(addr, val)            },
            0xFE00..=0xFE9F => { self.ppu.oam_mut().write(addr, val)  },
            0xFF00..=0xFF7F => { Err(AddressError::Other(addr))       },
            0xFF80..=0xFFFE => { self.hram.write(addr, val)           },
            0xFFFF          => { Err(AddressError::Other(addr))       },
            _ => Err(AddressError::Invalid(addr)),
        }
    }
}


#[cfg(test)]
#[path = "../../../tests/bus.rs"]
mod tests;
