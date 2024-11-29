use crate::*;

pub struct CPU {
    pub int_reg: [i16; 6], // r0 thru r5
    pub float_reg: [f32; 2], // r6 and r7
    pub memory: [i16; 65536],
    pub pc: u16, // program counter
    pub ic: u16, // instruction counter
    pub sp: u16,
    pub bp: u16,
    pub jloc: u16, // location from which a jump was performed
    pub running: bool,
    pub zflag: bool,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            int_reg: [0; 6],
            float_reg: [0; 2],
            memory: [0; 65536],
            pc: 0,
            ic: 0,
            sp: 0,
            bp: 0,
            running: false,
            zflag: false,
        }
    }
    
    // we need a function to load instructions into RAM
    // we also need interrupts for pseudo-instructions
    //
    // debug messages would be nice too

