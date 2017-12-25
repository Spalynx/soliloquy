/*  Emulates the MOS Technology 6052 CPU of the NES.
 *  Author: Spalynx
 *  Date: 12/8/17
 */

pub fn new() -> CPU{
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
        opcodes:        &[&[&str]],
    }
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

    //CPU Flags
    //  See [1] for reference:
    pub flags           u8,        

	pub interrupt :     u8,     // interrupt type to perform
	pub stall:          i32,    // number of cycles to stall

    opcodes:        &[&[&str]],
}

impl CPU {
    fn step() -> i32{

    }
    pub fn add(&self) -> u64 {
        self.memory as u64 + self.cycles
    }

}

// pub &[&[f64]]











//https://wiki.nesdev.com/w/index.php/CPU_unofficial_opcodes


/* Creates a function array of CPU operations.
 * <p>
 * I was originally planning on making a global table, that could be used
 * as the CPU pleases, but this allowed easy hi-jacking by really anyone.
 *
 */

//[1] https://wiki.nesdev.com/w/index.php/CPU_status_flag_behavior
    //  0: carry flag
    //  1: zero flag
    //  2: interrupt disable flag
    //  3: decimal mode flag
    //  4: break command flag
    //  5: unused flag
    //  6: overflow flag
    //  7: negative flag
//

