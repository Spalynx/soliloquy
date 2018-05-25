/*  Emulates the MOS Technology 6052 CPU of the NES.
 *  Author: Spalynx
 *  Init: 12/8/17
 */
//TODO: For now, all opcodes receive simple value for the operand, this /might/ be changed to a more complex type.

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
/// This struct emulates the NES cpu.
/// CPU holds within it: a set of registers, a connection to memory,
/// it's instruction set, and it commands to parse instructinos.
pub struct CPU {
    pub memory:         i32,
    pub cycles:         u64,    //Clock cycle counter. Other hardware relies on this. [5]
    pub pc:             u16,    //Program Counter - Supports 65536 direct memory locations.
    pub sp:             u8,     //Stack Pointer - Accessed using interrupts, pulls, pushes, and transfers.

    pub a:              u8,     //Accumulator
    pub x:              u8,     // x register
    pub y:              u8,     // y register

    pub flags:          u8,     //CPU Flags See [1] for reference

    pub interrupt:      u8,     // interrupt type to perform pub stall:          i32,    // number of cycles to stall 
    //instructions:        &[&[&str]],
}

//cpu=implementation=================================================================================

//===================================================================================================

/// there will be two main function types here:
/// cpu instructions
/// cpu meta functions - like parsing opcodes, etc.
///
#[allow(dead_code)]
impl CPU {
    /// Initializes an empty CPU struct
    /// All values initialized as empty.
    pub fn new() -> CPU {
        CPU{
            memory:         0,
            cycles:         0,    //Number of cycles
            pc:             0,    //Program Counter
            sp:             0,     //Stack Pointer

            a:              0,     //Accumulator
            x:              0,     // x register
            y:              0,     // y register

            flags:          0,      //cpu flags

            interrupt:      0,     // interrupt type to perform
            stall:          0,    // number of cycles to stall
        }
    }

