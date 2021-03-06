 /*  Unit test module of the MOS 6052 CPU (cpu.rs).
 *  Author: Spalynx
 *  Init: 6/17/18
 */
use crate::core::cpu::*;
//use core::memory::*;

#[cfg(test)]
pub mod cpu_test {
    //Tests could be greatly improved with
    // [before_each]/[assemblyintialize]... Maybe one day!
    use super::*;

    //~~~CPU~META~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    #[test]
    fn test_cpu_init(){
        let test_cpu = super::CPU::new_empty();
        
        //Testing each struct field initialization.
        assert_eq!(test_cpu.cycles,   0);
        assert_eq!(test_cpu.pc,       0);
        assert_eq!(test_cpu.sp,       0xFF);
        assert_eq!(test_cpu.x,        0);
        assert_eq!(test_cpu.y,        0);
        assert_eq!(test_cpu.status,   34);
        assert_eq!(test_cpu.interrupt,0);
        assert_eq!(test_cpu.stall,    0);
    }

    #[test]
    fn test_status_each(){
        let mut cpu = super::CPU::new_empty();

        //Array of status codes.

        let status = ["C", "Z", "I", "D", "B", "S", "V", "N",
                      "c", "z", "i", "d", "b", "s", "v", "n"];

        //Loops through array of status codes, and sets/resets each flag.
        for f in status.iter() {
            //Setting each flag.
            cpu.set_status_old(f, true);
            assert_eq!(cpu.get_status(f), true);

            //Resetting each flag.
            cpu.set_status_old(f, false);
            assert_eq!(cpu.get_status(f), false);
        }

        for f in 0..8 {
            //Setting each flag.
            //println!("{} : {}", f, status[f as usize]);
            cpu.set_status(f, true);
            assert_eq!(cpu.get_status(status[f as usize]), true);

            //Resetting each flag.
            cpu.set_status(f, false);
            assert_eq!(cpu.get_status(status[f as usize]), false);
        }

    }

    #[test]
    fn test_status_settingtwice(){
        //Making sure that nothing is changed when setting something twice.
        // When I would set a false flag to false, it would be set to true.
        let mut cpu = super::CPU::new_empty();

        //False
        cpu.set_status(2, false);
        cpu.set_status(2, false);
        assert_eq!(cpu.get_status("I"), false);
        
        //True
        cpu.set_status(6, true);
        cpu.set_status(6, true);
        assert_eq!(cpu.get_status("V"), true);
    }

    #[test]
    fn test_new_memory(){
        let mut cpu = super::CPU::new_empty();
        let mem = super::MEM::new_empty();

        cpu.new_memory(mem);

        //Testing access/mutability.
        cpu.memory.set_zp(0x1A, 10);
        assert_eq!(cpu.memory.get_zp(0x1A), 10);
        cpu.memory.set_zp(0x0, 254);
        assert_eq!(cpu.memory.get_zp(0x0), 254);
        cpu.memory.set(0x800-1, 111);
        assert_eq!(cpu.memory.get(0x800-1), 111);

        //Further mem testing should be done in it's module.
    }

