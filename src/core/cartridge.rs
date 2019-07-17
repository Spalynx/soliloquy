//extern crate memmap;
//use self::memmap::Mmap;

use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;
use std::fmt;               //Implementing fmt::Debug.

pub use ::log::*;

#[allow(non_snake_case)]

pub struct CART {
    pub filename: &'static str, 
    pub ines_fmt: bool, //True if ROM is in iNES format.
    pub nes2_fmt: bool, //True if ROM is NES 2.0 format.
    pub section_sizes: [u32;5], // Differing sizes of certain registers:
                                // [0] : Trainer, either 0 or 512.
                                // [1] : PRG_ROM 
                                // [2] : CHR_ROM, can be 0.
                                // [3] : INST_ROM, either 0 or 8192.
                                // [4] : PROM, either 0 or 32.
    pub HEAD: [u8;16],
    pub TRAIN: [u8;512],
    pub PRG: Vec<u8>,
    pub CHR: Vec<u8>,
    pub INST_ROM: [u8;8192],
    pub PROM: [u8;32],
}

impl CART {
    pub fn new(file_n: &'static str) -> CART {
    	CART {
            filename:   file_n,
            ines_fmt: false,
            nes2_fmt: false,
            section_sizes: [0;5], 
            HEAD: [0;16],
            TRAIN: [0;512],
            PRG:  Vec::new(), //Possibly make a Box<[T]>
            CHR: Vec::new(),
            INST_ROM: [0;8192],
            PROM: [0;32],
        }
    }
    ///TODO: Differentiate between iNES and NES2.0
    pub fn read_cart(&mut self) -> io::Result<()> { 
        let mut f = File::open(self.filename.to_string())?;
        let mut reader = BufReader::new(f);

        //Store and read header.
        reader.read(&mut self.HEAD)?; // I think.. this works... maybe?
        self.section_sizes[1] = self.HEAD[4] as u32; //This is in 16kb units!
        self.section_sizes[2] = self.HEAD[5] as u32; //This is in 8kb units!
        self.section_sizes[0] = if self.HEAD[6] & 0b100 == 0b100 {512} else {0};

        //Check for iNES format identifier. 
        if self.HEAD[0]==0x4E && self.HEAD[1]==0x45 
            && self.HEAD[2]==0x53 && self.HEAD[3]==0x1A {
                self.ines_fmt = true;
        } 
        //Check for NES 2.0 format identifier.
        if self.ines_fmt && ((self.HEAD[7] & 0x0C) == 0x08) {
            self.nes2_fmt = true;
        }
        //Snippet from nesdev -> NES_2.0#Identification

        //Handle NES 2.0 format.
        if self.ines_fmt && self.nes2_fmt { 
            panic!("NES 2.0 not currently supported.");
        }

        //Fill trainer if it exists.
        if self.section_sizes[0] > 0 {
            reader.read(&mut self.TRAIN)?;
        }
        
        //Filling variable length PRG
        let mut i = 0;
        let s = self.section_sizes[1]*16384; 
        for byte in reader.by_ref().bytes() {
            self.PRG.push(byte.unwrap());

            if i >= s { break;}
            i += 1;
        }

        //Filling variable length CHR 
        let mut i = 0;
        let s = self.section_sizes[2]*8192; 
        for byte in reader.by_ref().bytes() {
            self.CHR.push(byte.unwrap());

            if i >= s { break;}
            i += 1;
        }

        //INST-ROM, when I need it
        //PROM, when I need it
        //Name bytes if I need it.
        Ok(())
    }

    pub fn fill_banks(){

    }
}
impl fmt::Debug for CART {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,   "Filename: {} \n
                     INES: {}\tNES2.0: {} \n 
                     SECT-Sizes: {:?} \n 
                     HEAD: {:?}\n",
                     self.filename, self.ines_fmt, self.nes2_fmt, self.section_sizes, self.HEAD)
    }
}