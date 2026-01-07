use std::path::PathBuf;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "data-gate")]
#[command(version, about = "A data checking and reporting CLI application", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Check data files for consistency and correctness
    Check {

        /// Path to the data file to be checked
        #[arg(value_name = "INPUT", index = 1)]
        input: PathBuf,

        /// Path to a spec file defining the checks
        #[arg(short, long, value_name = "SPEC")]
        spec: PathBuf,

        /// Output directory for the check report
        #[arg(short, long, value_name = "OUT_DIR", default_value = "./data/outbox")]
        out: Option<PathBuf>,

        /// Format of the output report (json or html)
        #[arg(short, long, value_name = "json|html", default_value = "json")]
        format: Option<String>,

        /// Fail on warnings option
        #[arg(short = 'w', long)]
        fail_on_warn: bool,

        /// Sample size for huge files
        #[arg(short = 'z', long, value_name = "N")]
        sample_size: Option<usize>,

    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    
    match &cli.command {
        Commands::Check { input, spec, out, format, fail_on_warn, sample_size } => {
            println!("Checking file: {:?}", input);
            println!("Using spec: {:?}", spec);
            if let Some(out_dir) = out {
                println!("Output directory: {:?}", out_dir);
            }
            println!("Output format: {:?}", format.as_deref().unwrap_or("json"));
            println!("Fail on warnings: {}", fail_on_warn);
            println!("Sample size: {:?}", sample_size);
        }
    }
    Ok(())
}