    #[test]
    fn test_AM(){
        //All of these are being tested in one large module because
        // I'm too lazy to give it's own module, and I don't expect
        // much trouble from these.

        let mut cpu = super::CPU::new_empty();
        // AccumulatorAM
        cpu.a = 10;
        assert_eq!(AccumulatorAM.load(&mut cpu), 10, "Accumulator load");
        AccumulatorAM.save(&mut cpu, 11);
        assert_eq!(AccumulatorAM.load(&mut cpu), 11, "Accumulator save");

        // ImmediateAM
        assert_eq!(ImmediateAM{address: 12}.load(&mut cpu),
                   12, "Immediate load");
        
        // AbsoluteAM
        AbsoluteAM{address: 0x7FF}.save(&mut cpu, 13);
        assert_eq!(cpu.memory.get(0x7FF), 13,  "Absolute save");
        assert_eq!(cpu.memory.get(0x7FF),
                   AbsoluteAM{address: 0x7FF}.load(&mut cpu), "Absolute load");
        
            
        //Avoiding a reliance on LDX/LDY. 
        //Also assuming that absolute is working.

        // AbsoluteXAM
        cpu.x = 10;
        AbsoluteXAM{address: 255}.save(&mut cpu, 14);
        assert_eq!(AbsoluteXAM{address: 255}.load(&mut cpu),
                   AbsoluteAM {address: 265}.load(&mut cpu), "Absolute X save/load");

        // AbsoluteYAM
        cpu.y = 25;
        AbsoluteYAM{address: 800}.save(&mut cpu, 15);
        assert_eq!(AbsoluteYAM{address: 800}.load(&mut cpu),
                   AbsoluteAM {address: 825}.load(&mut cpu), "Absolute Y save/load");

        // ZeroPageAM
        ZeroPageAM{address: 125}.save(&mut cpu, 16);
        assert_eq!(ZeroPageAM{address: 125}.load(&mut cpu),
                   16, "Zero page");

        // ZeroPageXAM
        cpu.x = 5;
        ZeroPageXAM{address: 125}.save(&mut cpu, 17);
        assert_eq!(ZeroPageAM{address: 130}.load(&mut cpu),
                   ZeroPageXAM{address: 125}.load(&mut cpu), "Zero page + X");

        // ZeroPageYAM
        cpu.y = 5;
        ZeroPageYAM{address: 125}.save(&mut cpu, 18);
        assert_eq!(ZeroPageAM{address: 130}.load(&mut cpu),
                   ZeroPageYAM{address: 125}.load(&mut cpu), "Zero page + Y");

        // IndexedIndirectAM
        cpu.x = 10;
        cpu.memory.set(165, 01);
        cpu.memory.set(166, 00);
        IndexedIndirectAM{address: 155}.save(&mut cpu, 19);
        assert_eq!(cpu.memory.get(0001), 19, "IndexIndir save");
        assert_eq!(IndexedIndirectAM{address: 155}.load(&mut cpu),
                   19, "IndexIndir load");

        // IndirectIndexedAM
        cpu.y = 20;
        cpu.memory.set(245, 02);
        cpu.memory.set(246, 00);
        IndirectIndexedAM{address: 245}.save(&mut cpu, 20);
        assert_eq!(cpu.memory.get(0022), 20, "IndirIndex save");
        assert_eq!(IndirectIndexedAM{address: 245}.load(&mut cpu),
                   20, "IndirIndex load");
        
    }


    #[test]
    pub fn test_stack_overflow(){
        assert!(false);
    }

    #[test]
    pub fn test_stack_underflow(){
        assert!(false);
    }

    //~~~CPU~OPCODES~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    #[test]
    fn testOP_flags(){
        //Testing CLC, CLD, CLI, CLV, SEC, SED, SEI.
        // Reliant on test_status() being correct.
        //Yes, I understand that I should split this up,
        // but cargo test doesn't really avoid bpcode atm.

        let mut cpu = super::CPU::new_empty();

        //~! CLEARS

        //Carry clear.
        cpu.set_status(0, true);
        cpu.CLC();
        assert_eq!(cpu.get_status("C"), false);
        //Decimal clear.
        cpu.set_status(3, true);
        cpu.CLD();
        assert_eq!(cpu.get_status("D"), false);
        //Interrupt carry.
        cpu.set_status(2, true);
        cpu.CLI();
        assert_eq!(cpu.get_status("I"), false);

        //~! SETS
        //Carry set.
        cpu.set_status(0, false);
        cpu.SEC();
        assert_eq!(cpu.get_status("C"), true);
        //Decimal set.
        cpu.set_status(3, false);
        cpu.SED();
        assert_eq!(cpu.get_status("D"), true);
        //Interrupt set.
        cpu.set_status(2, false);
        cpu.SEI();
        assert_eq!(cpu.get_status("I"), true);
        

    }

    #[test]
    fn testOP_LDA(){
        let mut cpu = super::CPU::new_empty();

        cpu.memory.set(0x755, 234); // Random position non zero page.
        cpu.memory.set(0xAF, 123);  // Random position in zero page.

        //The rest should be mechanically similar to these 3, and tested in test_AM.
        //assert_eq!(cpu.LDA(ImmediateAM{address: 10}), cpu.a, "AM: Immediate.");
        cpu.LDA(AbsoluteAM{address:  0x755});
        assert_eq!(234, cpu.a, "AM: Absolute");
        cpu.LDA(ZeroPageAM{address:  0xAF});
        assert_eq!(123, cpu.a, "AM: Absolute");

        //Testing flag changes.
        cpu.LDA(ImmediateAM{address: 0});
        assert_eq!(cpu.get_status("Z"), true, "Testing zero.");

        cpu.LDA(ImmediateAM{address: 254});
        assert_eq!(cpu.get_status("N"), true, "Testing negative.");
    }

