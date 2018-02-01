// Author: Mark Eikel
// Date 9/13/17
// This should only be used for testing of the virtual hardware.

mod core;
use core::cpu::*;
fn main() {
    let mut c = new_cpu();
    c.memory = 10000;
    c.cycles = 200000;
    println!("{}", c.add());

    //Messing with CPU flags
    let new_flags = 85;
    println!("Printing CPU flags:");
    c.print_flags();
    println!("Setting CPU flags to: {}", new_flags);
    c.set_flags(new_flags);
    println!("Printing CPU flags:");
    c.print_flags();

}
