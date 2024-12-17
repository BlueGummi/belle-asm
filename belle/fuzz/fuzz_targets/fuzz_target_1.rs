#![no_main]

use belle::CPU;
use libfuzzer_sys::fuzz_target;
extern crate belle;

fuzz_target!(|data: &[u8]| {
    if data.len() >= 2 {
        let arr: [u8; 2] = [data[0], data[1]];
        let random_i16 = i16::from_be_bytes(arr);
        let mut cpu = CPU::new();

        cpu.ir = random_i16;

        let instruction = cpu.parse_instruction();
        let _ = cpu.execute_instruction(&instruction);

    }
});
