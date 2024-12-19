use clap::Parser;
pub use once_cell::sync::Lazy;
pub static CONFIG: Lazy<Args> = Lazy::new(declare_config);
/// Command line arguments
#[derive(Parser)]
#[command(name = "belle-asm")]
#[command(version = "0.1.0")]
#[command(author = "gummi")]
#[command(about = "The assembler for BELLE", long_about = None)]
pub struct Args {
    /// Output file for binary
    #[clap(short = 'o', long)]
    pub output: Option<String>,

    /// Path to input
    #[clap(required = true)]
    pub file: String,

    /// Verbose output
    #[clap(short = 'v', long, default_value_t = false)]
    pub verbose: bool,

    /// Display debug messages
    #[clap(short = 'd', long, default_value_t = false)]
    pub debug: bool,

    /// Display tips (may improve errors)
    #[clap(short = 't', long, default_value_t = true)]
    pub tips: bool,
    // /// Cow
    //#[clap(short = 'c', long, default_value_t = false)]
    //pub cow: bool,
}

/// Parse command line arguments and return the configuration
#[must_use]
pub fn declare_config() -> Args {
    let cli = Args::parse();
    let output = cli.output.unwrap_or_else(|| "a.out".to_string());
    Args {
        file: cli.file,
        output: Some(output),
        verbose: cli.verbose,
        debug: cli.debug,
        tips: cli.tips,
        // cow: cli.cow,
    }
}
