use crate::Argument::*;
use crate::Instruction::*;
use crate::*;

impl CPU {
    pub fn report_invalid_register(&mut self) {
        UnrecoverableError::InvalidRegister(
            self.pc,
            Some("The register number is too large.".to_string()),
        )
        .err();
        if !CONFIG.quiet {
            println!("Attempting to recover by restarting...");
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
        self.pc = self.starts_at;
    }

    pub fn report_unknown_flag(&self, instruction: &str) {
        RecoverableError::UnknownFlag(
            self.pc,
            Some(format!("Unknown flag in {} instruction", instruction)),
        )
        .err();
    }

    pub fn report_divide_by_zero(&mut self) {
        UnrecoverableError::DivideByZero(self.pc, Some("Attempted to divide by zero.".to_string()))
            .err();
        if !CONFIG.quiet {
            println!("Attempting to recover by restarting...");
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
        self.pc = self.starts_at;
    }
    pub fn check_overflow(&mut self, new_value: i32) {
        if new_value > i16::MAX as i32 || new_value < i16::MIN as i32 {
            RecoverableError::Overflow(self.pc, Some("Overflowed a register.".to_string())).err();
            self.oflag = true;
        }
    }

    pub fn handle_segmentation_fault(&mut self, message: &str) {
        UnrecoverableError::SegmentationFault(self.pc, Some(message.to_string())).err();
        if !CONFIG.quiet {
            println!("Attempting to recover by restarting...");
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
        self.pc = self.starts_at;
    }
}
