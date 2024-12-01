use crate::*;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
pub static CPU_STATE: Lazy<Mutex<HashMap<u32, Arc<CPU>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));
pub static CLOCK: Lazy<Mutex<u32>> = Lazy::new(|| Mutex::new(0));
impl CPU {
    pub fn record_state(&self) {
        let mut state = CPU_STATE.lock().unwrap();
        let clock = CLOCK.lock().unwrap();
        state.insert(*clock, Arc::new(self.clone()));
    }

    pub fn display_state(clock: u32) {
        if !CONFIG.verbose && !CONFIG.debug {
            return;
        }
        let state = CPU_STATE.lock().unwrap();
        if let Some(cpu) = state.get(&clock) {
            println!("\nCPU State for clock cycle {}:", clock);
            println!("  Integer Registers        : {:?}", cpu.int_reg);
            println!("  Float Registers          : {:?}", cpu.float_reg);
            println!("  Program Counter          : {}", cpu.pc);
            println!("  Instruction Register     : {:016b}", cpu.ir);
            println!("  Jump Location            : {}", cpu.jloc);
            println!("  Running                  : {}", cpu.running);
            println!("  Zero flag                : {}", cpu.zflag);
            println!("  Overflow flag            : {}", cpu.oflag);
            println!("  Remainder flag           : {}", cpu.rflag);
            println!("  Disassembled Instruction : \n{}", disassemble(cpu.ir));
        } else {
            println!("No CPU state found for clock: {}", clock);
        }
    }
}
