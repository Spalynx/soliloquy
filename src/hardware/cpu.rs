/*  Emulates the MOS Technology 6052 CPU of the NES.
 *
 *
 */

//library of different instruction names/ instruction sizes/ instruction modes
//
//






struct CPU {
    Memory: i32,
    Cycles: u64,
    PC: u16,
    SP: u8,

   //	X         byte   // x register
    //Y         byte   // y register
	//C         byte   // carry flag
	//Z         byte   // zero flag
	//I         byte   // interrupt disable flag
	//D         byte   // decimal mode flag
	//B         byte   // break command flag
	//U         byte   // unused flag
	//V         byte   // overflow flag
	//N         byte   // negative flag
	//interrupt byte   // interrupt type to perform
	//stall     int    // number of cycles to stall
	//table     [256]func(*stepInfo) 
}
