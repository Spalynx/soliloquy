/*  Emulates the MOS Technology 6052 CPU of the NES.
 *  Author: Spalynx
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
    #[macro_use]
    macro_rules! word_to_h_byte {
        ($w:expr) => ($w >> 8);
    }
    #[macro_use]
    macro_rules! word_to_l_byte {
        ($w:expr) => ($w & 0xff);
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

    pub pc:             u16,        //Program Counter - 65536 memory locations.
    pub cycles:         u64,        //Clock cycle counter.
                                    //      Other hardware relies on this. [5]

    pub sp:             u8,         //Stack Pointer - Accessed using interrupts,
                                    //     pulls, pushes, and transfers.

    pub a:              u8,         // Accumulator.
    pub x:              u8,         // X register.
    pub y:              u8,         // Y register.

    pub status:         u8,         // CPU Flags See [1] for reference

    pub interrupt:      u8,         // Interrupt type to perform.
    pub stall:          u8,         // Number of cycles to stall.
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

            pc:             0,		        //Program Counter
            cycles:         0,		        //Number of cycles

            sp:             0xFF,	        //Stack Pointer, \S 8.13 in KIM-1

            a:              0,		        //Accumulator
            x:              0,		        // x register
            y:              0,		        // y register

            status:         0,		        //cpu flags

            interrupt:      0,		        // interrupt type to perform
            stall:          0,		        // number of cycles to stall
        }
    }
    /// Though memory is already initialized, I felt it appropriate to
    /// keep the manual option available.
    pub fn new_memory(&mut self, mem: MEM){
        self.memory = mem;
    }

    ///Meta-Functions~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///--------------~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    
    fn parse_opcode(&mut self, OP: u8){
        match OP {
            0x00    =>  self.BRK(), 
            _       =>  panic!("OPCODE not implemented yet."),
        }
    }

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


    /// Sets the Z/N flags based upon a given value.
    ///     I think that this will be used often, but not sure how
    ///     much.
    fn set_zn(&mut self, val: u8){
        //Setting flags based upon accumulator value.
        if val == 0 {
            self.set_status(1, true);
            self.set_status(7, false);
        }
        else if val >= 128 {
            self.set_status(1, false);
            self.set_status(7, true);
        }
        else {
            self.set_status(1, false);
            self.set_status(7, false);
        }
    }


    //The stack is on page 1 of memory. ($0100-$01FF)
    //
    fn stack_push(&mut self, val: u8){
        self.memory.mem_stack_push(self.sp, val);
        self.sp = self.sp - 1;
    }
    // Pops an item from the stack and returns it.
    fn stack_pop(&mut self) -> u8 {
        self.sp = self.sp + 1;

        return self.memory.mem_stack_pop(self.sp);
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
    /// This function implements all of ADC functionality by simply negating the
    ///  value of the operand.
    /// NOTE: Without carry being added, the sent value is off-by-one. Supposedly
    ///       it is common practice to call SEC() prior to SBC(val).
    pub fn SBC <AM: AddressingMode>(&mut self, am: AM){
        let b: u8 = am.load(self) as u8;

        self.ADC(ImmediateAM{address: b^0xFF});
    }

    /// AND
    /// Bitwise AND with accumulator, takes memory address as parameter,
    /// and comp/replaces cpu.a.
    pub fn AND <AM: AddressingMode>(&mut self, am: AM){
        let a: u8 = self.a;
        let b: u8 = am.load(self);

        self.LDA(ImmediateAM{address: (a & b)});
    }

    /// ORA
    /// Bitwise OR with accumulator, param of memory val to bitwise OR cpu.a.
    pub fn ORA <AM: AddressingMode>(&mut self, am: AM){
        let a: u8 = self.a;
        let b: u8 = am.load(self);

        self.LDA(ImmediateAM{address: (a | b)})
    }

    /// EOR
    /// Bitwise XOR with accumulator.
    pub fn EOR <AM: AddressingMode>(&mut self, am: AM){
        let a: u8 = self.a;
        let b: u8 = am.load(self);

        self.LDA(ImmediateAM{address: (a ^ b)})
    }

    //#! Bit Manipulation
    /// ASL
    /// Arithmatic shift left. Shifts all bits left one position.
    /// 0 is shifted into bit 0 and original bit 7 is shifted to Carry.
    /// Modifies either accumulator or memory, this can be controlled via addressing modes.
    pub fn ASL <AM: AddressingMode>(&mut self, am: AM){
        let mut b: u8 = am.load(self);

        //Using SEC/CLC should be faster than set_status.
        if b & 128 == 128 {   self.SEC(); }
        else {                self.CLC(); }

        b <<= 1;
        self.set_zn(b);
        am.save(self, b);
    }

    /// LSR
    /// Logical Shift Right
    pub fn LSR <AM: AddressingMode>(&mut self, am: AM){
        let mut b: u8 = am.load(self);

        //Using SEC/CLC should be faster than set_status.
        if b & 1 == 1 {       self.SEC(); }
        else {                self.CLC(); }

        b >>= 1;
        self.set_zn(b);
        am.save(self, b);
    }

    /// ROL
    /// Rotate Left
    pub fn ROL <AM: AddressingMode>(&mut self, am: AM){
        let mut b: u8 = am.load(self);

        //End result of Carry
        let new_C =
            if b & 128 == 128 { true  }
            else              { false };

        //Perform shift.
        b <<= 1;

        //Check end result of bit 1.
        if self.get_status("C") { b |= 1; }

        self.set_status(0,new_C);
        self.set_zn(b);
        am.save(self, b);

    }

    /// ROR
    /// Rotate Right
    /// Available on Microprocessors after June, 1976.
    pub fn ROR <AM: AddressingMode>(&mut self, am: AM){
        let mut b: u8 = am.load(self);

        //End result of Carry
        let new_C =
            if b & 1 == 1 { true  }
            else          { false };

        //Perform shift
        b >>= 1;

        //Check end result of bit 7
        if self.get_status("C") { b |= 128; }

        self.set_status(0, new_C);
        self.set_zn(b);
        am.save(self, b);
    }

    /// DEC
    /// "Decrement Memory By One"
    /// Subtracts 1 from a memory location, changing the "Z" and "N" flags based
    ///  upon the resulting value.
    /// Theoretically, this is used by two's complement subtraction.
    /// As far as I know, implementing actual twos complement would be slower.
    pub fn DEC <AM: AddressingMode>(&mut self, am: AM){
        let dec_M = am.load(self) - 1;

        self.set_zn(dec_M);
        am.save(self, dec_M);
    }
    /// DEX
    /// "Decrement Index Register X by One"
    /// Decrements register X by one, and sets "Z" if the result is 0, and
    ///  sets "N" if the result is negative.
    pub fn DEX(&mut self) {
        let dec_X = (255) & (self.x as u16 - 1) as u8;

        self.set_zn(dec_X);
        self.x = dec_X;
    }
    /// DEY
    /// "Decrement Index Register Y by One"
    /// Decrements register Y by one, and sets "Z" if the result is 0, and
    ///  sets "N" if the result is negative.
    pub fn DEY(&mut self) {
        let dec_Y = (255) & (self.y as u16 - 1) as u8;

        self.set_zn(dec_Y);
        self.y = dec_Y;
    }
    /// INC
    /// "Increment Memory By One"
    pub fn INC<AM: AddressingMode>(&mut self, am: AM){
        let inc_M = (255) & (am.load(self) as u16 + 1) as u8;

        self.set_zn(inc_M);
        am.save(self, inc_M);
    }

    /// INX
    /// "Increment Index Register X by One"
    pub fn INX(&mut self) {
        let inc_X = (255) & (self.x as u16 + 1) as u8;

        self.set_zn(inc_X);
        self.x = inc_X;
    }
    /// INY
    /// "Increment Index Register Y by One"
    pub fn INY(&mut self) {
        let inc_Y = (255) & (self.y as u16 + 1) as u8;

        self.set_zn(inc_Y);
        self.y = inc_Y;
    }
    /// STA
    /// "Store Accumulator in Memory"
    /// Variable 'A' created because of borrowing issues, probably my bad.
    pub fn STA<AM: AddressingMode>(&mut self, am: AM){
        let A = self.a;
        am.save(self, A);
    }
    /// STX
    /// "Store Index Register X in Memory"
    pub fn STX<AM: AddressingMode>(&mut self, am: AM){
        let X = self.x;
        am.save(self, X);
    }

    /// STY
    /// "Store Index Register Y in Memory"
    pub fn STY<AM: AddressingMode>(&mut self, am: AM){
        let Y = self.y;
        am.save(self, Y);
    }

    /// TAX
    /// "Transfer Accumulator to Index X"
    pub fn TAX(&mut self) {
        let A = self.a;
        self.x = A;
        self.set_zn(A);
    }

    /// TAY
    /// "Transfer Accumulator to Index Y"
    pub fn TAY(&mut self) {
        let Y = self.y;
        self.a = Y;
        self.set_zn(Y);

    }

    /// TXA
    /// "Transfer Index X to Accumulator"
    pub fn TXA(&mut self) {
        let X = self.x;
        self.a = X;
        self.set_zn(X);

    }

    /// TYA
    /// "Transfer Index Y to Accumulator"
    pub fn TYA(&mut self) {
        let Y = self.y;
        self.a = Y;
        self.set_zn(Y);

    }

    /// TXS
    /// "Transfer Index X to Stack Pointer"
    pub fn TXS(&mut self) {
        self.sp = self.x;

    }

    /// TSX
    /// "Transfer Stack Pointer to Index"
    pub fn TSX(&mut self) {
        self.x = self.sp;

    }

    //#! General Operations

    /// BIT
    /// Test Bits in Memory with Accumulator
    /// Performans an AND between a memory location and the accumulator.
    /// ONLY CHANGES THE 'N, V, Z' FLAGS!
    ///   This is useful for testing one bit, and branching if 'Z' is set!
	
    /* TEMP! From KIM-1 Programming manual.
    This instruction performs an AND between a memory location
     and the accumulator but does not store the result of the AND into
     the accumulator.
            The symbolic notation is M /\ A.
            The bit instruction affects the N flag with N being set to
     the value of bit 7 of the memory being tested, the V flag with V
     being set equal to bit 6 of the memory being tested and Z being set
     by the result of the AND operation between the accumulator and the
     memory if the result is Zero, Z is reset otherwise.  It does not
     affect the accumulator.
    */
    pub fn BIT<AM: AddressingMode>(&mut self, am: AM){
        let M = am.load(self);

        //Set bit 7 ('N') to the value being tested.
        if M & 0b10000000 != 0 	{ self.set_status(7, true); }
        else 			{ self.set_status(7, false); }

        //Set bit 6 ('V') to the value being tested.
        if M & 0b01000000 != 0 	{ self.set_status(6, true); }
        else 			{ self.set_status(6, false); }

        //Set bit 0 ('Z') if the value is zero.
        if (M & self.a) == 0	{ self.set_status(1, true); }
        else 			{ self.set_status(1, false); }

    }


    /// NOP
    /// AFAIK, IT DOES NOTHING... PRODUCTIVITY!
    /// Arguably, it looks like this opcode is meant to be a way to manually step.
    /// This Opcode has given some anxiety, because of unofficial opcodes that are
    ///   effectively a NOP, but people have implied have differing timings.
    pub fn NOP(&self) {
    }

    /// BRK
    /// Break. Throws a NMI, and increments the program counter by one.
    ///  BRK is a 2 byte opcode. The first is #$00 and the second is a padding byte.
    ///  Since the PC increment/decrement is handled in the step function,
    ///  we skip that part.
    pub fn BRK(&mut self) {
        self.throw_interrupt("NMI");

        // PC + 2 \|/ (FFFE) -> PCL (FFFF) -> PCH
        // Microprocessor transfers control to the interrupt vector
        // The B flag is stored on the stack, at stack pointer plus 1, containing
        // a one in the break bit position. Indicating the interrupt was caused
        // by a BRK instruction.
        // The B bit in the stack contains a 0 if it was caused by a normal IRQ.

        //Push PC to stack.
        let PC = self.pc;
        self.stack_push(word_to_h_byte!(PC) as u8);
        self.stack_push(word_to_l_byte!(PC) as u8);

        //Set BRK flag on status.
        let P = self.status | 0b0010000;
        //push P
        self.stack_push(P);

        //PC = Vector
        self.pc = 0xFFFE
    }
    /// RTI
    /// Return from Interrupt
    /// Restores the microprocessor to the state previous to the interrupt.
    /// To do this, it reads P and PC from the stack into their places. 
    /// NOTE: Ignores P bits 4 (B) and 5 (s). Checking the B flag (post BRK) must
    /// done manually!
    pub fn RTI(&mut self) {
        let P:   u8 = self.stack_pop();
        let PCL: u8 = self.stack_pop();
        let PCH: u8 = self.stack_pop();

        self.status = P & 0b11001111;
        self.pc     = bytes_to_word!(PCH as u16, PCL as u16);
    }

    //#! Stack Manipulation

    /// PHA
    /// Push Accumulator on Stack
    /// Pushes the current accumulator value on the stack.
    pub fn PHA(&mut self) {
        let A: u8 = self.a;
        self.stack_push(A);
    }
    /// PLA
    /// Pull Accumulator from Stack
    /// Pops the top value off of the stack and places in accumulator.
    /// NOTE: This implies that popping might not clear the previous value!
    pub fn PLA(&mut self) {
        let A: u8 = self.stack_pop();
        self.a = A;
    }
    /// PHP
    /// Push Processor Status on Stack
    /// Pushes the status register (P) _unchanged_ onto the stack.
    pub fn PHP(&mut self) {
        let P: u8 = self.status;
        self.stack_push(P); 
    }
    /// PLP
    /// Pull Processor Status from Stack
    /// Pops the top stack value into the status register (P).
    pub fn PLP(&mut self) {
        let P: u8 = self.stack_pop();
        self.status = P;
    }
    /// RTS
    /// Return From Subroutine
    /// Loads PCL then PCH from stack, into PC and increments by 1 to point 
    /// to the instruction following the JSR.
    /// Note: Since two pops occur, SP gets decremented twice (technically incremented).
    pub fn RTS(&mut self) {
        let PCL: u16 = self.stack_pop() as u16;
        let PCH: u16 = self.stack_pop() as u16;

        self.pc = bytes_to_word!(PCH, PCL) + 1;
    }

    //#! Comparators (Probably used in jumping)

    /// CMP
    /// Compare Accumulator 
    /// The purpose of the compare instruction is to allow the user
    /// to compare a value in memory to the accumulator without changing
    /// the value of the accumulator.
    pub fn CMP<AM: AddressingMode>(&mut self, am: AM){
        let M:      u8 = am.load(self);
        let A:      u8 = self.a;
        let sub:    i16 = A as i16 - M as i16;

        self.set_status(0, M <= A);     //'C' set on memory being \leq accumulator.
        self.set_status(1, A == M);     //'Z' set if two values are equal.
        self.set_status(7, sub > 127);  //'N' set based on result bit 7
    }

    /// CPX 
    /// Compare X Register
    /// Seems to be the same thing as CMP but with X register.
    /// Operation and flag results are identical to equivalent mode accumulator CMP
    /// ops. 
    pub fn CPX<AM: AddressingMode>(&mut self, am: AM){
        let M:      u8 = am.load(self);
        let X:      u8 = self.x;
        let sub:    i16 = X as i16 - M as i16;

        self.set_status(0, M <= X);     //'C' set on memory being \leq X.
        self.set_status(1, X == M);     //'Z' set if two values are equal.
        self.set_status(7, sub < 0);  //'N' set based on result bit 7
    }

    /// CPY 
    /// Compare Y Register
    /// Seems to be the same thing as CMP but with Y register.
    /// Operation and flag results are identical to equivalent mode accumulator CMP
    /// ops. 
    pub fn CPY<AM: AddressingMode>(&mut self, am: AM){
        let M:      u8 = am.load(self);
        let Y:      u8 = self.y;
        let sub:    i16 = Y as i16 - M as i16;

        self.set_status(0, M <= Y);     //'C' set on memory being \leq Y.
        self.set_status(1, Y == M);     //'Z' set if two values are equal.
        self.set_status(7, sub < 0);   //'N' set based on result bit 7

    }

    //#! Branching/Jumping
    /// JSR
    /// Jump to Subroutine
    /// Takes a PC address (assembler replaces subroutine tag with the address)
    ///  as an argument. Stores the current PC on the stack (for RTS), and
    ///  overwrites the current PC value with the newly obtained value.
    /// Uses Absolute addressing, which means that the new PC is u16.
    pub fn JSR<AM: AddressingMode>(&mut self, am: AM){
        let PC_new  = am.address(); 
        let PCH: u8 = word_to_h_byte!(self.pc+2) as u8;
        let PCL: u8 = word_to_l_byte!(self.pc+2) as u8;

        self.stack_push(PCH);
        self.stack_push(PCL);
        self.pc = PC_new;
    }
    pub fn BCC(&self) {}
    pub fn BCS(&self) {}
    pub fn BEQ(&self) {}
    pub fn BMI(&self) {}
    pub fn BNE(&self) {}
    pub fn BPL(&self) {}
    pub fn JMP(&self) {}
    pub fn BVC(&self) {}
    pub fn BVS(&self) {}
} //IMPL CPU



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
    fn address (&self) -> u16; 
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

    fn address (&self) -> u16 { 0 as u16 } 
}
impl AddressingMode for ImmediateAM {
    #[allow(unused_variables)]
    fn load (&self, cpu: &mut CPU) -> u8
    {	self.address  }
    #[allow(unused_variables)]
    fn save (&self, cpu: &mut CPU, storeval: u8)
    {	panic!("No way to store in ImmediateAM!"); }

