use crate::*;
use colored::*;
#[derive(Debug)]
// tips
pub enum Tip<'a> {
    Try(&'a str),
    Maybe(&'a str),
    NoIdea(&'a str),
}

impl Tip<'_> {
    pub fn display_tip(&self) {
        let msg = format!("no idea. line {}, file {}", line!(), file!());
        let tip = match self {
            Tip::Try(_) => "try to",
            Tip::Maybe(_) => "maybe you",
            Tip::NoIdea(_) => msg.as_str(),
        };
        let tip_message = match self {
            Tip::Try(s) => s,
            Tip::Maybe(s) => s,
            Tip::NoIdea(s) => s,
        };
        if CONFIG.tips {
            println!("{} {} {}", "tip:".yellow(), tip, tip_message);
        }
    }
}
