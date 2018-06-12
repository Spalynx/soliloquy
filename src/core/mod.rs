pub mod cpu;
pub mod memory;
pub mod nes;
pub mod cartridge;

pub use core::cartridge::*;
pub use core::cpu::*;
pub use core::memory::*;
pub use core::nes::*;

//A note to make: When casting u16 to usize, that means that as long as this is running on a 16-bit architecture, this should run? It is pointer sized...    Odd.
