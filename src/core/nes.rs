/* Base module for the Nintendo Entertainment System.
 * Should have master functions for saving, loading, and booting.
 * 	Theoretically, this should be the place to implement some sort
 * 	of plugin system.
 *      Though, that is a large stretch goal.
 * Author: Spalynx.
 */

pub use crate::core::*;
pub use crate::core::cpu::OP_SIZES;

const DEBUG_ROM: bool = true;


pub struct NES {
    cpu:    CPU,
}
impl NES {
    //Loads values for each hardware device, including rom-file. 
    pub fn new(file_n: &'static str) -> NES {
        //Creates a CART type that will be read as memory later.
        let mut cart = CART::new(file_n);

        //Reads values from ROM file into memory.
        //This CART data can now be used to propogate 16-bit address space. 
        match cart.read_cart(){
            Ok(e)    => debug!("COMPLETE -> ROM read."),
            Err(e)   => panic!("ERROR    -> ROM read."),
        }

        //Find and create mapper.
        let map_num = (cart.HEAD[7] & 240) 
                        + (((cart.HEAD[6]) & 240) >> 4);
        let mapper: Box<MAP> = new_map(map_num, Box::new(cart));
        debug!("COMPLETE -> Mapper init.");

        //PPU init
        let ppu: u8 = 0;
        debug!("COMPLETE -> PPU init.");
        //APU init
        let apu: u8 = 0;
        debug!("COMPLETE -> APU init.");
        //Input init
        let input: u8 = 0;
        debug!("COMPLETE -> INPUT init.");

        //Main memory map init
        let mut memory = MEM::new(mapper, ppu, apu, input); 
        debug!("COMPLETE -> MEM init.");


        //CPU init
        NES{
            cpu: CPU::new(memory),
        }

    }
    //+ Further Boot Stuff.
    //-+ Read CHR ROM write data to PPU
    //-+ Code starts to read its ROM data and writes to APU registers
    //-+ Code waits for your input to make selection


    //Runs a step for CPU, APU, PPU. In theory these should run async,
    // so, this function, and similar ones will hold timing logic.
    pub fn step(&mut self) {
        //CPU running code

        //Run a step from each piece of hardware!
        self.cpu.step();
        //self.cpu.memory.ppu.step(); //Theoretically how PPU is called.        


/*


        while val <= RANGE_MAX{
            OP = self.cart.HEAD[val];
            P1 = if val+1 >= RANGE_MAX { 0 } else { self.cart.HEAD[val+1] };
            P2 = if val+2 >= RANGE_MAX { 0 } else { self.cart.HEAD[val+2] };

            self.cpu.parse_opcode(OP, P1, P2);
            debug!("CPU -> OPCODE -> [{:02X}] with possible args [{:03}] [{:03}] -- OP SIZE: {} -- A: {:02X}", OP, P1, P2, self::OP_SIZES[OP as usize], self.cpu.a);

            val = val + self::OP_SIZES[OP as usize] as usize;

        }
        self.cpu.print_status();
        */
    }

    //Ends fde loop, deallocates if needed.
    //This likely won't be needed considering the ultimate lack of
    // pointers needed so far, but it's a logical state to at least
    // define for now.
    pub fn shutdown (&mut self){

    }

    ///**Currently NIL** Intended to use serialization to save the NES state.
    pub fn save (&mut self){}
    ///**Currently NIL** Intended to use serialization to load the NES state.
    pub fn load (&mut self){}
}
