use crate::*;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use std::collections::HashMap;
pub static CPU_STATE: Lazy<Mutex<HashMap<u16, CPU>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));
pub static CLOCK: Lazy<Mutex<u32>> = Lazy::new(|| Mutex::new(1));
impl CPU {
    fn record_state(&self) {
        // append to the hashmap
        let state = CPU_STATE.lock().unwrap();
        let clock = CLOCK.lock().unwrap();
        state.insert(clock, &self); 

    }
    fn display_state() {
        // print
        println!("blah");
    }
}
