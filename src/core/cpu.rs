/*  Emulates the MOS Technology 6052 CPU of the NES.
 *  Author: Spalynx
 *  Date: 12/8/17
 */

//I'm pretty sure I have no clue whats going on!

//CPU=DEFINITION====================================================================================

/** Emulate a CPU STEP */
#[allow(dead_code)]
pub struct CpuStep {
	  address:    u16,
	  pc:         u16, 
	  mode:       u8, 
}
//type CpuOp = fn(CpuStep) -> i8;


/** CPU struct definition.
 *
 *  This struct emulates the NES cpu.
 *  CPU holds within it: a set of registers, a connection to memory,
 *  it's instruction set, and it commands to parse instructinos.
 */
pub struct CPU {
    pub memory:         i32,
    pub cycles:         u64,    //Number of cycles
    pub pc:             u16,    //Program Counter
    pub sp:             u8,     //Stack Pointer

    pub x:              u8,     // x register
    pub y:              u8,     // y register

    pub flags:          u8,     //CPU Flags See [1] for reference

    pub interrupt:      u8,     // interrupt type to perform
    pub stall:          i32,    // number of cycles to stall

    //instructions:        &[&[&str]],
}

/* Initializes an empty CPU struct. */
pub fn new_cpu() -> CPU {
    CPU{
        memory:         0,
        cycles:         0,    //Number of cycles
        pc:             0,    //Program Counter
        sp:             0,     //Stack Pointer

        x:              0,     // x register
        y:              0,     // y register

        flags:          0,      //cpu flags

        interrupt :     0,     // interrupt type to perform
        stall:          0,    // number of cycles to stall
        //instructions:        &[&[&str]],
    }
}
enum {
    AMImplied,//0
    AMAccumulator,//1
    AMImmediate,//2
    AMZeroPage,//3
    AMZeroPageX,//4
    AMZeroPageY,//5
    AMAbsolute,//6
    AMAbsoluteX,//7
    AMAbsoluteY,//8
    AMRelative,//9
    AMIndirect,//10
    AMIndirectIndexed,//11
    AMIndexedIndirect,//12
}

//CPU=IMPLEMENTATION=================================================================================

/** There will be two main function types here:
 *  CPU Instructions
 *  CPU Meta Functions - Like parsing opcodes, etc.
 */
#[allow(dead_code)]
impl CPU {

    //Meta~Functions~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /** Runs a CPU cycle with each call
     *
     */
    fn step() -> i32{
        1
    }

    fn parse_opcodes() {
        
    }

    //CPU~Instruction~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    //Refer to [3] for all cpu explanations.
    //This doesn't exist in CPU, this is just messing with impl.
    pub fn add(&self) -> u64 {
        self.memory as u64 + self.cycles
    }

    pub fn ADC(&self) {}
    pub fn AND(&self) {}
    pub fn ASL(&self) {}
    pub fn BCC(&self) {}
    pub fn BCS(&self) {}
    pub fn BEQ(&self) {}
    pub fn BIT(&self) {}
    pub fn BMI(&self) {}
    pub fn BNE(&self) {}
    pub fn BPL(&self) {}
    pub fn BRK(&self) {}
    pub fn BVC(&self) {}
    pub fn BVS(&self) {}
    pub fn CLC(&self) {}
    pub fn CLD(&self) {}
    pub fn CLI(&self) {}
    pub fn CLV(&self) {}
    pub fn CMP(&self) {}
    pub fn CPX(&self) {}
    pub fn CPY(&self) {}
    pub fn DEC(&self) {}
    pub fn DEX(&self) {}
    pub fn DEY(&self) {}
    pub fn EOR(&self) {}
    pub fn INC(&self) {}
    pub fn INX(&self) {}
    pub fn INY(&self) {}
    pub fn JMP(&self) {}
    pub fn JSR(&self) {}
    pub fn LDA(&self) {}
    pub fn LDX(&self) {}
    pub fn LDY(&self) {}
    pub fn LSR(&self) {}
    pub fn NOP(&self) {}
    pub fn ORA(&self) {}
    pub fn PHA(&self) {}
    pub fn PHP(&self) {}
    pub fn PLA(&self) {}
    pub fn PLP(&self) {}
    pub fn ROL(&self) {}
    pub fn ROR(&self) {}
    pub fn RTI(&self) {}
    pub fn RTS(&self) {}
    pub fn SBC(&self) {}
    pub fn SEC(&self) {}
    pub fn SED(&self) {}
    pub fn SEI(&self) {}
    pub fn STA(&self) {}
    pub fn STX(&self) {}
    pub fn STY(&self) {}
    pub fn TAX(&self) {}
    pub fn TAY(&self) {}
    pub fn TSX(&self) {}
    pub fn TXA(&self) {}
    pub fn TXS(&self) {}
    pub fn TYA(&self) {}