    #[test]
    fn testOP_LDX(){
        let mut cpu = super::CPU::new_empty();
        
        //Test actual load of x.
        cpu.memory.set_zp(0xFF, 12);
        cpu.LDX(ZeroPageAM{address: 0xFF});
        assert_eq!(cpu.x, 12);

        //Make sure flags are being set.
        cpu.LDX(ImmediateAM{address: 0});   //Zero is set.
        assert!(cpu.get_status("Z"));
        cpu.LDX(ImmediateAM{address: 245}); //Bit 7 is set.
        assert!(cpu.get_status("N"));
    }

    #[test]
    fn testOP_LDY(){
        let mut cpu = super::CPU::new_empty();

        //Test actual load of x.
        cpu.memory.set_zp(0x0, 13);
        cpu.LDY(ZeroPageAM{address: 0x0});
        assert_eq!(cpu.y, 13);

        //Make sure flags are being set.
        cpu.LDY(ImmediateAM{address: 0});   //Zero is set.
        assert!(cpu.get_status("Z"));
        cpu.LDY(ImmediateAM{address: 245}); //Bit 7 is set.
        assert!(cpu.get_status("N"));
    }

    #[test]
    fn testOP_ADC_carry(){
        let mut cpu = super::CPU::new_empty();

        cpu.a = 255;
        cpu.ADC(ImmediateAM{address: 255});

        assert_eq!(cpu.a, 254);
        assert_eq!(cpu.get_status("C"), true);
    }

    #[test]
    fn testOP_ADC_zero(){
        let mut cpu = super::CPU::new_empty();

        cpu.a = 0;
        cpu.ADC(ImmediateAM{address: 0});

        assert_eq!(cpu.a, 0);
        assert_eq!(cpu.get_status("Z"), true);
    }

    #[test]
    fn testOP_ADC_overflow(){
        let mut cpu = super::CPU::new_empty();

        cpu.a = 126;
        cpu.ADC(ImmediateAM{address: 5});

        //let d = 126;
        //let e = 5;
        //let f = d+e;
        //println!("In carry: {}",((d^f)&(e^f)&(128))==128);
        assert_eq!(cpu.a, 131);
        assert_eq!(cpu.get_status("V"), true);
    }

    #[test]
    fn testOP_ADC_memory_with_lda(){
        let mut cpu = super::CPU::new_empty();

        
        cpu.memory.set(0x755, 237); 
        cpu.LDA(AbsoluteAM{address:  0x755}); //Filling register a with first value.
        cpu.ADC(ImmediateAM{address: 255});   //Adding second value to A.

        
        assert_eq!(cpu.a, 236);               //Assert that the value has changed correctly.
        assert_eq!(cpu.get_status("C"), true);//Assert that flags have be suitably modified.
        assert_eq!(cpu.get_status("N"), true);
    }

