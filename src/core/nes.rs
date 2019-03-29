/* Base module for the Nintendo Entertainment System.
 * Should have master functions for saving, loading, and booting.
 * 	Theoretically, this should be the place to implement some sort
 * 	of plugin system.
 *      Though, that is a large stretch goal.
 * Author: Spalynx.
 */
//use super::*;
use core::cartridge;
use core::cpu::CPU;

pub struct NES {
    //rom:    ROM,
    cpu:    CPU,
}

impl NES {
    //Avoiding properly booting in this function.
    pub fn new() -> NES {
        // There is utility in loading without instantiating (Load
        // multiple NES, and boot individually?).
        NES {
            //rom:    ROM::new("".to_string()),
            cpu:    CPU::new(),
        }
    }

    //Loads values for each hardware device, including rom-file. 
    pub fn boot(&mut self, filepath: &str) {
        //Cartridge maps ROM contents into RAM/16-bit addr space
        //self.rom = ROM::new(filepath.to_string());
        //CPU running code

        //PPU init

        //Read CHR ROM write data to PPU

        //Code starts to read its ROM data and writes to APU registers

        //Code waits for your input to make selection

    }

    //Runs a step for CPU, APU, PPU. In theory these should run async,
    // so, this function, and similar ones will hold timing logic.
    pub fn step(&mut self) {

    }

    //Ends fde loop, deallocates if needed.
    //This likely won't be needed considering the ultimate lack of
    // pointers needed so far, but it's a logical state to at least
    // define for now.
    pub fn shutdown (&mut self){

    }
}