    fn address (&self) -> u16 { self.address as u16 } 
}
impl AddressingMode for AbsoluteAM {
    fn load (&self, cpu: &mut CPU) -> u8
    {	cpu.memory.get( self.address ) }
    fn save (&self, cpu: &mut CPU, storeval: u8)
    {	cpu.memory.set( self.address, storeval ); }
    fn address (&self) -> u16 { self.address as u16 } 
}
impl AddressingMode for AbsoluteXAM {
    fn load (&self, cpu: &mut CPU) -> u8
    {	cpu.memory.get( self.address + cpu.x as u16 ) }
    fn save (&self, cpu: &mut CPU, storeval: u8)
    {	cpu.memory.set( self.address + cpu.x as u16, storeval ); }
    fn address (&self) -> u16 { self.address as u16 } 
}
impl AddressingMode for AbsoluteYAM {
    fn load (&self, cpu: &mut CPU) -> u8
    {	cpu.memory.get( self.address + cpu.y as u16) }
    fn save (&self, cpu: &mut CPU, storeval: u8)
    {	cpu.memory.set( self.address + cpu.y as u16, storeval ); }
    fn address (&self) -> u16 { self.address as u16 } 
}
impl AddressingMode for ZeroPageAM {
    fn load (&self, cpu: &mut CPU) -> u8
    {	cpu.memory.get_zp( self.address ) }
    fn save (&self, cpu: &mut CPU, storeval: u8)
    {	cpu.memory.set_zp( self.address, storeval ); }
    fn address (&self) -> u16 { self.address as u16 } 
}
impl AddressingMode for ZeroPageXAM  {
    fn load (&self, cpu: &mut CPU) -> u8
    {	cpu.memory.get_zp( self.address+cpu.x ) }
    fn save (&self, cpu: &mut CPU, storeval: u8)
    {	cpu.memory.set_zp( self.address + cpu.x , storeval); }
    fn address (&self) -> u16 { self.address as u16 } 
}
impl AddressingMode for ZeroPageYAM  {
    fn load (&self, cpu: &mut CPU) -> u8
    {	cpu.memory.get_zp( self.address + cpu.y ) }
    fn save (&self, cpu: &mut CPU, storeval: u8)
    {	cpu.memory.set_zp( self.address + cpu.y , storeval); }
    fn address (&self) -> u16 { self.address as u16 } 
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
    fn address (&self) -> u16 { self.address as u16 } 
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
    fn address (&self) -> u16 { self.address as u16 } 
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
