/*  Unit test module of the MOS 6052 CPU (cpu.rs).
 *  Author: Spalynx
 *  Init: 6/17/18
 */
use core::cpu::*;
//use core::memory::*;

#[cfg(test)]
pub mod cpu_test {
    //Tests could be greatly improved with
    // [before_each]/[assemblyintialize]... Maybe one day!
    use super::*;

    //~~~CPU~META~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    #[test]
    fn test_cpu_init(){
        let test_cpu = super::CPU::new();
        
        //Testing each struct field initialization.
        assert_eq!(test_cpu.cycles,   0);
        assert_eq!(test_cpu.pc,       0);
        assert_eq!(test_cpu.sp,       0);
        assert_eq!(test_cpu.x,        0);
        assert_eq!(test_cpu.y,        0);
        assert_eq!(test_cpu.status,   0);
        assert_eq!(test_cpu.interrupt,0);
        assert_eq!(test_cpu.stall,    0);
    }

    #[test]
    fn test_status_each(){
        let mut cpu = super::CPU::new();

        //Array of status codes.
        let status = ["N", "n", "V", "v", "S", "s", "B", "b",
                      "D", "d", "I", "i", "Z", "z", "C", "c"];

        //Loops through array of status codes, and sets/resets each flag.
        for f in status.iter() {
            //Setting each flag.
            cpu.set_status(f, true);
            assert_eq!(cpu.get_status(f), true);

            //Resetting each flag.
            cpu.set_status(f, false);
            assert_eq!(cpu.get_status(f), false);
        }

    }

    #[test]
    fn test_status_settingtwice(){
        //Making sure that nothing is changed when setting something twice.
        // When I would set a false flag to false, it would be set to true.
        let mut cpu = super::CPU::new();

        //False
        cpu.set_status("I", false);
        cpu.set_status("I", false);
        assert_eq!(cpu.get_status("I"), false);
        
        //True
        cpu.set_status("V", true);
        cpu.set_status("V", true);
        assert_eq!(cpu.get_status("V"), true);
    }

    #[test]
    fn test_new_memory(){
        let mut cpu = super::CPU::new();
        let mem = super::MEM::new();

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

        let mut cpu = super::CPU::new();
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



    //~~~CPU~OPCODES~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    #[test]
    fn testOP_flags(){
        //Testing CLC, CLD, CLI, CLV, SEC, SED, SEI.
        // Reliant on test_status() being correct.
        //Yes, I understand that I should split this up,
        // but cargo test doesn't really avoid bpcode atm.

        let mut cpu = super::CPU::new();

        //~! CLEARS

        //Carry clear.
        cpu.set_status("C", true);
        cpu.CLC();
        assert_eq!(cpu.get_status("C"), false);
        //Decimal clear.
        cpu.set_status("D", true);
        cpu.CLD();
        assert_eq!(cpu.get_status("D"), false);
        //Interrupt carry.
        cpu.set_status("I", true);
        cpu.CLI();
        assert_eq!(cpu.get_status("I"), false);

        //~! SETS
        //Carry set.
        cpu.set_status("C", false);
        cpu.SEC();
        assert_eq!(cpu.get_status("C"), true);
        //Decimal set.
        cpu.set_status("D", false);
        cpu.SED();
        assert_eq!(cpu.get_status("D"), true);
        //Interrupt set.
        cpu.set_status("I", false);
        cpu.SEI();
        assert_eq!(cpu.get_status("I"), true);
        

    }

    #[test]
    fn testOP_LDA(){
        let mut cpu = super::CPU::new();

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
        let mut cpu = super::CPU::new();
        
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
        let mut cpu = super::CPU::new();

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



    //~~~INSTRUCTION~META~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    #[test]
    fn test_instruction_init(){
        let test_instr = super::Instructions::new();

        //Testing the instruction table for init.
        assert_eq!(test_instr.names.len()     ,256);
        assert_eq!(test_instr.sizes.len()     ,256);
        assert_eq!(test_instr.modes.len()     ,256);
        assert_eq!(test_instr.speeds.len()    ,256);
        assert_eq!(test_instr.paging.len()    ,256);

        //This can be expanded later when testing opcode parsing.
    }

    #[test]
    fn testOP_ADC(){
        //Testing for basic ADd with Carry.
        let mut cpu = super::CPU::new();


        //Filling register a with first value.
        cpu.a = 255;
        //Adding second value to A.
        cpu.ADC(ImmediateAM{address: 255})

        //Assert that the value has changed correctly.
            assert_eq!(cpu.a, 254);
        //Assert that flags have be suitably modified.
            assert_eq!(cpu.get_status("C"), true);
            //status("N") should be false, because set_ZN wasn't run.
    }
    fn testOP_ADC_memory_with_lda(){
        //Testing for basic ADd with Carry.
        let mut cpu = super::CPU::new();



        //Filling register a with first value.
        cpu.memory.set(0x755, 237); 
        cpu.LDA(AbsoluteAM{address:  0x755});
        //Adding second value to A.
        cpu.ADC(ImmediateAM{address: 255})

        //Assert that the value has changed correctly.
            assert_eq!(cpu.a, 236);
        //Assert that flags have be suitably modified.
            assert_eq!(cpu.get_status("C"), true);
            assert_eq!(cpu.get_status("N"), true);
    }
    /*
    #[test]
    fn testOP_ADC_signed (){
        //Testing signed arithmetic.
        let test_cpu = super::CPU::new();

        test_cpu.LDA(0b01111111u8); //+127
        test_cpu.ADC(0b00000010u8); //+2
        assert_eq!(test_cpu.A, 0b10000001u8); // = -127
        //Overflow should be set because bit 7 is '1'.
        assert_eq!(test_cpu.get_status("O"), 1);
    }
    #[test]
    fn testOP_ADC_decimal(){
        //TODO: Test adc with a decimal.
        let test_cpu = super::CPU::new();

        test_cpu.SED();
        assert!();
    }
    #[test]
    fn testOP_flags(){
        /// TEST -> CLC, CLD, CLI, CLV, SEC, SED, SEI
        let test_cpu = super::CPU::new();
    }
    */

}
