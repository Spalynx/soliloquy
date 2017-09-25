/*  Emulates the MOS Technology 6052 CPU of the NES.
 *
 *
 */

//https://wiki.nesdev.com/w/index.php/CPU_unofficial_opcodes
let CPU_OPCODES = 
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


struct CPU {
    Memory:         i32,
    Cycles:         u64,    //Number of cycles
    PC:             u16,    //Program Counter
    SP:             u8,     //Stack Pointer

    X:              u8,     // x register
    Y:              u8,     // y register

	C:              u8,     // 0: carry flag
	Z:              u8,     // 1: zero flag
	I:              u8,     // 2: interrupt disable flag
	D:              u8,     // 3: decimal mode flag
	B:              u8,     // 4: break command flag
	U:              u8,     // 5: unused flag
	V:              u8,     // 6: overflow flag
	N:              u8,     // 7: negative flag

	interrupt :     u8,     // interrupt type to perform
	stall:          i32,    // number of cycles to stall
}
