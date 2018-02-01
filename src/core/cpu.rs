/*  Emulates the MOS Technology 6052 CPU of the NES.
 *  Author: Spalynx
 *  Date: 12/8/17
 */

//I'm pretty sure I have no clue whats going on!

//CPU=DEFINITION====================================================================================
//==================================================================================================

/// The MOS Technology 6052 CPU of the NES.
///
/// #Examples
///
/// ```
/// let mut c = new_cpu();
/// c.memory = 10000;
/// c.cycles = 200000;
/// println!("{}", c.add());
/// ```
///
/// Currently, I don't have a thorough enough understanding of rust types
/// to effectively make the CPU safe (private fields). I think that I might
/// for now just treat the struct as a OOP class, and implement getters/setters.
///
/// This struct emulates the NES cpu.
/// CPU holds within it: a set of registers, a connection to memory,
/// it's instruction set, and it commands to parse instructinos.
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

/// Initializes an empty CPU struct
///
/// All values initialized as empty.
/// 
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
    }
}



#[allow(dead_code)]
/// Emulates a CPU step.
///
/// #Examples
/// ```
///
/// ```
/// Contains fields for an address, programcounter, and cpu mode.
pub struct CpuStep {
	  address:    u16,
	  pc:         u16, 
	  mode:       u8, 
}
type cpuop = fn(cpustep) -> u8;


/// the cpu addressing modes enum, contains all possible cpu modes.
///
///
enum am {
    implied,//0
    accumulator,//1
    immediate,//2
    zeropage,//3
    zeropagex,//4
    zeropagey,//5
    absolute,//6
    absolutex,//7
    absolutey,//8
    relative,//9
    indirect,//10
    indirectindexed,//11
    indexedindirect,//12
}



