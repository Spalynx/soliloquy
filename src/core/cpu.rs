/*  Emulates the MOS Technology 6052 CPU of the NES.
 *  Author: Spalynx
 *  Init: 12/8/17
 */

//Because OPCODES are cooler in CAPS!
    #![allow(non_snake_case)]

//Imports
    use core::memory::MEM;

//Test module definition.
    #[cfg(test)]
    #[path = "./cpu_test.rs"]
    pub mod cpu_test;


//General helper macro, might move to a general module.
    #[macro_use]
    macro_rules! bytes_to_word {
        ($h:expr,$l:expr) => (($h << 8) | ($l & 0xff));
    }

//CPU=DEFINITION================================================================
//==============================================================================

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
    pub memory:         MEM,
    pub cycles:         u64,    //Clock cycle counter.
                                //      Other hardware relies on this. [5]
    pub pc:             u16,    //Program Counter - 65536 memory locations.
    pub sp:             u8,     //Stack Pointer - Accessed using interrupts,
                                //     pulls, pushes, and transfers.

    pub a:              u8,     // Accumulator.
    pub x:              u8,     // X register.
    pub y:              u8,     // Y register.

    pub status:          u8,    // CPU Flags See [1] for reference

    pub interrupt:      u8,     // Interrupt type to perform.
    pub stall:          u8,     // Number of cycles to stall.
}


//~CPU-IMPLEMENTATION===========================================================
//==============================================================================

#[allow(dead_code)]
impl CPU {
    /// Initializes an empty CPU struct
    /// All values initialized as empty.
    pub fn new() -> CPU {
        CPU{
            memory:         MEM::new(),
            cycles:         0,    //Number of cycles
            pc:             0,    //Program Counter
            sp:             0,     //Stack Pointer

            a:              0,     //Accumulator
            x:              0,     // x register
            y:              0,     // y register

            status:         0,      //cpu flags

            interrupt:      0,     // interrupt type to perform
            stall:          0,    // number of cycles to stall
        }
    }
    /// Though memory is already initialized, I felt it appropriate to
    /// keep the manual option available.
    pub fn new_memory(&mut self, mem: MEM){
        self.memory = mem;
    }

    ///Meta-Functions~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///--------------~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    /// Runs a cpu cycle with each call
    /// TODO: At the moment, I'm not
    /// sure if I want to impl with a param of the incoming opcode, or
    /// if there should be some buffer?
    fn step() -> i32{
        /*
        let step = CpuStep {
            address:    0,
            pc:         0, 
            mode:       0, 
        }
        */

        1
    }

    fn parse_opcodes() {

    }

    /// Receives a string as a param, and throws one of the 3 (?) cpu
    /// interrupts.
    /// NOTE: This is me pushing off interrupts until I actually
    /// understand what they do.
    fn throw_interrupt(&mut self, interrupt: &'static str){
        println!("{}", interrupt);
    }


    /// Sets flags based upon a given byte.
    /// Bit crunching here to reduce overhead, as flags are going to change often.
    pub fn set_status(&mut self, status_num: u8, val: bool){
        //Panic on non-valid status.
        if status_num > 7 {
            panic!("Status position ${} is not possible!", status_num);
        }

        self.status = 	if val {self.status |  (1 << status_num)} //Setting status to true...
        		else   {self.status & !(1 << status_num)} //Setting status to false....
    }


    /// Sets flags based upon a given byte.
    pub fn set_status_old(&mut self, flag: &'static str, val: bool){

        // Yes, I know that this is a really slow way to access
        // flags, but I plan on refactoring with actual opcodes.
        
        //This whole system is messy, can't wait to remove.
        self.status = match flag.as_ref() {
            "N" | "n" => if val { self.status |  (1 << 7)}
                         else   { self.status & !(1 << 7)},
            "V" | "v" => if val { self.status |  (1 << 6)}
                         else   { self.status & !(1 << 6)},
            "S" | "s" => if val { self.status |  (1 << 5)}
                         else   { self.status & !(1 << 5)},
            "B" | "b" => if val { self.status |  (1 << 4)}
                         else   { self.status & !(1 << 4)},
            "D" | "d" => if val { self.status |  (1 << 3)}
                         else   { self.status & !(1 << 3)},
            "I" | "i" => if val { self.status |  (1 << 2)}
                         else   { self.status & !(1 << 2)},
            "Z" | "z" => if val { self.status |  (1 << 1)}
                         else   { self.status & !(1 << 1)},
            "C" | "c" => if val { self.status |  (1 << 0)}
                         else   { self.status & !(1 << 0)},
            _ => panic!("NOT A FLAG"),
        };
    }
    pub fn get_status(&mut self, flag: &'static str) -> bool{
        //Matches given flag number 
        return match flag.as_ref() {
            "N" | "n" => self.status & (1 << 7) == 128,
            "V" | "v" => self.status & (1 << 6) == 64,
            "S" | "s" => self.status & (1 << 5) == 32,
            "B" | "b" => self.status & (1 << 4) == 16,
            "D" | "d" => self.status & (1 << 3) == 8,
            "I" | "i" => self.status & (1 << 2) == 4,
            "Z" | "z" => self.status & (1 << 1) == 2,
            "C" | "c" => self.status & (1 << 0) == 1,
            _ => panic!("NOT A FLAG"),
        }
    }
    pub fn print_status(&self) {
        println!("N V - - D I Z C");
        let mut y = 128;

        while y > 0 {
            match &self.status & y {
                0 => print!("0 "),
                _ => print!("1 "),
            }
            y = y / 2;
        }
        println!();
    }