    #[test]
    fn testOP_ADC_signed (){
        let mut test_cpu = super::CPU::new_empty();

        test_cpu.LDA(ImmediateAM{address: 0b01111111u8}); //+127
        test_cpu.ADC(ImmediateAM{address: 0b00000010u8}); //+2

        //Tests of this addition:
        assert_eq!(test_cpu.a, 0b10000001u8); // = -127
        assert_eq!(test_cpu.get_status("V"), true, "ADC-Signed Overflow Unset."); 
        assert_eq!(test_cpu.get_status("N"), true, "ADC-Signed Negative Unset."); 
    }
    #[test]
    fn testOP_ADC_decimal(){
        /// Ultimately, it looks like the NES disabled BCD arith "possibly
        ///  due to patent concerns" (fogleman @ Medium).
        /// But I still want the functionality, because of the obscure few
        ///  famicon games that I've heard rumors about.
        /// Implementation of a switch for manual inclusion of this feature
        ///  will be added to be turned on by the overarching NES system.
        /// Since I don't have such an overarching module fully planned,
        /// This is going to be turfed to an issue to be solved post v1.

        assert!(true);
        //let mut test_cpu = super::CPU::new_empty();

        ////Set to decimal mode:
        //test_cpu.SED();

        ////Add two decimal mode numbers:
        //cpu.LDA(ImmediateAM{address: 0b11111111u8});
        //test_cpu.ADC(ImmediateAM{address: 0b00000010u8)};

        ////Testing this addition:
        //assert_eq!(test_cpu.a, 0b10101010u8, "ADC-Decimal Addition");
    }
    #[test]
    fn testOP_SBC() {
        let mut cpu = super::CPU::new_empty();

        cpu.SEC(); //C is set in subtraction.
        cpu.memory.set(0x755, 237); 
        cpu.LDA(AbsoluteAM{address:  0x755}); //Filling register a with first value.
        cpu.SBC(ImmediateAM{address: 20});   //Subtracting second value to A.

        assert_eq!(cpu.a, 217);
    }
    #[test]
    fn testOP_SBC_signed() {
        let mut cpu = super::CPU::new_empty();

        cpu.SEC(); //C is set in subtraction.
        cpu.LDA(ImmediateAM{address: 0b00000001u8}); //+1
        cpu.SBC(ImmediateAM{address: 0b00000010u8}); //+2

        //Tests of this addition:
        assert_eq!(cpu.a, 0b11111111u8); // = -1 (254)
        assert_eq!(cpu.get_status("C"), false, "SBC-Signed-Borrow Required"); 
    }
    #[test]
    fn testOP_SBC_decimal() {
        ///Same hold as testOP_ADC_decimal()...
        /// Task state on WAIT.
        assert!(true);
    }
    #[test]
    fn testOP_AND() {
        let mut cpu = super::CPU::new_empty();

        cpu.LDA(ImmediateAM{address: 0b11000001u8});
        cpu.AND(ImmediateAM{address: 0b10111111u8}); //Resetting the 6th bit with AND.

        assert_eq!(cpu.a, 0b10000001u8);
        //Not going to test both flags for each binary op, but It is at least checking once.
        assert_eq!(cpu.get_status("N"), true); 
    }
    #[test]
    fn testOP_ORA() {
        let mut cpu = super::CPU::new_empty();

        cpu.LDA(ImmediateAM{address: 0b01010101u8});
        cpu.ORA(ImmediateAM{address: 0b10001101u8}); //Random OR operation, really.

        assert_eq!(cpu.a, 0b11011101u8);
    }

    #[test]
    fn testOP_EOR() {
        let mut cpu = super::CPU::new_empty();

        cpu.LDA(ImmediateAM{address: 0b11111111u8});
        cpu.EOR(ImmediateAM{address: 0b11111111u8}); //Exclusive OR, should return nothing.

        assert_eq!(cpu.a, 0);
        //Not going to test both flags for each binary op, but It is at least checking once.
        assert_eq!(cpu.get_status("Z"), true); 
        
    }

    #[test]
    fn testOP_ASL() {
        let mut cpu = super::CPU::new_empty();

        cpu.LDA(ImmediateAM{address: 0b11111111u8});
        cpu.ASL(AccumulatorAM); 

        assert_eq!(cpu.a, 0b11111110u8);
        assert_eq!(cpu.get_status("C"), true);
        assert_eq!(cpu.get_status("N"), true);
    }

    #[test]
    fn testOP_LSR() {
        let mut cpu = super::CPU::new_empty();

        cpu.LDA(ImmediateAM{address: 0b11111111u8});
        cpu.LSR(AccumulatorAM); 

        assert_eq!(cpu.a, 0b01111111u8);
        assert_eq!(cpu.get_status("C"), true);
        assert_eq!(cpu.get_status("N"), false); //"N" should always reset
    }

    #[test]
    fn testOP_ROL() {
        let mut cpu = super::CPU::new_empty();

        cpu.SEC();
        cpu.LDA(ImmediateAM{address: 0b01111110u8});
        cpu.ROL(AccumulatorAM); 

        assert_eq!(cpu.a, 0b11111101u8);
        assert_eq!(cpu.get_status("C"), false);
        assert_eq!(cpu.get_status("N"), true);
    }

    #[test]
    fn testOP_ROR() {
        let mut cpu = super::CPU::new_empty();

        cpu.SEC();
        cpu.LDA(ImmediateAM{address: 0b01111110u8});
        cpu.ROR(AccumulatorAM); 

        assert_eq!(cpu.a, 0b10111111u8);
        assert_eq!(cpu.get_status("C"), false);
        assert_eq!(cpu.get_status("N"), true);
    }

