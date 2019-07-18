/* Emulates the NES MOS6502 CPU memory map.
 * Author: Spalynx
 *--------------Memory Map---------------------------------------------
 * $0000-$07FF     =      Internal CPU RAM
 * $4020-$FFFF     =      Cartridge Space and Misc (Interrupt Vectors).
 *---------------------------------------------------------------------
 */

pub use crate::core::*;
#[allow(non_snake_case)]
#[allow(dead_code)]
//Truthfully, I don't think that I'll be able to fully emulate cartridge space.
//I think that Cartridge.rs will just simply use standard std::fs to access PC
// locations.
pub struct MEM {
    RAM:	[u8; 0x800],        //2kb internal RAM.
    pub CART:   Box<MAP>,    //Cartridge Space
    PPU:    u8,         //TODO
    APU:    u8,         //TODO
    INPUT:  u8,         //TODO

}

impl MEM {
    /// Simple empty initialization used for unit testing.
    /// I have no clue how hard this will be to modify when most 
    ///  struct values are not u8.
    pub fn new_empty() -> MEM {
        return MEM {
            RAM:	    [0; 0x800],
            CART:	    Box::new(EMPTY_MAP),
            PPU:        0,
            APU:        0,
            INPUT:      0,
        }
    }
    //Initializes the full memory map of the NES.
    // TODO: PPU, APU, and INPUT registers are unimplemented!
    pub fn new(mapper: Box<MAP>, ppu: u8, apu: u8, input: u8) -> MEM {
        return MEM {
            RAM:	    [0; 0x800],
            CART:	    mapper,
            PPU:        ppu,
            APU:        apu,
            INPUT:      input,
        }
    }
    

    //Obtains values from full memory map.
    pub fn get(&self, address: u16) -> u8 {
        if address <= 0x800{
            //2kb internal ram
            self.RAM[address as usize]
        }
        else if address >= 0x4020 {
            self.CART.get(address) 
        }
        else {
            panic!("Other values in the memory map not implemented yet!");
        }
    }

    //Much faster, only has to access the first page of memory.
    pub fn get_zp(&self, address: u8) -> u8 {
        let zp = address & 255;
        return self.RAM[zp as usize];
    }

    // block any illegal storing.
    pub fn set(&mut self, address: u16, val: u8){
        if address <= 0x800 {
            //2kb internal ram
            self.RAM[address as usize] = val;
        }
        else if address >= 0x4020 {
            //~6kb Cartridge space.
            self.CART.set(address, val);
        }
        else {
            panic!("Other values in the memory map not implemented yet!");
        }
    }
    //Sets a value in the zero page.
    //Much faster, only has to access the first page of memory.
    pub fn set_zp(&mut self, address: u8, val: u8) {
        let zp = address & 255;
        self.RAM[zp as usize] = val;
    }

    //Pushes a byte onto the stack.
    //Called by cpu.stack_push to actually modify memory.
    pub fn mem_stack_push(&mut self, sp: u8, val: u8){
        self.RAM[(0xFF + (sp as usize))] = val;
    }
    //Pops an item off of the stack, and returns it as a u8.
    //Called by cpu.stack_pop to actually modify memory.
    pub fn mem_stack_pop(&mut self, sp: u8) -> u8{
        let temp: u8 = self.RAM[(0xFF + (sp as usize))];
        self.RAM[(0xFF + (sp as usize))] = 0;
        return temp;
    }
}


/*Memory locations $200 to $5ff map to the screen pixels. Different values will
draw different colour pixels. The colours are:

$0: Black
$1: White
$2: Red
$3: Cyan
$4: Purple
$5: Green
$6: Blue
$7: Yellow
$8: Orange
$9: Brown
$a: Light red
$b: Dark grey
$c: Grey
$d: Light green
$e: Light blue
$f: Light grey

I think this might be easy 6502 specific...
 */
//JESUS CHRIST, why did they have to make it all confusing and call the High Order Bytes pages?
//Now Zero Paging seems really fukken obvious!