    /// Sets the Z/N flags based upon accumulator value.
    ///     I think that this will be used often, but not sure how
    ///     much.
    fn set_zn(&mut self, val: u8){
        //Setting flags based upon accumulator value.
        if val == 0 {
            self.set_status(1, true);
            self.set_status(7, false);
        }
        else if val > 128 {
            self.set_status(1, false);
            self.set_status(7, true);
        }
        else {
            self.set_status(1, false);
            self.set_status(7, false);
        }
    }

    ///CPU~Instruction~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///---------------~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Refer to [3] for all cpu explanations.

    //#! Flag OPCODES.
    //   Why not set_status_old()? Simply: Not fast enough.
    //   Wondering if I should make /fake/ OPs for other flags.
    /// CLC - Clear Carry. Sets carry to false.

    pub fn CLC(&mut self) {     self.status = self.status ^ (1);}
    /// CLD - Clear Decimal. Sets decimal to false.
    pub fn CLD(&mut self) {     self.status = self.status ^ (1 << 3);   }
    /// CLI - Clear Interrupt. Sets interrupt to false.
    pub fn CLI(&mut self) {     self.status = self.status ^ (1 << 2);   }
    /// CLV - Clear O*V*ERFLOW. Sets overflow to false.
    pub fn CLV(&mut self) {     self.status = self.status ^ (1 << 6);   }

    /// SEC  - SEt Carry. Sets carry to true.
    pub fn SEC(&mut self) {     self.status = self.status | (1 << 0);   }
    /// SED  - SEt Decimal. Sets decimal to true.
    pub fn SED(&mut self) {     self.status = self.status | (1 << 3);   }
    /// SEI  - SEt Interrupt. Sets interrupt to true.
    pub fn SEI(&mut self) {     self.status = self.status | (1 << 2);   }

    //#! Register Loads.
    /// LDA (LoaD Accumulator with memory)
    /// One of the most used opcodes, loads the accumulator with a given value.
    pub fn LDA <AM: AddressingMode> (&mut self, am: AM){
        self.a = am.load(self);
        let a = self.a; //Can't mutably borrow, but can't deref u8? ok.
        self.set_zn(a);
    }

    ///LDX (LoaD X with memory) 
    pub fn LDX <AM: AddressingMode> (&mut self, am: AM){
        self.x = am.load(self);

        let x = self.x;
        self.set_zn(x);
    }

    ///LDY (LoaD Y with memory) 
    pub fn LDY <AM: AddressingMode> (&mut self, am: AM){
        self.y = am.load(self);

        let y = self.y;
        self.set_zn(y);
    }

    //#! Primary Arithmetic Operations
    /// ADC
    /// ADd with Carry. ADC results are based on the decimal flag. In
    /// decimal mode, addition is carried out as if the values are in Binary
    /// Coded Decimal.
    pub fn ADC <AM: AddressingMode>(&mut self, am: AM){
        let a: u16 = self.a as u16;
        let b: u16 = am.load(self) as u16;
        let c: u16 = self.get_status("C") as u16; //Should be 1 or 0.
        
        //d is the sum.
        let d = a + b + c;

        self.set_status(1, d == 0);
        self.set_status(0, d > 255);
        self.set_status(7, d > 127);
        //From disch on nesdev.
        //Overflow is like carry for decimal math.
        self.set_status(6, ((a^d)&(b^d)&(128))==128 ); 

        self.a = d as u8;
    }

    /// SBC
    /// Subtract with borrow.
    pub fn SBC(&self) {}

    /// AND
    /// Bitwise AND with accumulator, takes memory address as parameter,
    /// and comp/replaces cpu.a.
    pub fn AND(&mut self) {}

    /// EOR
    /// Bitwise XOR with accumulator.
    pub fn EOR(&self) {}

    /// ORA
    /// Bitwise OR with accumulator, param of memory val to bitwise OR cpu.a.
    pub fn ORA(&mut self) {}

} //IMPL CPU