    #[test]
    fn testOP_DEC() {
        let mut cpu = super::CPU::new_empty();

        //Testing Z flag in relation to DEC.
        cpu.memory.set(0x755, 1);

        cpu.DEC(AbsoluteAM{address: 0x755});
        assert_eq!(cpu.memory.get(0x755), 0);
        assert_eq!(cpu.get_status("Z"), true);
        
        //Testing N flag in relation to DEC.
        // Setting the value to 128 (and N flag to true), and seeing if
        //  the N flag is modified on decrement.
        cpu.set_status(7, true);
        ZeroPageAM{address: 0xFF}.save(&mut cpu, 128);

        cpu.DEC(ZeroPageAM{address: 0xFF});

        assert_eq!(ZeroPageAM{address: 0xFF}.load(&mut cpu), 127);
        assert_eq!(cpu.get_status("N"), false);
    }
    #[test]
    fn testOP_DEX() {
        let mut cpu = super::CPU::new_empty();

        //Testing Z flag in relation to DEX, and proper decrement.
        cpu.x = 1;
        cpu.DEX();
        assert_eq!(cpu.get_status("Z"), true);
        assert_eq!(cpu.x, 0);
       
        //Same for N flag in relation to DEX.
        cpu.x = 128;
        cpu.set_status(7, true);
        cpu.DEX();
        assert_eq!(cpu.get_status("N"), false);
        assert_eq!(cpu.x, 127);
    }
    #[test]
    fn testOP_DEY() {
        let mut cpu = super::CPU::new_empty();

        //Testing Z flag in relation to DEY, and proper decrement.
        cpu.y = 1;
        cpu.DEY();
        assert_eq!(cpu.get_status("Z"), true);
        assert_eq!(cpu.y, 0);
       
        //Same for N flag in relation to DEY.
        cpu.y = 128;
        cpu.set_status(7, true);
        cpu.DEY();
        assert_eq!(cpu.get_status("N"), false);
        assert_eq!(cpu.y, 127);
    }
    #[test]
    fn testOP_INC() {
        let mut cpu = super::CPU::new_empty();

        //Setting memory value for increment.
        cpu.memory.set(0x755, 129);

        //Increment value
        cpu.INC(AbsoluteAM{address: 0x755});
        //If bit 7 is on as a result of the increment, 'N' = true.
        assert_eq!(cpu.memory.get(0x755), 130);
        assert_eq!(cpu.get_status("N"), true);
    }
    #[test]
    fn testOP_INX() {
        let mut cpu = super::CPU::new_empty();

        //Setting X to 255 for INX: Tests overflow add, 'N', and 'Z'.
        cpu.x = 255;
        cpu.set_status(7, true);
        cpu.set_status(1, false);
        cpu.INX();

        //This increment should make X = 0, 'N' = false, 'Z' = true.
        assert_eq!(cpu.get_status("Z"), true);
        assert_eq!(cpu.get_status("N"), false);
        assert_eq!(cpu.x, 0);
    }
    #[test]
    fn testOP_INY() {
        //NOTE: A carbon copy of testOP_INX.
        //TODO: BP tests might not be useful. Possibly prune dupes?
        let mut cpu = super::CPU::new_empty();

        //Setting Y to 255 for INY: Tests overflow add, 'N', and 'Z'.
        cpu.y = 255;
        cpu.set_status(7, true);
        cpu.set_status(1, false);
        cpu.INY();

        //This increment should make Y = 0, 'N' = false, 'Z' = true.
        assert_eq!(cpu.get_status("Z"), true);
        assert_eq!(cpu.get_status("N"), false);
        assert_eq!(cpu.y, 0);
    }
    #[test]
    fn testOP_STA() {
        //Inane, but we're just gonna test a basic store.
        let mut cpu = super::CPU::new_empty();

        cpu.a = 255;
        cpu.STA(AbsoluteAM{address: 0x741});
        assert_eq!(cpu.memory.get(0x741), 255, "Did store happen?");
    }
    #[test]
    fn testOP_STX() {
        //Inane, but we're just gonna test a basic store.
        let mut cpu = super::CPU::new_empty();

        cpu.x = 255;
        cpu.STX(AbsoluteAM{address: 0x741});
        assert_eq!(cpu.memory.get(0x741), 255, "Did store happen?");
        
    }
    #[test]
    fn testOP_STY() {
        //Inane, but we're just gonna test a basic store.
        let mut cpu = super::CPU::new_empty();

        cpu.y = 255;
        cpu.STY(AbsoluteAM{address: 0x741});
        assert_eq!(cpu.memory.get(0x741), 255, "Did store happen?");
    }
    #[test]
    fn testOP_TXS() {
        let mut cpu = super::CPU::new_empty();

        cpu.x = 255;
        cpu.TXS();
        assert_eq!(cpu.sp, 255); //No flag change.
    }
    #[test]
    fn testOP_TSX() {
        let mut cpu = super::CPU::new_empty();

        cpu.sp = 255;
        cpu.TSX();
        assert_eq!(cpu.x, 255); //No flag change.
    }
    #[test]
    fn testOP_TXA() {
        let mut cpu = super::CPU::new_empty();

        cpu.x = 255;
        cpu.TXA();
        assert_eq!(cpu.a, 255); //Changes 'Z, N'
        assert_eq!(cpu.get_status("N"), true);
    }
    #[test]
    fn testOP_TAY() {
        let mut cpu = super::CPU::new_empty();

        cpu.a = 0;
        cpu.TAY();
        assert_eq!(cpu.y, 0); //Changes 'Z, N'
        assert_eq!(cpu.get_status("Z"), true);
    }
    #[test]
    fn testOP_TAX() {
        let mut cpu = super::CPU::new_empty();

        cpu.a = 0;
        cpu.TAX();
        assert_eq!(cpu.x, 0); //Changes 'Z, N'
        assert_eq!(cpu.get_status("Z"), true);
    }
    #[test]
    fn testOP_TYA() {
        let mut cpu = super::CPU::new_empty();

        cpu.y = 0;
        cpu.TYA();
        assert_eq!(cpu.a, 0); //Changes 'Z, N'
        assert_eq!(cpu.get_status("Z"), true);
    }