    //meta~functions~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Runs a cpu cycle with each call
    /// TODO: At the moment, I'm not sure if I want to impl with a param of the incoming opcode, or if there should be some buffer?
    fn step() -> i32{
        let step = CpuStep {
	          address:    0,
	          pc:         0, 
	          mode:       0, 

        1
    }

    fn parse_opcodes() {
        
    }

    /// throw_interrupt
    /// Receives a string as a param, and throws one of the 3 (?) cpu interrupts.
    /// NOTE: This is me pushing off interrupts until I actually understand what they do.
    fn throw_interrupt($mut self, int: &'static str){

    }
    //CPU~Instruction~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Refer to [3] for all cpu explanations.

    /// ADC
    /// ADd with Carry. ADC results are based on the decimal flag. In
    /// decimal mode, addition is carried out as if the values are in Binary
    /// Coded Decimal.
    pub fn ADC(&mut self, step: CpuStep) {
        let a: u16 = self.a as u16;
        let b: u16 = step.address;
        let c: u16 = self.get_flag("C") as u16;

        //Intentionally cutting off anything over 256.
        self.a = (a + b + c) as u8;

        //
        self.set_flag("Z", self.a > 0);
        self.set_flag("N", self.a > 0);


        //Standard stuff, filling carry if we go over 256.
        if a + b + c > 0xFF {
		        self.set_flag("C", true);
	      }
        else {
		        self.set_flag("C", false);
	      }

        //Now I'm legit copying Fogelman...
        //So, 0x80 = 128 and I think 1 << 8.
        // TODO understand this.
	      if (( a^b ) & 0x80) == 0 		&&	 (( a^self.a) & 0x80) != 0 {
		        self.set_flag("V", true);
	      }
        else {
		        self.set_flag("V", false);
        }

    }

    /// AND
    /// Bitwise AND with accumulator, takes memory address as parameter, and comp/replaces cpu.a.
    pub fn AND(&mut self, mem: u8) {
        /// For now, We're gonna act like the ROM interpreting program will supply
        /// the memory to the function. If this doesn't happen to be true, maybe we
        /// can read directly from file. (Seems messy).

        self.a = self.a & self.mem;

        self.set_flag("Z", self.a > 0);
        self.set_flag("N", self.a > 0);
    }
    
    /// ASL
    /// Arithmatic shift left. Shifts all bits left one position. 0 is shifted into bit 0 and original bit 7 is shifted to Carry.
    pub fn ASL(&mut self) {
        if self.a & 0x1000000 == 0x1000000 {
            self.set_flag("C", true);
        } 
        else { self.set_flag("C", false); }

        self.a = self.a << 1;
    }

    pub fn BCC(&self) {}
    pub fn BCS(&self) {}
    pub fn BEQ(&self) {}
    pub fn BIT(&self) {}
    pub fn BMI(&self) {}
    pub fn BNE(&self) {}
    pub fn BPL(&self) {}
    
    /// BRK
    /// Break. Throws a NMI, and increments the program counter by one.
    // BRK is a 2 byte opcode. The first is #$00 and the second is a padding byte.
    //Since the PC increment/decrement is handled in the step function, we skip that part.
    pub fn BRK(&mut self) {
        self.throw_interrupt("NMI");

        self.set_flag("B", true);
        self.set_flag("U", true);
    }
    pub fn BVC(&self) {}
    pub fn BVS(&self) {}

    /// CLC
    /// Clear Carry. Sets carry to false.
    pub fn CLC(&self) {
        self.set_flag("C", false);
    }

    /// CLD
    /// Clear Decimal. Sets decimal to false.
    pub fn CLD(&self) {
        self.set_flag("D", false);
    }

    /// CLI
    /// Clear Interrupt. Sets interrupt to false.
    pub fn CLI(&self) {
        self.set_flag("I", false);
    }

    /// CLV
    /// Clear O*V*ERFLOW. Sets overflow to false.
    pub fn CLV(&self) {
        self.set_flag("V", false);
    }

    ///CMP (CoMPare accumulator) 
    ///Affects Flags: S Z C 
    ///MODE           SYNTAX       HEX LEN TIM
    ///Immediate     CMP #$44      $C9  2   2
    ///Zero Page     CMP $44       $C5  2   3
    ///Zero Page,X   CMP $44,X     $D5  2   4
    ///Absolute      CMP $4400     $CD  3   4
    ///Absolute,X    CMP $4400,X   $DD  3   4+
    ///Absolute,Y    CMP $4400,Y   $D9  3   4+
    ///Indirect,X    CMP ($44,X)   $C1  2   6
    ///Indirect,Y    CMP ($44),Y   $D1  2   5+
    ///+ add 1 cycle if page boundary crossed
    ///Compare sets flags as if a subtraction had been carried out. If the value in the
    ///accumulator is equal or greater than the compared value, the Carry will be
    ///set. The equal (Z) and sign (S) flags will be set based on equality or lack
    ///thereof and the sign (i.e. A>=$80) of the accumulator. 
    pub fn CMP(&self) {}

    ///CPX (ComPare X register) 
    ///Operation and flag results are identical to equivalent mode accumulator CMP
    ///ops. 
    pub fn CPX(&self) {}

    ///CPY (ComPare Y register) 
    ///Operation and flag results are identical to equivalent mode accumulator CMP
    ///ops. 
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
    /// LDA (LoaD Accumulator with memory)
    /// One of the most used opcodes, loads the A register.
    pub fn LDA(&self, operand: i8) {
        if (operand >= 0x00){
            self.set_flag("Z", true);
            self.set_flag("N", false);
        }
        else if ( operand >= 0x00 || operand <= 0x7f){
            self.set_flag("N", false);
            self.set_flag("Z", false);
        }
        else {
            self.set_flag("N", true);
            self.set_flag("Z", false);
        }

        this.a = operand;
    }
    ///LDX (LoaD X with memory) 
    pub fn LDX(&self, operand: i8) {
        if (operand > 0){
            self.set_flag("N", false);
        }
        //sets negative flag equal to 7th bit
        //sets the zero flag if the operand is $#00
        this.x = operand;
    }

    pub fn LDY(&self) {}
    pub fn LSR(&self) {}

    /// NOP
    /// AFAIK, IT DOES NOTHING. PRODUCTIVITY.
    /// Arguably, it looks like this opcode is meant to be a way to manually step.
    pub fn NOP(&self) {
    }

    /// ORA
    /// Bitwise OR with accumulator, param of memory val to bitwise OR cpu.a.
    pub fn ORA(&mut self, mem: u8) {
        self.a = self.a | self.mem;

        self.set_flag("Z", self.a > 0);
        self.set_flag("N", self.a > 0);
    }
    pub fn PHA(&self) {}
    pub fn PHP(&self) {}
    pub fn PLA(&self) {}
    pub fn PLP(&self) {}
    pub fn ROL(&self) {}
    pub fn ROR(&self) {}
    pub fn RTI(&self) {}
    pub fn RTS(&self) {}
    pub fn SBC(&self) {}

    /// SEC 
    /// SEt Carry. Sets carry to true.
    pub fn SEC(&self) {
        self.set_flag("C", true);
    }

    /// SED 
    /// SEt Decimal. Sets decimal to true.
    pub fn SED(&self) {
        self.set_flag("D", true);
    }

    /// SEI 
    /// SEt Interrupt. Sets interrupt to true.
    pub fn SED(&self) {
        self.set_flag("I", true);
    }

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
    pub fn set_flag(&mut self, flag: &'static str, val: bool){

        self.flags = match flag.as_ref() {
            "N" | "n" => self.flags | (1 << 8),
            "V" | "v" => self.flags | (1 << 7),
            "s"       => self.flags | (1 << 6),
            "B" | "b" => self.flags | (1 << 5),
            "D" | "d" => self.flags | (1 << 4),
            "I" | "i" => self.flags | (1 << 3),
            "Z" | "z" => self.flags | (1 << 2),
            "C" | "c" => self.flags | (1 << 1),
        };
    }
        
    pub fn get_flag(&mut self, flag: &'static str) -> bool{

        return match flag.as_ref() {
            "N" | "n" => self.flags & (1 << 8) == 128,
            "V" | "v" => self.flags & (1 << 7) == 64,
            "s"       => self.flags & (1 << 6) == 32,
            "B" | "b" => self.flags & (1 << 5) == 16,
            "D" | "d" => self.flags & (1 << 4) == 8,
            "I" | "i" => self.flags & (1 << 3) == 4,
            "Z" | "z" => self.flags & (1 << 2) == 2,
            "C" | "c" => self.flags & (1 << 1) == 1,
        }
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




#[allow(dead_code)]
/// Emulates a CPU step.
///
/// #Examples
/// ```
//
/// ```
/// Contains fields for an address, programcounter, and cpu mode.
pub struct CpuStep {
	  address:    u16,
	  pc:         u16, 
	  mode:       u8, 
}
//type cpuop = fn(CpuStep) -> u8;


/// The cpu addressing modes enum, contains all possible cpu modes.
///
///
enum Am {
    Implied,//0
    Accumulator,//1
    Immediate,//2
    Zeropage,//3
    Zeropagex,//4
    Zeropagey,//5
    Absolute,//6
    Absolutex,//7
    Absolutey,//8
    Relative,//9
    Indirect,//10
    IndirectIndexed,//11
    IndexedIndirect,//12
}
    impl Am {
        fn call (&self, &cpu: CPU) -> u16 {
            return match self {
                Am::Implied => ,
                Am::Accumulator => ,
                Am::Immediate => ,
                Am::Zeropage => ,
                Am::Zeropagex => ,
                Am::Zeropagey => ,
                Am::Absolute => ,
                Am::Absolutex => ,
                Am::Absolutey => ,
                Am::Relative => ,
                Am::Indirect => ,
                Am::IndirectIndexed => ,
                Am::IndexedIndirect => 
            };
        }
    }



/// contructs tables for all cpu instructions on all addressing modes.
///
/// when tables are initialized with fn new_instructions(), there are 256
/// indexes per table.
///
/// 6 tables exist: instruction names, instruction sizes, instruction modes,
/// instruction speeds, and instruction speeds when a page boundary is crossed.
pub struct Instructions {
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
impl Instructions {
    pub fn new() -> Instructions {
        Instructions {
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
}





#[cfg(test)]
pub mod tests {
    //Tests could be greatly improved with
    // [before_each]/[assemblyintialize]... Maybe one day!
    use super::*;

    #[test]
    fn test_cpu_init(){
        let test_cpu = super::CPU::new();
        
        assert_eq!(test_cpu.memory,			0);
        assert_eq!(test_cpu.cycles,			0);
        assert_eq!(test_cpu.pc,				0);
        assert_eq!(test_cpu.sp,				0);
        assert_eq!(test_cpu.x,				0);
        assert_eq!(test_cpu.y,				0);
        assert_eq!(test_cpu.flags,			0);
        assert_eq!(test_cpu.interrupt,			0);
        assert_eq!(test_cpu.stall, 			0);
    }

    #[test]
    fn test_instruction_init(){
        let test_instr = super::Instruction::new();

        //I'm pretty sure that these fields are arrays, and therefore have len().
        //If this fails, DELET THIS!
        assert_eq!(test_instr.names.len()		,256);
        assert_eq!(test_instr.sizes.len()		,256);
        assert_eq!(test_instr.modes.len()		,256);
        assert_eq!(test_instr.speeds.len()	,256);
        assert_eq!(test_instr.paging.len()	,256);
    }

    #[test]
    fn test_flags(){
        let test_cpu = super::CPU::new();
        let flags = {"N", "n", "V", "v", "s", "B", "b", "D", "d", "I", "i", "Z",
        "z", "C", "c"};

        //Should cycle through each flag setting it true, and testing to see if
        // it actually happened.
        for f in flags {
            test_cpu.set_flag(f, 1);
            assert_eq!(test_cpu.get_flag(f), 1);

            //Reset flag.
            test_cpu.set_flag(f, 0);
        }
    }

    #[test]
    fn testOP_adc(){
        //Testing for basic ADd with Carry.
        let test_cpu = super::CPU::new();

        test_cpu.LDA(254);
        test_cpu.ADC(6);
        assert_eq!(test_cpu.A, 5);
        assert_eq!(test_cpu.get_flag("C"), 1);
    }
    #[test]
    fn testOP_adc_signed (){
        //Testing signed arithmetic.
        let test_cpu = super::CPU::new();

        test_cpu.LDA(0b01111111u8); //+127
        test_cpu.ADC(0b00000010u8); //+2
        assert_eq!(test_cpu.A, 0b10000001u8); // = -127
        //Overflow should be set because bit 7 is '1'.
        assert_eq!(test_cpu.get_flag("O"), 1);
    }
    #[test]
    fn testOP_adc_decimal(){
        //TODO: Test adc with a decimal.
        let test_cpu = super::CPU::new();

        test_cpu.SED();
        assert!();
    }


    /// TEST -> CLC, CLD, CLI, CLV, SEC, SED, SEI
    #[test]
    fn test_flagsetclear_instructions(){
        let test_cpu = super::CPU::new();
    }
}


//[1]	0: carry flag
//	1: zero flag
//  	2: interrupt disable flag
//  	3: decimal mode flag
//  	4: break command flag
//  	5: unused flag
//  	6: overflow flag
//  	7: negative flag
//
//[3] 	izx = indexed indirect =  12
//    	izy = indirect indexed = 11
//    	ind = indirect = 10
//    	zp = zero page = 3
//    	zpx = zero page x = 4
//    	zpy = zero page y = 5
//    	abs = absolute val = 6
//    	abx = absolute val x = 7 
//    	aby = absolute val y = 8
//    	imm = immediate = 2
//    	rel = relative = 9

/*
macro_rules! vec_of_strings {
($($x:expr),*) => (vec![$($x.to_string()),*]);
}
*/