/*
pub fn cpu_function_arr(&self){
    let self.OPCODES: &[ &[&str] ] = 
            [["BRK", "ORA", "STP", "SLO", "NOP", "ORA", "ASL", "SLO", "PHP", "ORA", "ASL", "ANC", "NOP", "ORA", 
                    "ASL", "SLO", "BPL", "ORA", "STP", "SLO", "NOP", "ORA", "ASL", "SLO", "CLC", "ORA", "NOP",
                    "SLO", "NOP", "ORA", "ASL", "SLO"],
            ["JSR", "AND", "STP", "RLA", "BIT", "AND", "ROL", "RLA", "PLP", "AND", "ROL", "ANC", "BIT", "AND", 
                    "ROL", "RLA", "BMI", "AND", "STP", "RLA", "NOP", "AND", "ROL", "RLA", "SEC", "AND", "NOP",
                    "RLA", "NOP", "AND", "ROL", "RLA"],
            ["RTI", "EOR", "STP", "SRE", "NOP", "EOR", "LSR", "SRE", "PHA", "EOR", "LSR", "ALR", "JMP", "EOR", 
                    "LSR", "SRE", "BVC", "EOR", "STP", "SRE", "NOP", "EOR", "LSR", "SRE", "CLI", "EOR", "NOP",
                    "SRE", "NOP", "EOR", "LSR", "SRE"],
            ["RTS", "ADC", "STP", "RRA", "NOP", "ADC", "ROR", "RRA", "PLA", "ADC", "ROR", "ARR", "JMP", "ADC", 
                    "ROR", "RRA", "BVS", "ADC", "STP", "RRA", "NOP", "ADC", "ROR", "RRA", "SEI", "ADC", "NOP",
                    "RRA", "NOP", "ADC", "ROR", "RRA"],
            ["NOP", "STA", "NOP", "SAX", "STY", "STA", "STX", "SAX", "DEY", "NOP", "TXA", "XAA", "STY", "STA", 
                    "STX", "SAX", "BCC", "STA", "STP", "AHX", "STY", "STA", "STX", "SAX", "TYA", "STA", "TXS",
                    "TAS", "SHY", "STA", "SHX", "AHX"],
            ["LDY", "LDA", "LDX", "LAX", "LDY", "LDA", "LDX", "LAX", "TAY", "LDA", "TAX", "LAX", "LDY", "LDA", 
                    "LDX", "LAX", "BCS", "LDA", "STP", "LAX", "LDY", "LDA", "LDX", "LAX", "CLV", "LDA", "TSX",
                    "LAS", "LDY", "LDA", "LDX", "LAX"],
            ["CPY", "CMP", "NOP", "DCP", "CPY", "CMP", "DEC", "DCP", "INY", "CMP", "DEX", "AXS", "CPY", "CMP", 
                    "DEC", "DCP", "BNE", "CMP", "STP", "DCP", "NOP", "CMP", "DEC", "DCP", "CLD", "CMP", "NOP",
                    "DCP", "NOP", "CMP", "DEC", "DCP"],
            ["CPX", "SBC", "NOP", "ISC", "CPX", "SBC", "INC", "ISC", "INX", "SBC", "NOP", "SBC", "CPX", "SBC", 
                    "INC", "ISC", "BEQ", "SBC", "STP", "ISC", "NOP", "SBC", "INC", "ISC", "SED", "SBC", "NOP",
                    "ISC", "NOP", "SBC", "INC", "ISC"]];
}


        ["BRK7", "ORAizx6",  "KIL",  "SLOizx8",  "NOPzp3",  "ORAzp3",  "ASLzp5",  "SLOzp5", "PHP3", "ORAimm2", "ASL2", "ANCimm2", "NOPabs4", "ORAabs4", "ASLabs6", "SLOabs6"];
BPLrel 2* 	ORAizy 5* 	KIL 	SLOizy 8 	NOPzpx 4 	ORAzpx 4 	ASLzpx 6 	SLOzpx 6 	CLC2 	ORAaby 4* 	NOP2 	SLOaby 7 	NOPabx 4* 	ORAabx 4* 	ASLabx 7 	SLOabx 7
-
 JSRabs 6 ANDizx 6 KIL RLAizx 8 BITzp 3 ANDzp 3 ROLzp 5 RLAzp 5 PLP4 ANDimm 2 ROL2 ANCimm 2 BITabs 4 ANDabs 4 ROLabs 6 RLAabs 6
-

 BMIrel 2* ANDizy 5* KIL RLAizy 8 NOPzpx 4 ANDzpx 4 ROLzpx 6 RLAzpx 6 SEC2 ANDaby 4* NOP2 RLAaby 7 NOPabx 4* ANDabx 4* ROLabx 7 RLAabx 7
-
RTI6 EORizx 6 KIL SREizx 8 NOPzp 3 EORzp 3 LSRzp 5 SREzp 5 PHA3 EORimm 2 LSR2 ALRimm 2 JMPabs 3 EORabs 4 LSRabs 6 SREabs 6
-
BVCrel 2* EORizy 5* KIL SREizy 8 NOPzpx 4 EORzpx 4 LSRzpx 6 SREzpx 6 CLI2 EORaby 4* NOP2 SREaby 7 NOPabx 4* EORabx 4* LSRabx 7 SREabx 7
-
RTS6 ADCizx 6 KIL RRAizx 8 NOPzp 3 ADCzp 3 RORzp 5 RRAzp 5 PLA4 ADCimm 2 ROR2 ARRimm 2 JMPind 5 ADCabs 4 RORabs 6 RRAabs 6
-
BVSrel 2* ADCizy 5* KIL RRAizy 8 NOPzpx 4 ADCzpx 4 RORzpx 6 RRAzpx 6 SEI2 ADCaby 4* NOP2 RRAaby 7 NOPabx 4* ADCabx 4* RORabx 7 RRAabx 7
-
NOPimm 2 STAizx 6 NOPimm 2 SAXizx 6 STYzp 3 STAzp 3 STXzp 3 SAXzp 3 DEY2 NOPimm 2 TXA2 XAAimm 2 STYabs 4 STAabs 4 STXabs 4 SAXabs 4

-
 BCCrel 2* STAizy 6 KIL AHXizy 6 STYzpx 4 STAzpx 4 STXzpy 4 SAXzpy 4 TYA2 STAaby 5 TXS2 TASaby 5 SHYabx 5 STAabx 5 SHXaby 5 AHXaby 5
-
LDYimm 2 LDAizx 6 LDXimm 2 LAXizx 6 LDYzp 3 LDAzp 3 LDXzp 3 LAXzp 3 TAY2 LDAimm 2 TAX2 LAXimm 2 LDYabs 4 LDAabs 4 LDXabs 4 LAXabs 4

-
BCSrel 2* LDAizy 5* KIL LAXizy 5* LDYzpx 4 LDAzpx 4 LDXzpy 4 LAXzpy 4 CLV2 LDAaby 4* TSX2 LASaby 4* LDYabx 4* LDAabx 4* LDXaby 4* LAXaby 4*
-
 CPYimm 2 CMPizx 6 NOPimm 2 DCPizx 8 CPYzp 3 CMPzp 3 DECzp 5 DCPzp 5 INY2 CMPimm 2 DEX2 AXSimm 2 CPYabs 4 CMPabs 4 DECabs 6 DCPabs 6

-
BNErel 2* CMPizy 5* KIL DCPizy 8 NOPzpx 4 CMPzpx 4 DECzpx 6 DCPzpx 6 CLD2 CMPaby 4* NOP2 DCPaby 7 NOPabx 4* CMPabx 4* DECabx 7 DCPabx 7
-
CPXimm 2 SBCizx 6 NOPimm 2 ISCizx 8 CPXzp 3 SBCzp 3 INCzp 5 ISCzp 5 INX2 SBCimm 2 NOP2 SBCimm 2 CPXabs 4 SBCabs 4 INCabs 6 ISCabs 6

-
BEQrel 2* SBCizy 5* KIL ISCizy 8 NOPzpx 4 SBCzpx 4 INCzpx 6 ISCzpx 6 SED2 SBCaby 4* NOP2 ISCaby 7 NOPabx 4* SBCabx 4* INCabx 7 ISCabx 7
-
*/
