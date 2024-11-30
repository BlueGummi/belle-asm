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
        if !CONFIG.verbose {
            return;
        }
        let state = CPU_STATE.lock().unwrap();
        if let Some(cpu) = state.get(&clock) {
            println!("\nCPU State for clock {}:", clock);
            println!("  Int   Registers: {:?}", cpu.int_reg);
            println!("  Float Registers: {:?}", cpu.float_reg);
            /*
            println!("  memory:");
            let mut none_count = 0;

            for (i, &item) in cpu.memory.iter().enumerate() {
                match item {
                    Some(value) => {
                        println!("    memory[{}]: Some({})", i, value);
                        none_count = 0;
                    }
                    None => {
                        println!("    memory[{}]: None", i);
                        none_count += 1;
                        if none_count >= 3 {
                            println!("    ... (stopped displaying after 3 consecutive Nones)");
                            break;
                        }
                    }
                }
            }
            */
            println!("  Program Counter: {}", cpu.pc);
            println!("  Instruction Register: {:016b}", cpu.ir);
            println!("  Jump Location: {}", cpu.jloc);
            println!("  Running state: {}", cpu.running);
            println!("  Zero flag: {}", cpu.zflag);
            println!("  Ovflow flag: {}", cpu.oflag);
        } else {
            println!("No CPU state found for clock: {}", clock);
        }
    }
}
