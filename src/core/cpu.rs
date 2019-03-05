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
    /// Arguably, it looks like this opcode is meant to be a way to manually
    ///  step. This Opcode has given some anxiety, because of unofficial
    ///  opcodes that are effectively a NOP that people have implied have
    ///  differing timings.
    pub fn NOP(&self) {
    }

    /// BRK
    /// Break. Throws a NMI, and increments the program counter by one.
    ///  BRK is a 2 byte opcode. The first is #$00 and the second is a padding
    ///  byte.
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
    /// NOTE: Ignores P bits 4 (B) and 5 (s). Checking the B flag (post BRK)
    /// must be done manually!
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

    /// JMP
    /// Jump to New Location
    /// Loads program counter value from a given memory value, and it's
    /// subsequent location.
    /// AKA: (address) -> PCL, (address+1) -> PCH
    pub fn JMP<AM: AddressingMode>(&mut self, am: AM){
        let PC_L = am.load(self) as u16;
        let ADDR = am.address() + 1;
        let PC_H = AbsoluteAM{address: ADDR}.load(self) as u16;

        self.pc = bytes_to_word!(PC_H, PC_L);
    }

    /// A base function for branching.
    /// Applies the concept of relative addressing.
    ///   A new u8 is obtained as the offset and added to the PC.
    ///   This is signed arithmetic, and therefore can reduce the PC. 
    pub fn branch_base(&mut self, offset: u8){
        if offset > 128 {
            self.pc -= offset as u16;
        }
        else {
            self.pc += offset as u16;
        }
    }

    /// BCC
    /// Branch on Carry Clear
    /// Adds the offset value given to PC if condition is met. 
    pub fn BCC<AM: AddressingMode>(&mut self, am: AM){
        let offset = am.load(self);

        if self.status & 1 == 0 {
            self.branch_base(offset);
        }
    }
    /// BCS
    /// Branch on Carry Set
    /// Adds the offset value given to PC if condition is met. 
    pub fn BCS<AM: AddressingMode>(&mut self, am: AM){
        let offset = am.load(self);

        if self.status & 1 == 1 {
            self.branch_base(offset);
        }
    }
    /// BEQ
    /// Branch on Result Zero
    /// Adds the offset value given to PC if condition is met. 
    pub fn BEQ<AM: AddressingMode>(&mut self, am: AM){
        let offset = am.load(self);

        if self.status & 2 == 2 {
            self.branch_base(offset);
        }
    }
    /// BMI
    /// Branch on Result Minus
    /// Adds the offset value given to PC if condition is met. 
    pub fn BMI<AM: AddressingMode>(&mut self, am: AM){
        let offset = am.load(self);

        if self.status & 128 == 128 {
            self.branch_base(offset);
        }
    }
    /// BNE
    /// Branch on Result Not Zero
    /// Adds the offset value given to PC if condition is met. 
    pub fn BNE<AM: AddressingMode>(&mut self, am: AM){
        let offset = am.load(self);

        if self.status & 2 == 0 {
            self.branch_base(offset);
        }
    }
    /// BPL
    /// Branch on Result Plus
    /// Adds the offset value given to PC if condition is met. 
    pub fn BPL<AM: AddressingMode>(&mut self, am: AM){
        let offset = am.load(self);

        if self.status & 128 == 0 {
            self.branch_base(offset);
        }
    }
    /// BVC
    /// Branch on Overflow Clear
    /// Adds the offset value given to PC if condition is met. 
    pub fn BVC<AM: AddressingMode>(&mut self, am: AM){
        let offset = am.load(self);

        if self.status & 64 == 0 {
            self.branch_base(offset);
        }
    }
    /// BVS
    /// Branch on Overflow Set
    /// Adds the offset value given to PC if condition is met. 
    pub fn BVS<AM: AddressingMode>(&mut self, am: AM){
        let offset = am.load(self);

        if self.status & 64 == 64 {
            self.branch_base(offset);
        }
    }

    fn parse_opcode(&mut self, OP: u8, ARG: u16){
        match OP {
            0x00    => self.BRK( ---Implicit ),
            0x01    => self.ORA(IndexedIndirectAM{address: ARG}), 
            0x02    => self.SLO(IndexedIndirectAM{address: ARG}), 
            0x03    => self.NOP(ZeroPageAM{address: ARG}), 
            0x04    => self.ORA(ZeroPageAM{address: ARG}), 
            0x05    => self.ASL(ZeroPageAM{address: ARG}), 
            0x06    => self.SLO(ZeroPageAM{address: ARG}), 
            0x07    => self.PHP(ZeroPageAM{address: ARG}), 
            0x08    => self.ORA(ImmediateAM{address: ARG}), 
            0x09    => self.ASL(ImmediateAM{address: ARG}), 
            0x0A    => self.ANC(ImmediateAM{address: ARG}), 
            0x0B    => self.NOP(AbsoluteAM{address: ARG}), 
            0x0C    => self.ORA(AbsoluteAM{address: ARG}), 
            0x0D    => self.ASL(AbsoluteAM{address: ARG}), 
            0x0E    => self.SLO(AbsoluteAM{address: ARG}), 
            0x0F    => self.BPL(---Relative), 
            0x10    => self.ORA(IndirectIndexedAM{address: ARG}), 
            0x11    => self.STP(---Implicit), 
            0x12    => self.SLO(IndirectIndexedAM{address: ARG}), 
            0x13    => self.NOP(ZeroPageXAM{address: ARG}), 
            0x14    => self.ORA(ZeroPageXAM{address: ARG}), 
            0x15    => self.ASL(ZeroPageXAM{address: ARG}), 
            0x16    => self.SLO(ZeroPageXAM{address: ARG}), 
            0x17    => self.CLC(ImmediateAM{address: ARG}), 
            0x18    => self.ORA(AbsoluteYAM{address: ARG}), 
            0x19    => self.NOP(ImmediateAM{address: ARG}), 
            0x1A    => self.SLO(AbsoluteYAM{address: ARG}), 
            0x1B    => self.NOP(AbsoluteXAM{address: ARG}), 
            0x1C    => self.ORA(AbsoluteXAM{address: ARG}), 
            0x1D    => self.ASL(AbsoluteXAM{address: ARG}), 
            0x1E    => self.SLO(AbsoluteXAM{address: ARG}), 
            0x1F    => self.JSR(AbsoluteAM{address: ARG}), 
            0x20    => self.AND(IndexedIndirectAM{address: ARG}), 
            0x21    => self.STP(---Implicit), 
            0x22    => self.RLA(IndexedIndirectAM{address: ARG}), 
            0x23    => self.BIT(ZeroPageAM{address: ARG}), 
            0x24    => self.AND(ZeroPageAM{address: ARG}), 
            0x25    => self.ROL(ZeroPageAM{address: ARG}), 
            0x26    => self.RLA(ZeroPageAM{address: ARG}), 
            0x27    => self.PLP(ZeroPageXAM{address: ARG}), 
            0x28    => self.AND(ImmediateAM{address: ARG}), 
            0x29    => self.ROL(ImmediateAM{address: ARG}), 
            0x2A    => self.ANC(ImmediateAM{address: ARG}), 
            0x2B    => self.BIT(AbsoluteAM{address: ARG}), 
            0x2C    => self.AND(AbsoluteAM{address: ARG}), 
            0x2D    => self.ROL(AbsoluteAM{address: ARG}), 
            0x2E    => self.RLA(AbsoluteAM{address: ARG}), 
            0x2F    => self.BMI(---Relative), 
            0x30    => self.AND(IndirectIndexedAM{address: ARG}), 
            0x31    => self.STP(---Implicit), 
            0x32    => self.RLA(IndirectIndexedAM{address: ARG}), 
            0x33    => self.NOP(ZeroPageXAM{address: ARG}), 
            0x34    => self.AND(ZeroPageXAM{address: ARG}), 
            0x35    => self.ROL(ZeroPageXAM{address: ARG}), 
            0x36    => self.RLA(ZeroPageXAM{address: ARG}), 
            0x37    => self.SEC(ImmediateAM{address: ARG}), 
            0x38    => self.AND(AbsoluteYAM{address: ARG}), 
            0x39    => self.NOP(ImmediateAM{address: ARG}), 
            0x3A    => self.RLA(AbsoluteYAM{address: ARG}), 
            0x3B    => self.NOP(AbsoluteXAM{address: ARG}), 
            0x3C    => self.AND(AbsoluteXAM{address: ARG}), 
            0x3D    => self.ROL(AbsoluteXAM{address: ARG}), 
            0x3E    => self.RLA(AbsoluteXAM{address: ARG}), 
            0x3F    => self.RTI(AbsoluteAM{address: ARG}), 
            0x40    => self.EOR(IndexedIndirectAM{address: ARG}), 
            0x41    => self.STP(---Implicit), 
            0x42    => self.SRE(IndexedIndirectAM{address: ARG}), 
            0x43    => self.NOP(ZeroPageAM{address: ARG}), 
            0x44    => self.EOR(ZeroPageAM{address: ARG}), 
            0x45    => self.LSR(ZeroPageAM{address: ARG}), 
            0x46    => self.SRE(ZeroPageAM{address: ARG}), 
            0x47    => self.PHA(ZeroPageAM{address: ARG}), 
            0x48    => self.EOR(ImmediateAM{address: ARG}), 
            0x49    => self.LSR(ImmediateAM{address: ARG}), 
            0x4A    => self.ALR(ImmediateAM{address: ARG}), 
            0x4B    => self.JMP(AbsoluteAM{address: ARG}), 
            0x4C    => self.EOR(AbsoluteAM{address: ARG}), 
            0x4D    => self.LSR(AbsoluteAM{address: ARG}), 
            0x4E    => self.SRE(AbsoluteAM{address: ARG}), 
            0x4F    => self.BVC(---Relative), 
            0x50    => self.EOR(IndirectIndexedAM{address: ARG}), 
            0x51    => self.STP(---Implicit), 
            0x52    => self.SRE(IndirectIndexedAM{address: ARG} ),
            0x53    => self.NOP(ZeroPageXAM{address: ARG} ),
            0x54    => self.EOR(ZeroPageXAM{address: ARG} ),
            0x55    => self.LSR(ZeroPageXAM{address: ARG} ),
            0x56    => self.SRE(ZeroPageXAM{address: ARG} ),
            0x57    => self.CLI(ImmediateAM{address: ARG} ),
            0x58    => self.EOR(AbsoluteYAM{address: ARG} ),
            0x59    => self.NOP(ImmediateAM{address: ARG} ),
            0x5A    => self.SRE(AbsoluteYAM{address: ARG} ),
            0x5B    => self.NOP(AbsoluteXAM{address: ARG} ),
            0x5C    => self.EOR(AbsoluteXAM{address: ARG} ),
            0x5D    => self.LSR(AbsoluteXAM{address: ARG} ),
            0x5E    => self.SRE(AbsoluteXAM{address: ARG} ),
            0x5F    => self.RTS(AbsoluteAM{address: ARG} ),
            0x60    => self.ADC(IndexedIndirectAM{address: ARG} ),
            0x61    => self.STP(---Implicit ),
            0x62    => self.RRA(IndexedIndirectAM{address: ARG} ),
            0x63    => self.NOP(ZeroPageAM{address: ARG} ),
            0x64    => self.ADC(ZeroPageAM{address: ARG} ),
            0x65    => self.ROR(ZeroPageAM{address: ARG} ),
            0x66    => self.RRA(ZeroPageAM{address: ARG} ),
            0x67    => self.PLA(ZeroPageXAM{address: ARG} ),
            0x68    => self.ADC(ImmediateAM{address: ARG} ),
            0x69    => self.ROR(ImmediateAM{address: ARG} ),
            0x6A    => self.ARR(ImmediateAM{address: ARG} ),
            0x6B    => self.JMP(---Indirect ),
            0x6C    => self.ADC(AbsoluteAM{address: ARG} ),
            0x6D    => self.ROR(AbsoluteAM{address: ARG} ),
            0x6E    => self.RRA(AbsoluteAM{address: ARG} ),
            0x6F    => self.BVS(---Relative ),
            0x70    => self.ADC(IndirectIndexedAM{address: ARG} ),
            0x71    => self.STP(---Implicit ),
            0x72    => self.RRA(IndirectIndexedAM{address: ARG} ),
            0x73    => self.NOP(ZeroPageXAM{address: ARG} ),
            0x74    => self.ADC(ZeroPageXAM{address: ARG} ),
            0x75    => self.ROR(ZeroPageXAM{address: ARG} ),
            0x76    => self.RRA(ZeroPageXAM{address: ARG} ),
            0x77    => self.SEI(ImmediateAM{address: ARG} ),
            0x78    => self.ADC(AbsoluteYAM{address: ARG} ),
            0x79    => self.NOP(ImmediateAM{address: ARG} ),
            0x7A    => self.RRA(AbsoluteYAM{address: ARG} ),
            0x7B    => self.NOP(AbsoluteXAM{address: ARG} ),
            0x7C    => self.ADC(AbsoluteXAM{address: ARG} ),
            0x7D    => self.ROR(AbsoluteXAM{address: ARG} ),
            0x7E    => self.RRA(AbsoluteXAM{address: ARG} ),
            0x7F    => self.NOP(ImmediateAM{address: ARG} ),
            0x7G    => self.STA(IndexedIndirectAM{address: ARG} ),
            0x80    => self.NOP(ImmediateAM{address: ARG} ),
            0x81    => self.SAX(IndexedIndirectAM{address: ARG} ),
            0x82    => self.STY(ZeroPageAM{address: ARG} ),
            0x83    => self.STA(ZeroPageAM{address: ARG} ),
            0x84    => self.STX(ZeroPageAM{address: ARG} ),
            0x85    => self.SAX(ZeroPageAM{address: ARG} ),
            0x86    => self.DEY(ImmediateAM{address: ARG} ),
            0x87    => self.NOP(ImmediateAM{address: ARG} ),
            0x88    => self.TXA(ImmediateAM{address: ARG} ),
            0x89    => self.XAA(ImmediateAM{address: ARG} ),
            0x8A    => self.STY(AbsoluteAM{address: ARG} ),
            0x8B    => self.STA(AbsoluteAM{address: ARG} ),
            0x8C    => self.STX(AbsoluteAM{address: ARG} ),
            0x8D    => self.SAX(AbsoluteAM{address: ARG} ),
            0x8E    => self.BCC(---Relative ),
            0x8F    => self.STA(IndirectIndexedAM{address: ARG} ),
            0x90    => self.STP(---Implicit ),
            0x91    => self.AHX(IndirectIndexedAM{address: ARG} ),
            0x92    => self.STY(ZeroPageXAM{address: ARG} ),
            0x93    => self.STA(ZeroPageXAM{address: ARG} ),
            0x94    => self.STX(ZeroPageYAM{address: ARG} ),
            0x95    => self.SAX(ZeroPageYAM{address: ARG} ),
            0x96    => self.TYA(ImmediateAM{address: ARG} ),
            0x97    => self.STA(AbsoluteYAM{address: ARG} ),
            0x98    => self.TXS(ImmediateAM{address: ARG} ),
            0x99    => self.TAS(AbsoluteYAM{address: ARG} ),
            0x9A    => self.SHY(AbsoluteXAM{address: ARG} ),
            0x9B    => self.STA(AbsoluteXAM{address: ARG} ),
            0x9C    => self.SHX(AbsoluteYAM{address: ARG} ),
            0x9D    => self.AHX(AbsoluteYAM{address: ARG} ),
            0x9E    => self.LDY(ImmediateAM{address: ARG} ),
            0x9F    => self.LDA(IndexedIndirectAM{address: ARG} ),
            0xA0    => self.LDX(ImmediateAM{address: ARG} ),
            0xA1    => self.LAX(IndexedIndirectAM{address: ARG} ),
            0xA2    => self.LDY(ZeroPageAM{address: ARG} ),
            0xA3    => self.LDA(ZeroPageAM{address: ARG} ),
            0xA4    => self.LDX(ZeroPageAM{address: ARG} ),
            0xA5    => self.LAX(ZeroPageAM{address: ARG} ),
            0xA6    => self.TAY(ImmediateAM{address: ARG} ),
            0xA7    => self.LDA(ImmediateAM{address: ARG} ),
            0xA8    => self.TAX(ImmediateAM{address: ARG} ),
            0xA9    => self.LAX(ImmediateAM{address: ARG} ),
            0xAA    => self.LDY(AbsoluteAM{address: ARG} ),
            0xAB    => self.LDA(AbsoluteAM{address: ARG} ),
            0xAC    => self.LDX(AbsoluteAM{address: ARG} ),
            0xAD    => self.LAX(AbsoluteAM{address: ARG} ),
            0xAE    => self.BCS(---Relative ),
            0xAF    => self.LDA(IndirectIndexedAM{address: ARG} ),
            0xB0    => self.STP(---Implicit ),
            0xB1    => self.LAX(IndirectIndexedAM{address: ARG} ),
            0xB2    => self.LDY(ZeroPageXAM{address: ARG} ),
            0xB3    => self.LDA(ZeroPageXAM{address: ARG} ),
            0xB4    => self.LDX(ZeroPageYAM{address: ARG} ),
            0xB5    => self.LAX(ZeroPageYAM{address: ARG} ),
            0xB6    => self.CLV(ImmediateAM{address: ARG} ),
            0xB7    => self.LDA(AbsoluteYAM{address: ARG} ),
            0xB8    => self.TSX(ImmediateAM{address: ARG} ),
            0xB9    => self.LAS(AbsoluteYAM{address: ARG} ),
            0xBA    => self.LDY(AbsoluteXAM{address: ARG} ),
            0xBB    => self.LDA(AbsoluteXAM{address: ARG} ),
            0xBC    => self.LDX(AbsoluteYAM{address: ARG} ),
            0xBD    => self.LAX(AbsoluteYAM{address: ARG} ),
            0xBE    => self.CPY(ImmediateAM{address: ARG} ),
            0xBF    => self.CMP(IndexedIndirectAM{address: ARG} ),
            0xC0    => self.NOP(ImmediateAM{address: ARG} ),
            0xC1    => self.DCP(IndexedIndirectAM{address: ARG} ),
            0xC2    => self.CPY(ZeroPageAM{address: ARG} ),
            0xC3    => self.CMP(ZeroPageAM{address: ARG} ),
            0xC4    => self.DEC(ZeroPageAM{address: ARG} ),
            0xC5    => self.DCP(ZeroPageAM{address: ARG} ),
            0xC6    => self.INY(ImmediateAM{address: ARG} ),
            0xC7    => self.CMP(ImmediateAM{address: ARG} ),
            0xC8    => self.DEX(ImmediateAM{address: ARG} ),
            0xC9    => self.AXS(ImmediateAM{address: ARG} ),
            0xCA    => self.CPY(AbsoluteAM{address: ARG} ),
            0xCB    => self.CMP(AbsoluteAM{address: ARG} ),
            0xCC    => self.DEC(AbsoluteAM{address: ARG} ),
            0xCD    => self.DCP(AbsoluteAM{address: ARG} ),
            0xCE    => self.BNE(---Relative ),
            0xCF    => self.CMP(IndirectIndexedAM{address: ARG} ),
            0xD0    => self.STP(---Implicit ),
            0xD1    => self.DCP(IndirectIndexedAM{address: ARG} ),
            0xD2    => self.NOP(ZeroPageXAM{address: ARG} ),
            0xD3    => self.CMP(ZeroPageXAM{address: ARG} ),
            0xD4    => self.DEC(ZeroPageXAM{address: ARG} ),
            0xD5    => self.DCP(ZeroPageXAM{address: ARG} ),
            0xD6    => self.CLD(ImmediateAM{address: ARG} ),
            0xD7    => self.CMP(AbsoluteYAM{address: ARG} ),
            0xD8    => self.NOP(ImmediateAM{address: ARG} ),
            0xD9    => self.DCP(AbsoluteYAM{address: ARG} ),
            0xDA    => self.NOP(AbsoluteXAM{address: ARG} ),
            0xDB    => self.CMP(AbsoluteXAM{address: ARG} ),
            0xDC    => self.DEC(AbsoluteXAM{address: ARG} ),
            0xDD    => self.DCP(AbsoluteXAM{address: ARG} ),
            0xDE    => self.CPX(ImmediateAM{address: ARG} ),
            0xDF    => self.SBC(IndexedIndirectAM{address: ARG} ),
            0xE0    => self.NOP(ImmediateAM{address: ARG} ),
            0xE1    => self.ISC(IndexedIndirectAM{address: ARG} ),
            0xE2    => self.CPX(ZeroPageAM{address: ARG} ),
            0xE3    => self.SBC(ZeroPageAM{address: ARG} ),
            0xE4    => self.INC(ZeroPageAM{address: ARG} ),
            0xE5    => self.ISC(ZeroPageAM{address: ARG} ),
            0xE6    => self.INX(ImmediateAM{address: ARG} ),
            0xE7    => self.SBC(ImmediateAM{address: ARG} ),
            0xE8    => self.NOP(ImmediateAM{address: ARG} ),
            0xE9    => self.SBC(ImmediateAM{address: ARG} ),
            0xEA    => self.CPX(AbsoluteAM{address: ARG} ),
            0xEB    => self.SBC(AbsoluteAM{address: ARG} ),
            0xEC    => self.INC(AbsoluteAM{address: ARG} ),
            0xED    => self.ISC(AbsoluteAM{address: ARG} ),
            0xEE    => self.BEQ(---Relative ),
            0xEF    => self.SBC(IndirectIndexedAM{address: ARG} ),
            0xF0    => self.STP(---Implicit ),
            0xF1    => self.ISC(IndirectIndexedAM{address: ARG} ),
            0xF2    => self.NOP(ZeroPageXAM{address: ARG} ),
            0xF3    => self.SBC(ZeroPageXAM{address: ARG} ),
            0xF4    => self.INC(ZeroPageXAM{address: ARG} ),
            0xF5    => self.ISC(ZeroPageXAM{address: ARG} ),
            0xF6    => self.SED(ImmediateAM{address: ARG} ),
            0xF7    => self.SBC(AbsoluteYAM{address: ARG} ),
            0xF8    => self.NOP(ImmediateAM{address: ARG} ),
            0xF9    => self.ISC(AbsoluteYAM{address: ARG} ),
            0xFA    => self.NOP(AbsoluteXAM{address: ARG} ),
            0xFB    => self.SBC(AbsoluteXAM{address: ARG} ),
            0xFC    => self.INC(AbsoluteXAM{address: ARG} ),
            0xFD    => self.ISC(AbsoluteXAM{address: ARG} ),
            _       =>  panic!("OPCODE not implemented yet."),
        }

        let speed = 0 + OP_SPEEDS[OP as usize],
    }
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

