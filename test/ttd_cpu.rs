// Author: Mark Eikel
// Date 9/13/17
// This should only be used for testing of the virtual hardware.

mod core;
use core::cpu::*;

pub fn cpu_testall() -> bool {
    //Init test cpu.
    let mut c = new();
    
    //Fill with a set of values.
    c.memory = 10000;
    c.cycles = 200000;

    //Begin tests:
    println!("{}", c.add());
}

fn cpu_testADD() -> bool {

}
/*
fn cpu_test*() -> bool {

}
*/
