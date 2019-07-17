pub use crate::core::{cartridge::*};

/// new_map initializes a Boxed struct with the mapper trait to act as
///  a mapper in the memory map for the cpu.
/// A mapper number is supplied, and a simple match selects the appropriate val
pub fn new_map (map_num: u8, cart: Box<CART>) -> Box<dyn MAP> {
    debug!("START -> Mapper Initialization in mappper #{}.", map_num);
    match map_num {
        0 => Box::new(Nrom{cart}) as Box<dyn MAP>,
        1 => Box::new(MMC1{cart}) as Box<dyn MAP>,
        _ => panic!("Mapper {} has not been implemented!", map_num),
    }
}

/// Basic MAP trait, to be used in mappers.
/// So far, only has a get and set function, but might eventually
///  have bank switching functions.
pub trait MAP {
    fn get(&self, address: u16) -> u8;
    fn set(&mut self, address: u16, val: u8);
    fn get_chr(&self, address: u16) -> u8;
    fn set_chr(&mut self, address: u16, val: u8);
}

/// Compatability goes up the ladder, I'm afraid.
/// EMPTY_MAP really only exists for the sake of cpu_test.rs
/// This is a lesson on why proper planning/studying/unit test tooling should
///  be done before implementation.
pub struct EMPTY_MAP; 
impl MAP for EMPTY_MAP{
    fn get(&self, address: u16) -> u8{ 0 }
    fn set(&mut self, address: u16, val: u8){ }
    fn get_chr(&self, address: u16) -> u8{ 0 }
    fn set_chr(&mut self, address: u16, val: u8){ }
}

/// Mapper #00, NROM
/// Probably the most simple mapper.
/// Possibly some RAM, no bank switching, no scrolling, etc.
/// USED: Donkey Kong, Balloon Fight, tests, etc.
/// NRAM IS IMPLIED NOT TO BE USED ON THIS BOARD [7]
pub struct Nrom {
    pub cart: Box<CART>,
}
impl MAP for Nrom {
    fn get(&self, address: u16) -> u8 {
        // -- Check flags 6, bit 1 if PRG RAM Exists
        // ---- If so, check size with flags 8.
        // ---- Load $6000-7FFF with RAM data.
        // -- Check flags 4, for PRG ROM size (in 16 kb units)
        // ---- Load $8000-$BFFF with ROM.
        // ---- If ROM size is 256, load $C000-$FFFF with last half of ROM
        // ---- If ROM size is 128, load $C000-$FFFF with ROM.
        //if self.cart.head[6] & 2 > 0 { //   if 
        if address < 0x8000 {
            0 //PRG RAM function -- DNE on this mapper.
        }
        else if self.cart.PRG.len() > 16384 {
            self.cart.PRG[
                if address > 0xBFFF {
                    address & 0x3FFF
                }
                else {
                    address & 0x7FFF
                }
            as usize]
        }
        else {
            self.cart.PRG[(address & 0x7FFF) as usize] //This should start it at 0.
        }
    }
    fn set(&mut self, address: u16, val: u8) {
        
    }
    fn get_chr(&self, address: u16) -> u8 {
        0
    }
    fn set_chr(&mut self, address: u16, val: u8){

    }
}

pub struct MMC1 {
    pub cart: Box<CART>,
}
impl MAP for MMC1 {
    fn get(&self, address: u16) -> u8 {
        // -- Check flags 6, bit 1 if PRG RAM Exists
        // ---- If so, check size with flags 8.
        // ---- Load $6000-7FFF with RAM data.
        // -- Check flags 4, for PRG ROM size (in 16 kb units)
        // ---- Load $8000-$BFFF with ROM.
        // ---- If ROM size is 256, load $C000-$FFFF with last half of ROM
        // ---- If ROM size is 128, load $C000-$FFFF with ROM.
        //if self.cart.head[6] & 2 > 0 { //   if 
//        bfc tech
        if address < 0x8000 {
            0 //PRG RAM function.
        }
        else if self.cart.PRG.len() > 16384 {
            self.cart.PRG[
                if address > 0xBFFF {
                    address & 0x3FFF
                }
                else {
                    address & 0x7FFF
                }
            as usize]
        }
        else {
            self.cart.PRG[(address & 0x7FFF) as usize] //This should start it at 0.
        }
    }
    fn set(&mut self, address: u16, val: u8) {
        
    }
    fn get_chr(&self, address: u16) -> u8 {
        0
    }
    fn set_chr(&mut self, address: u16, val: u8){

    }
}