use crate::{CONFIG, CPU};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

pub static CPU_STATE: Lazy<Mutex<HashMap<u32, Arc<ModCPU>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));
pub static CLOCK: Lazy<Mutex<u32>> = Lazy::new(|| Mutex::new(0));

pub struct ModCPU {
    pub int_reg: [i16; 4], // r0 thru r5
    pub uint_reg: [u16; 2],
    pub float_reg: [f32; 2], // r6 and r7
    pub memory: HashMap<u16, i16>,
    pub pc: u16, // program counter
    pub ir: i16,
    pub starts_at: u16,
    pub running: bool,
    pub has_ran: bool,
    pub zflag: bool,
    pub oflag: bool,
    pub rflag: bool,
    pub sflag: bool,
    pub hlt_on_overflow: bool,
    pub sp: u16,
    pub bp: u16,
}

impl ModCPU {
    pub fn modcpu_from_cpu(origin: &CPU) -> ModCPU {
        let mut memory = std::collections::HashMap::new();
        for (i, element) in origin.memory.iter().enumerate() {
            if let Some(value) = element {
                memory.insert(i as u16, *value);
            }
        }

        ModCPU {
            int_reg: origin.int_reg,
            uint_reg: origin.uint_reg,
            float_reg: origin.float_reg,
            memory,
            pc: origin.pc,
            ir: origin.ir,
            starts_at: origin.starts_at,
            running: origin.running,
            has_ran: origin.has_ran,
            zflag: origin.zflag,
            oflag: origin.oflag,
            rflag: origin.rflag,
            sflag: origin.sflag,
            hlt_on_overflow: origin.hlt_on_overflow,
            sp: origin.sp,
            bp: origin.bp,
        }
    }
}

impl CPU {
    pub fn record_state(&self) {
        let mut state = CPU_STATE.lock().unwrap();
        let clock = CLOCK.lock().unwrap();
        // Idiot might still overflow
        /*
        while state.len() * std::mem::size_of::<(u32, Arc<CPU>)>() > MAX_MEMORY_LIMIT {
            if let Some((&oldest_key, _)) = state.iter().next() {
                state.remove(&oldest_key);
            }
        }
        */
        let modified = ModCPU::modcpu_from_cpu(self);
        state.insert(*clock, Arc::new(modified));
    }

    pub fn display_state(clock: &u32) {
        if !CONFIG.verbose && !CONFIG.debug {
            return;
        }
        let state = CPU_STATE.lock().unwrap();
        if let Some(cpu) = state.get(clock) {
            println!("\nCPU State for clock cycle {clock}:");
            println!("  Signed Integer Registers : {:?}", cpu.int_reg);
            println!("  Uint registers           : {:?}", cpu.uint_reg);
            println!("  Float Registers          : {:?}", cpu.float_reg);
            println!("  Program Counter          : {}", cpu.pc);
            println!("  Instruction Register     : {:016b}", cpu.ir);
            println!("  Running                  : {}", cpu.running);
            println!("  Zero flag                : {}", cpu.zflag);
            println!("  Overflow flag            : {}", cpu.oflag);
            println!("  Remainder flag           : {}", cpu.rflag);
            println!("  Stack pointer            : {}", cpu.sp);
            println!("  Base pointer             : {}", cpu.bp);
            let mut tmp = CPU::new();
            tmp.ir = cpu.ir;
            println!("  Disassembled Instruction : {}", tmp.parse_instruction());
            if let Some(n) = cpu.memory.get(&cpu.pc) {
                let mut tmp = CPU::new();
                tmp.ir = *n;
                println!("  Next instruction         : {}\n", tmp.parse_instruction());
            }
        } else {
            println!("No CPU state found for clock: {clock}");
        }
    }
}

pub fn display_mem(addr: &usize, clock: &u32) -> Option<i32> {
    let state = CPU_STATE.lock().unwrap();
    if let Some(cpu) = state.get(clock) {
        if let Some(v) = cpu.memory.get(&(*addr as u16)) {
            Some(*v as i32)
        } else {
            eprintln!("Nothing in memory here on this clock cycle\n");
            None
        }
    } else {
        None
    }
}
