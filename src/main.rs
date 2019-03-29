// Author: Mark Eikel
// Date 9/13/17
// This should only be used for testing of the virtual hardware.

mod core;
use core::cartridge::*;

fn main() {
    let mut c = CART::new("example/example.rom");
    match c.read_cart(){
        Ok(e)    => println!("ROM read correctly."),
        Err(e)   => panic!("ROM read ERR"),
    }

    println!("{:?}", c.raw_cart);
}
