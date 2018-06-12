pub mod cpu;
pub mod memory;
pub mod nes;
pub mod cartridge;

use core::cpu::*;
use core::cartridge::*;

//A note to make: When casting u16 to usize, that means that as long as this is running on a 16-bit architecture, this should run? It is pointer sized...    Odd.
