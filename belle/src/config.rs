use clap::Parser;
use once_cell::sync::Lazy;
pub static CONFIG: Lazy<Cli> = Lazy::new(declare_config);
/// Command line arguments
#[derive(Parser)]
#[command(name = "belle")]
#[command(version = "0.1.0")]
#[command(author = "gummi")]
#[command(about = "BELLE - The Big Endian, Low Level Emulator", long_about = None)]
pub struct Cli {
    /// Path to input
    #[clap(required = true)]
    pub file: String,

    /// Verbose output
    #[clap(short = 'v', long, default_value_t = false)]
    pub verbose: bool,

    /// Display debug messages
    #[clap(short = 'd', long, default_value_t = false)]
    pub debug: bool,

    /// Quiet (do not print errors)
    #[clap(short = 'q', long, default_value_t = false)]
    pub quiet: bool,

    /// Clock cycle delay (milliseconds)
    #[clap(short = 't', long)]
    pub time_delay: Option<u32>,

    /// Print the state of the CPU when it halts
    #[clap(short = 'p', long, default_value_t = false)]
    pub pretty: bool,
}
#[must_use]
pub fn declare_config() -> Cli {
    let cli = Cli::parse();

    Cli {
        file: cli.file,
        verbose: cli.verbose,
        debug: cli.debug,
        quiet: cli.quiet,
        time_delay: Some(cli.time_delay.unwrap_or(0)),
        pretty: cli.pretty,
    }
}