    #[test]
    pub fn testOP_BIT() {
        //A little hard to test, best way is to try to trigger each flag.
        let mut cpu = super::CPU::new_empty();

        //Fill first memory location. Sets 'N, V'.
        cpu.memory.set(0xAA, 0b11000000); 
        //Run comparison.
        cpu.BIT(AbsoluteAM{address: 0xAA});
        assert!(cpu.get_status("N"));
        assert!(cpu.get_status("V"));

        //Fill second memory location. Sets 'Z'.
        cpu.memory.set(0xAB, 0);
        cpu.a = 0b11111111;
        //Run comparison on the second one.
        cpu.BIT(AbsoluteAM{address: 0xAB});
        assert!(cpu.get_status("Z"));
    }

    #[test]
    pub fn testOP_BRK() {
        assert!(false); 
    } 
    #[test]
    pub fn testOP_RTI() {
        assert!(false); 
    }
    #[test]
    pub fn testOP_PHA() {
        let mut cpu = super::CPU::new_empty();
        cpu.a = 250;
        cpu.PHA();
        assert_eq!(cpu.stack_pop(), 250,  "A != stack top val!");
    }

    #[test]
    pub fn testOP_PLA() {
        let mut cpu = super::CPU::new_empty();
        cpu.stack_push(0xFA);
        cpu.PLA();
        assert_eq!(cpu.a, 0xFA,  "A != Popped value!");
    }

    #[test]
    pub fn testOP_PHP() {
        let mut cpu = super::CPU::new_empty();
        cpu.status = 250;
        cpu.PHP();
        assert_eq!(cpu.stack_pop(), 250,  "A != stack top val!");
    }

    #[test]
    pub fn testOP_PLP() {
        let mut cpu = super::CPU::new_empty();
        cpu.stack_push(0xFA);
        cpu.PLP();
        assert_eq!(cpu.status, 0xFA,  "A != Popped value!");
    }

    #[test]
    pub fn testOP_RTS() {
        let mut cpu = super::CPU::new_empty();
        cpu.stack_push(0b10000000);
        cpu.stack_push(0b00000001);
        cpu.RTS();
        assert_eq!(cpu.pc, (0b1000000000000001 + 1), "PC != RTS pulls!");
    }

