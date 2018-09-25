// Author: Mark Eikel
// Date 9/13/17
// This should only be used for testing of the virtual hardware.

mod core;
use core::cpu::*;

fn main() {
    let mut c = CPU::new();
    print!("Testing.");

    c.LDA(ImmediateAM{address: 0xFF});
    c.print_status();
    c.LDX(ImmediateAM{address: 0x00});
    c.print_status();
    c.LDY(ImmediateAM{address: 0xFF});
    c.print_status();
    c.LDA(ImmediateAM{address: 0x00});
    c.print_status();
    c.LDX(ImmediateAM{address: 0xFF});
    c.print_status();
    c.LDY(ImmediateAM{address: 0x00});
    c.print_status();
}