/*

    //#! Bit Manipulation
    /// ASL
    /// Arithmatic shift left. Shifts all bits left one position. 0 is shifted into bit 0 and original bit 7 is shifted to Carry.
    pub fn ASL(&mut self) {
        if self.a & 0x1000000 == 0x1000000 {
            self.set_status_old("C", true);
        } 
        else { self.set_status_old("C", false); }

        self.a = self.a << 1;
    }
    pub fn LSR(&self) {}
    pub fn ROL(&self) {}
    pub fn ROR(&self) {}



    //#! Register/Memory Manipulation
    pub fn DEC(&self) {}
    pub fn DEX(&self) {}
    pub fn DEY(&self) {}
    pub fn INC(&self) {}
    pub fn INX(&self) {}
    pub fn INY(&self) {}
    pub fn STA(&self) {}
    pub fn STX(&self) {}
    pub fn STY(&self) {}
    pub fn TAX(&self) {}
    pub fn TAY(&self) {}
    pub fn TSX(&self) {}
    pub fn TXA(&self) {}
    pub fn TXS(&self) {}
    pub fn TYA(&self) {}


    //#! General Operations
    pub fn BIT(&self) {}
    /// BRK
    /// Break. Throws a NMI, and increments the program counter by one.
    // BRK is a 2 byte opcode. The first is #$00 and the second is a padding byte.
    //Since the PC increment/decrement is handled in the step function, we skip that part.
    pub fn BRK(&mut self) {
        self.throw_interrupt("NMI");

        self.set_status_old("B", true);
        self.set_status_old("U", true);
    }
    /// NOP
    /// AFAIK, IT DOES NOTHING. PRODUCTIVITY.
    /// Arguably, it looks like this opcode is meant to be a way to manually step.
    pub fn NOP(&self) {}
    pub fn RTI(&self) {}
    pub fn RTS(&self) {}

    //#! Stack Manipulation
    pub fn PHA(&self) {}
    pub fn PHP(&self) {}
    pub fn PLA(&self) {}
    pub fn PLP(&self) {}

    //#! Comparators (Probably used in jumping)
    ///CMP (CoMPare accumulator) 
    ///Affects Flags: S Z C 
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

    //#! Branching/Jumping
    pub fn BCC(&self) {}
    pub fn BCS(&self) {}
    pub fn BEQ(&self) {}
    pub fn BMI(&self) {}
    pub fn BNE(&self) {}
    pub fn BPL(&self) {}
    pub fn JMP(&self) {}
    pub fn JSR(&self) {}
    pub fn BVC(&self) {}
    pub fn BVS(&self) {}
*/


//=ADDRESSING-MODES=============================================================
//==============================================================================
/// AddressingMode trait for opcodes within the CPU.
///
/// I think that the hardest part in the conceptualization of AM
/// is that they are not a trait of the CPU. Moreso, they are sort
/// of like a function trait.
///
/// #Example
/// ```
/// fn lda<AM: AddressingMode> (&mut self, ami: AM){
///     //ami = addressing mode interface
///     //	This is how you interact with load/save.
/// 
///     ami.load(self, cpu);
///     ami.save(self, cpu, 0x255);
/// }
/// self.lda(ZeroPageAM{0x2B});
/// ```
///
/// This struct emulates the NES cpu.
/// CPU holds within it: a set of registers, a connection to memory,
/// it's instruction set, and it commands to parse instructinos.
pub trait AddressingMode {
    fn load (&self, cpu: &mut CPU) -> u8;
    fn save (&self, cpu: &mut CPU, storeval: u8);
}

/// The commented numbers specify the addressing mode's int value in
/// the Instruction.mode table.

/// Indirect(10), Relative(9), and Accumulator either do not need
/// structs or, in accumulator's case are not given a number because
/// of it only being called when an operand is not given.
pub struct AccumulatorAM;    
pub struct ImmediateAM      {pub address: u8}      /*2*/ 
pub struct AbsoluteAM       {pub address: u16}     /*6*/ 
pub struct AbsoluteXAM      {pub address: u16}     /*7*/ 
pub struct AbsoluteYAM      {pub address: u16}     /*8*/ 
pub struct ZeroPageAM       {pub address: u8}      /*3*/  
pub struct ZeroPageXAM      {pub address: u8}      /*4*/ 
pub struct ZeroPageYAM      {pub address: u8}      /*5*/ 
pub struct IndexedIndirectAM{pub address: u8}      /*12*/ 
pub struct IndirectIndexedAM{pub address: u8}      /*11*/ 