/// contructs tables for all cpu instructions on all addressing modes.
///
/// when tables are initialized with fn new_instructions(), there are 256
/// indexes per table.
///
/// 6 tables exist: instruction names, instruction sizes, instruction modes,
/// instruction speeds, and instruction speeds when a page boundary is crossed.
pub struct instructions {
    pub names:		[&'static str;256],
    pub sizes:		[u8;256],
    pub modes:		[u8;256],
    pub speeds:		[u8;256],
    pub paging:	  [u8;256],
}

/// initializes struct instruction with all possible 6502 instructions.
///
/// i'm not sure how much memory this process takes, but efforts were made
/// to use values of lower sizes.
///
/// see [6502 unofficial opcodes](https://wiki.nesdev.com/w/index.php/cpu_unofficial_opcodes)
/// for the full table and info.
pub fn new_instruction() -> instructions {
    instructions {
        names:  
            ["brk", "ora", "stp", "slo", "nop", "ora", "asl", "slo", "php", "ora",
            "asl", "anc", "nop", "ora", "asl", "slo", "bpl", "ora", "stp", "slo",
            "nop", "ora", "asl", "slo", "clc", "ora", "nop", "slo", "nop", "ora",
            "asl", "slo", "jsr", "and", "stp", "rla", "bit", "and", "rol", "rla",
            "plp", "and", "rol", "anc", "bit", "and", "rol", "rla", "bmi", "and",
            "stp", "rla", "nop", "and", "rol", "rla", "sec", "and", "nop", "rla",
            "nop", "and", "rol", "rla", "rti", "eor", "stp", "sre", "nop", "eor",
            "lsr", "sre", "pha", "eor", "lsr", "alr", "jmp", "eor", "lsr", "sre",
            "bvc", "eor", "stp", "sre", "nop", "eor", "lsr", "sre", "cli", "eor",
            "nop", "sre", "nop", "eor", "lsr", "sre", "rts", "adc", "stp", "rra",
            "nop", "adc", "ror", "rra", "pla", "adc", "ror", "arr", "jmp", "adc",
            "ror", "rra", "bvs", "adc", "stp", "rra", "nop", "adc", "ror", "rra",
            "sei", "adc", "nop", "rra", "nop", "adc", "ror", "rra", "nop", "sta",
            "nop", "sax", "sty", "sta", "stx", "sax", "dey", "nop", "txa", "xaa",
            "sty", "sta", "stx", "sax", "bcc", "sta", "stp", "ahx", "sty", "sta",
            "stx", "sax", "tya", "sta", "txs", "tas", "shy", "sta", "shx", "ahx",
            "ldy", "lda", "ldx", "lax", "ldy", "lda", "ldx", "lax", "tay", "lda",
            "tax", "lax", "ldy", "lda", "ldx", "lax", "bcs", "lda", "stp", "lax",
            "ldy", "lda", "ldx", "lax", "clv", "lda", "tsx", "las", "ldy", "lda",
            "ldx", "lax", "cpy", "cmp", "nop", "dcp", "cpy", "cmp", "dec", "dcp",
            "iny", "cmp", "dex", "axs", "cpy", "cmp", "dec", "dcp", "bne", "cmp",
            "stp", "dcp", "nop", "cmp", "dec", "dcp", "cld", "cmp", "nop", "dcp",
            "nop", "cmp", "dec", "dcp", "cpx", "sbc", "nop", "isc", "cpx", "sbc",
            "inc", "isc", "inx", "sbc", "nop", "sbc", "cpx", "sbc", "inc", "isc",
            "beq", "sbc", "stp", "isc", "nop", "sbc", "inc", "isc", "sed", "sbc",
            "nop", "isc", "nop", "sbc", "inc", "isc"],
        sizes: //todo: fill sizes.
            [1; 256],

        modes: 
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
            7, 7, 7, 7],

        speeds: 
            [7, 6, 0, 8, 3, 3, 5, 5, 0, 2, 0, 2, 4, 4, 6, 6, 2, 5, 0, 8, 4, 4, 6,
            6, 0, 4, 0, 7, 4, 4, 7, 7, 6, 6, 0, 8, 3, 3, 5, 5, 0, 2, 0, 2, 4, 4,
            6, 6, 2, 5, 0, 8, 4, 4, 6, 6, 0, 4, 0, 7, 4, 4, 7, 7, 0, 6, 0, 8,
            3, 3, 5, 5, 0, 2, 0, 2, 3, 4, 6, 6, 2, 5, 0, 8, 4, 4, 6, 6, 0, 4, 0,
            7, 4, 4, 7, 7, 0, 6, 0, 8, 3, 3, 5, 5, 0, 2, 0, 2, 5, 4, 6, 6, 2, 5,
            0, 8, 4, 4, 6, 6, 0, 4, 0, 7, 4, 4, 7, 7, 2, 6, 2, 6, 3, 3, 3, 3, 0,
            2, 0, 2, 4, 4, 4, 4, 2, 6, 0, 6, 4, 4, 4, 4, 0, 5, 0, 5, 5, 5, 5, 5, 2,
            6, 2, 6, 3, 3, 3, 3, 0, 2, 0, 2, 4, 4, 4, 4, 2, 5, 0, 5, 4, 4, 4, 4,
            0, 4, 0, 4, 4, 4, 4, 4, 2, 6, 2, 8, 3, 3, 5, 5, 0, 2, 0, 2, 4, 4,
            6, 6, 2, 5, 0, 8, 4, 4, 6, 6, 0, 4, 0, 7, 4, 4, 7, 7, 2, 6, 2, 8,
            3, 3, 5, 5, 0, 2, 0, 2, 4, 4, 6, 6, 2, 5, 0, 8, 4, 4, 6, 6, 0, 4, 0,
            7, 4, 4, 7, 7],

        paging: 
            [7, 6, 0, 8, 3, 3, 5, 5, 0, 2, 0, 2, 4, 4, 6, 6, 3, 6, 0, 8, 4, 4, 6,
            6, 0, 5, 0, 7, 5, 5, 7, 7, 6, 6, 0, 8, 3, 3, 5, 5, 0, 2, 0, 2, 4, 4,
            6, 6, 3, 6, 0, 8, 4, 4, 6, 6, 0, 5, 0, 7, 5, 5, 7, 7, 0, 6, 0, 8,
            3, 3, 5, 5, 0, 2, 0, 2, 3, 4, 6, 6, 4, 6, 0, 8, 4, 4, 6, 6, 0, 5, 0,
            7, 5, 5, 7, 7, 0, 6, 0, 8, 3, 3, 5, 5, 0, 2, 0, 2, 5, 4, 6, 6, 3, 6,
            0, 8, 4, 4, 6, 6, 0, 5, 0, 7, 5, 5, 7, 7, 2, 6, 2, 6, 3, 3, 3, 3, 0,
            2, 0, 2, 4, 4, 4, 4, 3, 6, 0, 6, 4, 4, 4, 4, 0, 5, 0, 5, 5, 5, 5, 5, 2,
            6, 2, 6, 3, 3, 3, 3, 0, 2, 0, 2, 4, 4, 4, 4, 3, 6, 0, 6, 4, 4, 4, 4,
            0, 5, 0, 5, 5, 5, 5, 5, 2, 6, 2, 8, 3, 3, 5, 5, 0, 2, 0, 2, 4, 4,
            6, 6, 3, 6, 0, 8, 4, 4, 6, 6, 0, 5, 0, 7, 5, 5, 7, 7, 2, 6, 2, 8,
            3, 3, 5, 5, 0, 2, 0, 2, 4, 4, 6, 6, 3, 6, 0, 8, 4, 4, 6, 6, 0, 5, 0,
            7, 5, 5, 7, 7],
    }
}



//cpu=implementation=================================================================================
//===================================================================================================

/// there will be two main function types here:
/// cpu instructions
/// cpu meta functions - like parsing opcodes, etc.
///
#[allow(dead_code)]
impl cpu {

    //meta~functions~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// runs a cpu cycle with each call
    /// 
    ///
    fn step() -> i32{
        1
    }

    fn parse_opcodes() {
        
    }

    //CPU~Instruction~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Refer to [3] for all cpu explanations.
    /// This doesn't exist in CPU, this is just messing with impl.
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
    /// Sets flags based upon a given byte.
    /// TODO: Change to set register and print register.
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

/*
macro_rules! vec_of_strings {
($($x:expr),*) => (vec![$($x.to_string()),*]);
}
*/
