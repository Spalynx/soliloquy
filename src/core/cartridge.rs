extern crate memmap;

use self::memmap::Mmap;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;

pub struct CART {
    pub filename: &'static str, 
    pub fmt: bool, //True if NES2.0, false if iNES
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
            fmt: false,
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

        //Check format type
        if self.fmt { panic!("NES 2.0 not currently supported.");
            return Ok(());
        }

        //Store and read header.
        reader.read(&mut self.HEAD)?; // I think.. this works... maybe?
        self.section_sizes[1] = self.HEAD[4] as u32; //This is in 16kb units!
        self.section_sizes[2] = self.HEAD[5] as u32; //This is in 8kb units!
        self.section_sizes[0] = if self.HEAD[6] & 0b100 == 0b100 {512} else {0};

        //Fill trainer if it exists.
        if self.section_sizes[0] > 0 {
            reader.read(&mut self.TRAIN)?;
        }
        
        //Filling variable length PRG
        let mut i = 0;
        let s = self.section_sizes[1]*16384; 
        for byte in reader.bytes() {
            self.PRG.push(byte.unwrap());

            if i >= s { break;}
            i += 1;
        }

        //Filling variable length CHR 
        let mut i = 0;
        let s = self.section_sizes[2]*8192; 
        for byte in reader.bytes() {
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
