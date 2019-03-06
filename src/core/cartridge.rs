extern crate memmap;

use self::memmap::Mmap;
use std::fs::File;

pub struct ROM {
    filename: String, 
}

impl ROM {
    pub fn new(file_n: String) -> ROM {
    	ROM{
            filename:   file_n,
        }
    }
}