    #[test]
    pub fn testOP_CMP() {
        let mut cpu = super::CPU::new_empty();
        cpu.memory.set(0xFF, 0xFF);
        cpu.a = 1;
        cpu.CMP(AbsoluteAM{address: 0xFF});

        assert_eq!(cpu.get_status("C"), false, "CMP 'C'");
        assert_eq!(cpu.get_status("N"), false, "CMP 'N'");
        assert_eq!(cpu.get_status("Z"), false, "CMP 'Z'");
    }
    //Another round of copies of the original.
    #[test]
    pub fn testOP_CPY() {
        let mut cpu = super::CPU::new_empty();
        cpu.memory.set(0xFF, 0x80);
        cpu.x = 0x7F;
        cpu.CPY(AbsoluteAM{address: 0xFF});

        assert_eq!(cpu.get_status("C"), false, "CPY 'C'");
        assert_eq!(cpu.get_status("N"), true , "CPY 'N'");
        assert_eq!(cpu.get_status("Z"), false, "CPY 'Z'");
    }
    #[test]
    pub fn testOP_CPX() {
        let mut cpu = super::CPU::new_empty();
        cpu.memory.set(0xFF, 0x80);
        cpu.x = 0x7F;
        cpu.CPX(AbsoluteAM{address: 0xFF});

        assert_eq!(cpu.get_status("C"), false, "CPY 'C'");
        assert_eq!(cpu.get_status("N"), true , "CPY 'N'");
        assert_eq!(cpu.get_status("Z"), false, "CPY 'Z'");
    }
    #[test]
    pub fn testOP_JSR() {
        let mut cpu = super::CPU::new_empty();
        //Since we don't have an actual FDE loop yet, going to simulate
        cpu.pc = 0x4020;
        cpu.JSR(AbsoluteAM{address: 0x4029});
        assert_eq!(cpu.pc, 0x4029, "JSR: Checking if JSR loaded PC.");
        cpu.RTS();
        assert_eq!(cpu.pc, 0x4020+3, "JSR: Checking if RTI pulled from stack.");
        // +3 because JSR accounts for the +2 bytes, and RTS adds +1 for JMP
    }

    #[test]
    pub fn testOP_JMP() {
        let mut cpu = super::CPU::new_empty();

        cpu.memory.set(0x0030, 0b00000001);
        cpu.memory.set(0x0031, 0b10000000);
        cpu.pc = 0;

        cpu.JMP(AbsoluteAM{address: 0x0030});
        assert_eq!(cpu.pc, 0b1000000000000001);

    }
    #[test]
    pub fn testOP_BCC() {
        let mut cpu = super::CPU::new_empty();

        cpu.status = 0b11111110;
        cpu.BCC(ImmediateAM{address: 20});

        assert_eq!(cpu.pc, 20);
    }
    #[test]
    pub fn testOP_BCS() {
        let mut cpu = super::CPU::new_empty();

        cpu.status = 0b00000001;
        cpu.BCS(ImmediateAM{address: 20});

        assert_eq!(cpu.pc, 20);
    }
    #[test]
    pub fn testOP_BEQ() {
        let mut cpu = super::CPU::new_empty();

        cpu.status = 0b00000010;
        cpu.BEQ(ImmediateAM{address: 20});

        assert_eq!(cpu.pc, 20);
    }
    #[test]
    pub fn testOP_BNE() {
        let mut cpu = super::CPU::new_empty();

        cpu.status = 0b11111101;
        cpu.BNE(ImmediateAM{address: 20});

        assert_eq!(cpu.pc, 20);
    }
    #[test]
    pub fn testOP_BMI() {
        let mut cpu = super::CPU::new_empty();

        cpu.status = 0b10000000;
        cpu.BMI(ImmediateAM{address: 20});

        assert_eq!(cpu.pc, 20);

    }
    #[test]
    pub fn testOP_BPL() {
        let mut cpu = super::CPU::new_empty();

        cpu.status = 0b01111111;
        cpu.BPL(ImmediateAM{address: 20});

        assert_eq!(cpu.pc, 20);
    }
    #[test]
    pub fn testOP_BVC() {
        let mut cpu = super::CPU::new_empty();

        cpu.status = 0b10111111;
        cpu.BVC(ImmediateAM{address: 20});

        assert_eq!(cpu.pc, 20);
    }
    #[test]
    pub fn testOP_BVS() {
        let mut cpu = super::CPU::new_empty();

        cpu.status = 0b01000000;
        cpu.BVS(ImmediateAM{address: 20});

        assert_eq!(cpu.pc, 20);
    }
}
