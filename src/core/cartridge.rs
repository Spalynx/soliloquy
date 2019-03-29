extern crate memmap;

use self::memmap::Mmap;
use std::fs::File;
use std::io;
use std::io::prelude::*;

pub struct CART {
    pub filename: &'static str, 
    pub raw_cart: Vec<u8>,
}

impl CART {
   
    pub fn new(file_n: &'static str) -> CART {
    	CART {
            filename:   file_n,
            raw_cart: Vec::new(),
        }
    }
    pub fn read_cart(&mut self) -> io::Result<()> { 
        let mut f = File::open(self.filename.to_string())?;

        for byte in f.bytes(){
           self.raw_cart.push(byte.unwrap()); 
        }
        Ok(())
    }
}