//+! Fold these addressingmode impls.
impl AddressingMode for AccumulatorAM{
    fn load (&self, cpu: &mut CPU) -> u8
    {	cpu.a	}
    fn save (&self, cpu: &mut CPU, storeval: u8)
    {	cpu.a = storeval; }
}
impl AddressingMode for ImmediateAM {
    fn load (&self, cpu: &mut CPU) -> u8
    {	self.address  }
    fn save (&self, cpu: &mut CPU, storeval: u8)
    {	panic!("No way to store in ImmediateAM!"); }
}
impl AddressingMode for AbsoluteAM {
    fn load (&self, cpu: &mut CPU) -> u8
    {	cpu.memory.get( self.address ) }
    fn save (&self, cpu: &mut CPU, storeval: u8)
    {	cpu.memory.set( self.address, storeval ); }
}
impl AddressingMode for AbsoluteXAM {
    fn load (&self, cpu: &mut CPU) -> u8
    {	cpu.memory.get( self.address + cpu.x as u16 ) }
    fn save (&self, cpu: &mut CPU, storeval: u8)
    {	cpu.memory.set( self.address + cpu.x as u16, storeval ); }
}
impl AddressingMode for AbsoluteYAM {
    fn load (&self, cpu: &mut CPU) -> u8
    {	cpu.memory.get( self.address + cpu.y as u16) }
    fn save (&self, cpu: &mut CPU, storeval: u8)
    {	cpu.memory.set( self.address + cpu.y as u16, storeval ); }
}
impl AddressingMode for ZeroPageAM {
    fn load (&self, cpu: &mut CPU) -> u8
    {	cpu.memory.get_zp( self.address ) }
    fn save (&self, cpu: &mut CPU, storeval: u8)
    {	cpu.memory.set_zp( self.address, storeval ); }
}
impl AddressingMode for ZeroPageXAM  {
    fn load (&self, cpu: &mut CPU) -> u8
    {	cpu.memory.get_zp( self.address+cpu.x ) }
    fn save (&self, cpu: &mut CPU, storeval: u8)
    {	cpu.memory.set_zp( self.address + cpu.x , storeval); }
}
impl AddressingMode for ZeroPageYAM  {
    fn load (&self, cpu: &mut CPU) -> u8
    {	cpu.memory.get_zp( self.address + cpu.y ) }
    fn save (&self, cpu: &mut CPU, storeval: u8)
    {	cpu.memory.set_zp( self.address + cpu.y , storeval); }
}
impl AddressingMode for IndexedIndirectAM {
    fn load (&self, cpu: &mut CPU) -> u8 {
	let low  = cpu.memory.get_zp( self.address + cpu.x );
        let high = cpu.memory.get_zp( self.address + cpu.x + 1);

        cpu.memory.get( bytes_to_word!(high as u16,low as u16) )
    }
    fn save (&self, cpu: &mut CPU, storeval: u8){
	let low  = cpu.memory.get_zp( self.address + cpu.x );
        let high = cpu.memory.get_zp( self.address + cpu.x + 1);

        cpu.memory.set( bytes_to_word!(high as u16,low as u16), storeval );
    }
}
impl AddressingMode for IndirectIndexedAM {
    fn load (&self, cpu: &mut CPU) -> u8 {
	let low  = cpu.memory.get_zp( self.address );
        let high = cpu.memory.get_zp( self.address + 1);

        cpu.memory.get( bytes_to_word!(high as u16,low as u16) + cpu.y as u16 )
    }
    fn save (&self, cpu: &mut CPU, storeval: u8){
	let low  = cpu.memory.get_zp( self.address );
        let high = cpu.memory.get_zp( self.address + 1);

        cpu.memory.set( bytes_to_word!(high as u16,low as u16) + cpu.y as u16,
                        storeval );
    }
}



//=CPU-INSTRUCTION-INFO-TABLES==================================================
//==============================================================================
/// Contructs tables for all cpu instructions on all addressing modes.
///
/// When tables are initialized with fn new_instructions(), there are 256
/// indexes per table.
///
/// 6 tables exist: instruction names, instruction sizes, instruction modes,
/// instruction speeds, and instruction speeds when a page boundary is crossed.
pub struct Instructions {
    pub names:     [&'static str;256],
    pub sizes:     [u8;256],
    pub modes:     [u8;256],
    pub speeds:    [u8;256],
    pub paging:    [u8;256],
}

/// Initializes struct instruction with all possible 6502 instructions.
/// I would like to make this immutable, or something...
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


/* Temporary dead code.

macro_rules! vec_of_strings {
($($x:expr),*) => (vec![$($x.to_string()),*]);
}


#[allow(dead_code)]
/// Emulates a CPU step.
/// Contains fields for an address, programcounter, and cpu mode.
pub struct CpuStep {
	  address:    u16,
	  pc:         u16, 
	  mode:       u8, 
}
type cpuop = fn(CpuStep) -> u8;
*/
