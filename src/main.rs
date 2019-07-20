// Author: Spalynx 
// INIT Date: 9/13/17
// This should only be used for testing of the virtual hardware.

extern crate soliloquy;
pub mod core;

#[macro_use]
extern crate log;
extern crate env_logger;

fn main() {
    env_logger::from_env(env_logger::Env::default().default_filter_or("debug"))
        .init();

    debug!("COMPLETE -> Logger init.");
    let mut nes_main = 
        core::nes::NES::new("example/nestest.nes");
    debug!("COMPLETE -> NES boot/CPU boot");
    for i in 1..=200 { 
        debug!("INSTRUCTION: #{}", i );
        nes_main.step();
    }
}
