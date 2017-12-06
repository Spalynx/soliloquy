/*  Emulates the MOS Technology 6052 CPU of the NES.
 *
 *
 */

pub fn new() -> CPU{
    CPU{
        memory:         0,
        cycles:         0,    //Number of cycles
        pc:             0,    //Program Counter
        sp:             0,     //Stack Pointer

        x:              0,     // x register
        y:              0,     // y register

        c:              0,     // 0: carry flag
        z:              0,     // 1: zero flag
        i:              0,     // 2: interrupt disable flag
        d:              0,     // 3: decimal mode flag
        b:              0,     // 4: break command flag
        u:              0,     // 5: unused flag
        v:              0,     // 6: overflow flag
        n:              0,     // 7: negative flag

        interrupt :     0,     // interrupt type to perform
        stall:          0,    // number of cycles to stall
        //OPCODES:        &[&[CpuOp]],
    }
}

impl CPU {
    pub fn add(&self) -> u64 {
        self.memory as u64 + self.cycles
    }


/*    pub fn cpu_function_arr(&self){
        let self.OPCODES: &[&[&str]]= 
                [[BRK, ORA, STP, SLO, NOP, ORA, ASL, SLO, PHP, ORA, ASL, ANC, NOP, ORA, 
                        ASL, SLO, BPL, ORA, STP, SLO, NOP, ORA, ASL, SLO, CLC, ORA, NOP,
                        SLO, NOP, ORA, ASL, SLO],
                [JSR, AND, STP, RLA, BIT, AND, ROL, RLA, PLP, AND, ROL, ANC, BIT, AND, 
                        ROL, RLA, BMI, AND, STP, RLA, NOP, AND, ROL, RLA, SEC, AND, NOP,
                        RLA, NOP, AND, ROL, RLA],
                [RTI, EOR, STP, SRE, NOP, EOR, LSR, SRE, PHA, EOR, LSR, ALR, JMP, EOR, 
                        LSR, SRE, BVC, EOR, STP, SRE, NOP, EOR, LSR, SRE, CLI, EOR, NOP,
                        SRE, NOP, EOR, LSR, SRE],
                [RTS, ADC, STP, RRA, NOP, ADC, ROR, RRA, PLA, ADC, ROR, ARR, JMP, ADC, 
                        ROR, RRA, BVS, ADC, STP, RRA, NOP, ADC, ROR, RRA, SEI, ADC, NOP,
                        RRA, NOP, ADC, ROR, RRA],
                [NOP, STA, NOP, SAX, STY, STA, STX, SAX, DEY, NOP, TXA, XAA, STY, STA, 
                        STX, SAX, BCC, STA, STP, AHX, STY, STA, STX, SAX, TYA, STA, TXS,
                        TAS, SHY, STA, SHX, AHX],
                [LDY, LDA, LDX, LAX, LDY, LDA, LDX, LAX, TAY, LDA, TAX, LAX, LDY, LDA, 
                        LDX, LAX, BCS, LDA, STP, LAX, LDY, LDA, LDX, LAX, CLV, LDA, TSX,
                        LAS, LDY, LDA, LDX, LAX],
                [CPY, CMP, NOP, DCP, CPY, CMP, DEC, DCP, INY, CMP, DEX, AXS, CPY, CMP, 
                        DEC, DCP, BNE, CMP, STP, DCP, NOP, CMP, DEC, DCP, CLD, CMP, NOP,
                        DCP, NOP, CMP, DEC, DCP],
                [CPX, SBC, NOP, ISC, CPX, SBC, INC, ISC, INX, SBC, NOP, SBC, CPX, SBC, 
                        INC, ISC, BEQ, SBC, STP, ISC, NOP, SBC, INC, ISC, SED, SBC, NOP,
                        ISC, NOP, SBC, INC, ISC]];

    }
    */
}
#[allow(dead_code)]
pub struct CpuStep {
	address:    u16,
	pc:         u16, 
	mode:       u8, 
}

type CpuOp = fn(CpuStep) -> i8;

pub struct CPU {
    pub memory:         i32,
    pub cycles:         u64,    //Number of cycles
    pub pc:             u16,    //Program Counter
    pub sp:             u8,     //Stack Pointer

    pub x:              u8,     // x register
    pub y:              u8,     // y register

	pub c:              u8,     // 0: carry flag
	pub z:              u8,     // 1: zero flag
	pub i:              u8,     // 2: interrupt disable flag
	pub d:              u8,     // 3: decimal mode flag
	pub b:              u8,     // 4: break command flag
	pub u:              u8,     // 5: unused flag
	pub v:              u8,     // 6: overflow flag
	pub n:              u8,     // 7: negative flag

	pub interrupt :     u8,     // interrupt type to perform
	pub stall:          i32,    // number of cycles to stall

    //OPCODES:        &[&[CpuOp]],
}
//https://wiki.nesdev.com/w/index.php/CPU_unofficial_opcodes
// pub &[&[f64]]

/* Creates a function array of CPU operations.
 * <p>
 * I was originally planning on making a global table, that could be used
 * as the CPU pleases, but this allowed easy hi-jacking by really anyone.
 *
 */

