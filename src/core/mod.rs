pub mod cpu;
pub mod memory;
pub mod nes;
pub mod cartridge;
pub mod mapper;

pub use crate::core::cartridge::*;
pub use crate::core::cpu::*;
pub use crate::core::memory::*;
pub use crate::core::nes::*;
pub use crate::core::mapper::*;

//A note to make: When casting u16 to usize, that means that as long as this is running on a 16-bit architecture, this should run? It is pointer sized...    Odd.