pub struct Instructions {
    pub names:     [&'static str;256],
    pub sizes:     [u8;256],
    pub modes:     [u8;256],
    pub speeds:    [u8;256],
    pub paging:    [u8;256],
}

static OP_SPEEDS: [u8;256] =
                [7, 6, 0, 8, 3, 3, 5, 5, 0, 2, 0, 2, 4, 4, 6, 6, 2, 5, 0, 8, 4, 4, 6, 6, 0, 4, 0, 7, 4, 4, 7, 7, 6, 6, 0, 8, 3, 3, 5, 5, 0, 2, 0, 2, 4, 4, 6, 6, 2, 5, 0, 8, 4, 4, 6, 6, 0, 4, 0, 7, 4, 4, 7, 7, 0, 6, 0, 8, 3, 3, 5, 5, 0, 2, 0, 2, 3, 4, 6, 6, 2, 5, 0, 8, 4, 4, 6, 6, 0, 4, 0, 7, 4, 4, 7, 7, 0, 6, 0, 8, 3, 3, 5, 5, 0, 2, 0, 2, 5, 4, 6, 6, 2, 5, 0, 8, 4, 4, 6, 6, 0, 4, 0, 7, 4, 4, 7, 7, 2, 6, 2, 6, 3, 3, 3, 3, 0, 2, 0, 2, 4, 4, 4, 4, 2, 6, 0, 6, 4, 4, 4, 4, 0, 5, 0, 5, 5, 5, 5, 5, 2, 6, 2, 6, 3, 3, 3, 3, 0, 2, 0, 2, 4, 4, 4, 4, 2, 5, 0, 5, 4, 4, 4, 4, 0, 4, 0, 4, 4, 4, 4, 4, 2, 6, 2, 8, 3, 3, 5, 5, 0, 2, 0, 2, 4, 4, 6, 6, 2, 5, 0, 8, 4, 4, 6, 6, 0, 4, 0, 7, 4, 4, 7, 7, 2, 6, 2, 8, 3, 3, 5, 5, 0, 2, 0, 2, 4, 4, 6, 6, 2, 5, 0, 8, 4, 4, 6, 6, 0, 4, 0, 7, 4, 4, 7, 7]; 

static OP_PAGING: [u8; 256] = 
                [7, 6, 0, 8, 3, 3, 5, 5, 0, 2, 0, 2, 4, 4, 6, 6, 3, 6, 0, 8, 4, 4, 6, 6, 0, 5, 0, 7, 5, 5, 7, 7, 6, 6, 0, 8, 3, 3, 5, 5, 0, 2, 0, 2, 4, 4, 6, 6, 3, 6, 0, 8, 4, 4, 6, 6, 0, 5, 0, 7, 5, 5, 7, 7, 0, 6, 0, 8, 3, 3, 5, 5, 0, 2, 0, 2, 3, 4, 6, 6, 4, 6, 0, 8, 4, 4, 6, 6, 0, 5, 0, 7, 5, 5, 7, 7, 0, 6, 0, 8, 3, 3, 5, 5, 0, 2, 0, 2, 5, 4, 6, 6, 3, 6, 0, 8, 4, 4, 6, 6, 0, 5, 0, 7, 5, 5, 7, 7, 2, 6, 2, 6, 3, 3, 3, 3, 0, 2, 0, 2, 4, 4, 4, 4, 3, 6, 0, 6, 4, 4, 4, 4, 0, 5, 0, 5, 5, 5, 5, 5, 2, 6, 2, 6, 3, 3, 3, 3, 0, 2, 0, 2, 4, 4, 4, 4, 3, 6, 0, 6, 4, 4, 4, 4, 0, 5, 0, 5, 5, 5, 5, 5, 2, 6, 2, 8, 3, 3, 5, 5, 0, 2, 0, 2, 4, 4, 6, 6, 3, 6, 0, 8, 4, 4, 6, 6, 0, 5, 0, 7, 5, 5, 7, 7, 2, 6, 2, 8, 3, 3, 5, 5, 0, 2, 0, 2, 4, 4, 6, 6, 3, 6, 0, 8, 4, 4, 6, 6, 0, 5, 0, 7, 5, 5, 7, 7];
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
