pub enum Command {
    Quit(),           // q
    Help(),           // h
    Load(),           // l
    Run(),            // r
    Step(),           // s
    StepBack(),       // sb
    PrintMem(u16),    // p
    SetBreak(u16),    // b
    PrintBreak(),     // bp
    RemoveBreak(u16), // br
    RemoveAllBreak(),
    Info(), // i
    Unknown(),
}

impl Command {
    pub fn get_value(&self) -> u16 {
        match self {
            Command::PrintMem(n) => *n,
            Command::SetBreak(n) => *n,
            Command::RemoveBreak(n) => *n,
            _ => 0,
        }
    }
} // implement something here to print help
