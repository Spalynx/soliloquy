// Author: Mark Eikel
// Date 9/13/17
// This should only be used for testing of the virtual hardware.

mod core;
use core::cpu::*;
fn main() {
    let mut c = CPU::new();
    c.memory = 10000;
    c.cycles = 200000;
    println!("{}", c.add());

    //Messing with CPU flags
    println!("Printing CPU flags:");
    c.print_flags();
    println!("Setting CPU flags to: 10001010 ");
    c.set_flag("N", true);
    c.set_flag("Z", true);
    c.set_flag("D", true);
    
    println!("Printing CPU flags:");
    c.print_flags();

}
