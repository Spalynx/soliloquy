// Author: Mark Eikel
// Date 9/13/17
// This should only be used for testing of the virtual hardware.

mod core;
use core::cpu::*;
fn main() {
    let mut c = new();
    c.memory = 10000;
    c.cycles = 200000;
    println!("{}", c.add());
}