    //DEBUG~!~!~!~!~!~!~!~!~!~!~!~!~!~!~!~!~!~!~!~!
    pub fn set_flags(&mut self, a: u8){
        self.flags = a;

    }
    pub fn print_flags(&self) {
        println!("N V - - D I Z C");

        let mut y = 128;

        while y > 0 {
            match &self.flags & y {
                0 => print!("0 "),
                _ => print!("1 "),
            }
            y = y / 2;
        }

        println!();
        
    }
}


/* Constructs a table of CPU instructions based upon what addressing mode they are run with.
 *
 */
pub struct Instructions {
    pub names:		&[str],
    pub sizes:		&[u8],
    pub modes:		&[u8],
    pub speeds:		&[u8],
}
/*
pub fn new_instruction() -> Instructions {
    let names: &[str] = 
        ["BRK", "ORA", "STP", "SLO", "NOP", "ORA", "ASL", "SLO", "PHP", "ORA",
        "ASL", "ANC", "NOP", "ORA", "ASL", "SLO", "BPL", "ORA", "STP", "SLO",
        "NOP", "ORA", "ASL", "SLO", "CLC", "ORA", "NOP", "SLO", "NOP", "ORA",
        "ASL", "SLO", "JSR", "AND", "STP", "RLA", "BIT", "AND", "ROL", "RLA",
        "PLP", "AND", "ROL", "ANC", "BIT", "AND", "ROL", "RLA", "BMI", "AND",
        "STP", "RLA", "NOP", "AND", "ROL", "RLA", "SEC", "AND", "NOP", "RLA",
        "NOP", "AND", "ROL", "RLA", "RTI", "EOR", "STP", "SRE", "NOP", "EOR",
        "LSR", "SRE", "PHA", "EOR", "LSR", "ALR", "JMP", "EOR", "LSR", "SRE",
        "BVC", "EOR", "STP", "SRE", "NOP", "EOR", "LSR", "SRE", "CLI", "EOR",
        "NOP", "SRE", "NOP", "EOR", "LSR", "SRE", "RTS", "ADC", "STP", "RRA",
        "NOP", "ADC", "ROR", "RRA", "PLA", "ADC", "ROR", "ARR", "JMP", "ADC",
        "ROR", "RRA", "BVS", "ADC", "STP", "RRA", "NOP", "ADC", "ROR", "RRA",
        "SEI", "ADC", "NOP", "RRA", "NOP", "ADC", "ROR", "RRA", "NOP", "STA",
        "NOP", "SAX", "STY", "STA", "STX", "SAX", "DEY", "NOP", "TXA", "XAA",
        "STY", "STA", "STX", "SAX", "BCC", "STA", "STP", "AHX", "STY", "STA",
        "STX", "SAX", "TYA", "STA", "TXS", "TAS", "SHY", "STA", "SHX", "AHX",
        "LDY", "LDA", "LDX", "LAX", "LDY", "LDA", "LDX", "LAX", "TAY", "LDA",
        "TAX", "LAX", "LDY", "LDA", "LDX", "LAX", "BCS", "LDA", "STP", "LAX",
        "LDY", "LDA", "LDX", "LAX", "CLV", "LDA", "TSX", "LAS", "LDY", "LDA",
        "LDX", "LAX", "CPY", "CMP", "NOP", "DCP", "CPY", "CMP", "DEC", "DCP",
        "INY", "CMP", "DEX", "AXS", "CPY", "CMP", "DEC", "DCP", "BNE", "CMP",
        "STP", "DCP", "NOP", "CMP", "DEC", "DCP", "CLD", "CMP", "NOP", "DCP",
        "NOP", "CMP", "DEC", "DCP", "CPX", "SBC", "NOP", "ISC", "CPX", "SBC",
        "INC", "ISC", "INX", "SBC", "NOP", "SBC", "CPX", "SBC", "INC", "ISC",
        "BEQ", "SBC", "STP", "ISC", "NOP", "SBC", "INC", "ISC", "SED", "SBC",
        "NOP", "ISC", "NOP", "SBC", "INC", "ISC"];

    //TODO: Fill instruction tables.
    let sizes: &[i8:256] =
        [1; 256];
    let modes: &[i8:256] =
        [0, 12, 0, 12, 3, 3, 3, 3, 3, 2, 2, 2, 6, 6, 6, 6, 9, 11, 0, 11, 4, 4,
        4, 4, 2, 8, 2, 8, 7, 7, 7, 7, 6, 12, 0, 12, 3, 3, 3, 3, 4, 2, 2, 2, 6,
        6, 6, 6, 9, 11, 0, 11, 4, 4, 4, 4, 2, 8, 2, 8, 7, 7, 7, 7, 6, 12, 0, 12,
        3, 3, 3, 3, 3, 2, 2, 2, 6, 6, 6, 6, 9, 11, 0, 11, 4, 4, 4, 4, 2, 8, 2,
        8, 7, 7, 7, 7, 6, 12, 0, 12, 3, 3, 3, 3, 4, 2, 2, 2, 10, 6, 6, 6, 9, 11,
        0, 11, 4, 4, 4, 4, 2, 8, 2, 8, 7, 7, 7, 7, 2, 12, 2, 12, 3, 3, 3, 3, 2,
        2, 2, 2, 6, 6, 6, 6, 9, 11, 0, 11, 4, 4, 5, 5, 2, 8, 2, 8, 7, 7, 8, 8,
        2, 12, 2, 12, 3, 3, 3, 3, 2, 2, 2, 2, 6, 6, 6, 6, 9, 11, 0, 11, 4, 4, 5,
        5, 2, 8, 2, 8, 7, 7, 8, 8, 2, 12, 2, 12, 3, 3, 3, 3, 2, 2, 2, 2, 6, 6,
        6, 6, 9, 11, 0, 11, 4, 4, 4, 4, 2, 8, 2, 8, 7, 7, 7, 7, 2, 12, 2, 12, 3,
        3, 3, 3, 2, 2, 2, 2, 6, 6, 6, 6, 9, 11, 0, 11, 4, 4, 4, 4, 2, 8, 2, 8,
        7, 7, 7, 7];

    let speeds: &[i8:255] =
        [7, 6, 0, 8, 3, 3, 5, 5, 0, 2, 0, 2, 4, 4, 6, 6, 2*, 5*, 0, 8, 4, 4, 6,
        6, 0, 4*, 0, 7, 4*, 4*, 7, 7, 6, 6, 0, 8, 3, 3, 5, 5, 0, 2, 0, 2, 4, 4,
        6, 6, 2*, 5*, 0, 8, 4, 4, 6, 6, 0, 4*, 0, 7, 4*, 4*, 7, 7, 0, 6, 0, 8,
        3, 3, 5, 5, 0, 2, 0, 2, 3, 4, 6, 6, 2*, 5*, 0, 8, 4, 4, 6, 6, 0, 4*, 0,
        7, 4*, 4*, 7, 7, 0, 6, 0, 8, 3, 3, 5, 5, 0, 2, 0, 2, 5, 4, 6, 6, 2*, 5*,
        0, 8, 4, 4, 6, 6, 0, 4*, 0, 7, 4*, 4*, 7, 7, 2, 6, 2, 6, 3, 3, 3, 3, 0,
        2, 0, 2, 4, 4, 4, 4, 2*, 6, 0, 6, 4, 4, 4, 4, 0, 5, 0, 5, 5, 5, 5, 5, 2,
        6, 2, 6, 3, 3, 3, 3, 0, 2, 0, 2, 4, 4, 4, 4, 2*, 5*, 0, 5*, 4, 4, 4, 4,
        0, 4*, 0, 4*, 4*, 4*, 4*, 4*, 2, 6, 2, 8, 3, 3, 5, 5, 0, 2, 0, 2, 4, 4,
        6, 6, 2*, 5*, 0, 8, 4, 4, 6, 6, 0, 4*, 0, 7, 4*, 4*, 7, 7, 2, 6, 2, 8,
        3, 3, 5, 5, 0, 2, 0, 2, 4, 4, 6, 6, 2*, 5*, 0, 8, 4, 4, 6, 6, 0, 4*, 0,
        7, 4*, 4*, 7, 7];
}
*/


// pub &[&[f64]]

//[1] https://wiki.nesdev.com/w/index.php/CPU_status_flag_behavior
//  0: carry flag
//  1: zero flag
//  2: interrupt disable flag
//  3: decimal mode flag
//  4: break command flag
//  5: unused flag
//  6: overflow flag
//  7: negative flag
//[2] https://wiki.nesdev.com/w/index.php/CPU_unofficial_opcodes
//[3] http://obelisk.me.uk/6502/reference.html
//    izx = indexed indirect =  12
//    izy = indirect indexed = 11
//    ind = indirect = 10
//    zp = zero page = 3
//    zpx = zero page x = 4
//    zpy = zero page y = 5
//    abs = absolute val = 6
//    abx = absolute val x = 7 
//    aby = absolute val y = 8
//    imm = immediate = 2
//    rel = relative = 9
