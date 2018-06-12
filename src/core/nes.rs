/* Base module for the Nintendo Entertainment System.
 * Should have master functions for saving, loading, and booting.
 * 	Theoretically, this should be the place to implement some sort
 * 	of plugin system.
 *      Though, that is a large stretch goal.
 * Author: Spalynx.
 */
mod cpu;
mod cartridge;

struct NES {
    rom:    ROM,
    cpu:    CPU,
}

impl NES {
    //Avoiding properly booting in this function.
    pub fn new() -> NES {
        // There is utility in loading without instantiating (Load
        // multiple NES, and boot individually?).
        NES {
            rom:    ROM::new(),
            cpu:    CPU::new(),
        }
    }

    //Loads values in each hardware device, including rom-file. 
    pub fn boot(&mut self, game: &ROM) {
        
    }

    //This begins the fetch-decode-execute loop.
    // After booting, this should be run until termination.
    pub fn fde_loop (&mut self) {

    }

    //Ends fde loop, deallocates if needed.
    //This likely won't be needed considering the ultimate lack of
    // pointers needed so far, but it's a logical state to at least
    // define for now.
    pub fn shutdown (&mut self){

    }
}
