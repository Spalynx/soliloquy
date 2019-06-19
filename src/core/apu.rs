/*
$4000 	SQ1_VOL 	Duty and volume for square wave 1
$4001 	SQ1_SWEEP 	Sweep control register for square wave 1
$4002 	SQ1_LO 	Low byte of period for square wave 1
$4003 	SQ1_HI 	High byte of period and length counter value for square wave 1
$4004 	SQ2_VOL 	Duty and volume for square wave 2
$4005 	SQ2_SWEEP 	Sweep control register for square wave 2
$4006 	SQ2_LO 	Low byte of period for square wave 2
$4007 	SQ2_HI 	High byte of period and length counter value for square wave 2
$4008 	TRI_LINEAR 	Triangle wave linear counter
$4009 		Unused, but is eventually accessed in memory-clearing loops
$400A 	TRI_LO 	Low byte of period for triangle wave
$400B 	TRI_HI 	High byte of period and length counter value for triangle wave
$400C 	NOISE_VOL 	Volume for noise generator
$400D 		Unused, but is eventually accessed in memory-clearing loops
$400E 	NOISE_LO 	Period and waveform shape for noise generator
$400F 	NOISE_HI 	Length counter value for noise generator
$4010 	DMC_FREQ 	Play mode and frequency for DMC samples
$4011 	DMC_RAW 	7-bit DAC
$4012 	DMC_START 	Start of DMC waveform is at address $C000 + $40*$xx
$4013 	DMC_LEN 	Length of DMC waveform is $10*$xx + 1 bytes (128*$xx + 8 samples)
$4014 	OAMDMA 	Writing $xx copies 256 bytes by reading from $xx00-$xxFF and writing to OAMDATA ($2004)
$4015 	SND_CHN 	Sound channels enable and status
$4016 	JOY1 	Joystick 1 data (R) and joystick strobe (W)
$4017 	JOY2 	Joystick 2 data (R) and frame counter control (W) 

http://wiki.nesdev.com/w/index.php/2A03
*/