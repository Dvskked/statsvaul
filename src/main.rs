mod calculator;
mod formatter;
mod parser;

use std::io::Read;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use clap::Parser;

/// Reads a JSON dataset of performance scores and prints basic statistics
/// (average, minimum, maximum, standard deviation) as a clean table.
#[derive(Parser)]
#[command(name = "cli-stats-vault", version, about)]
struct Cli {
    /// Path to the JSON dataset. Use "-" (or omit it) to read from stdin.
    #[arg(value_name = "FILE")]
    input: Option<PathBuf>,

    /// Number of decimal places used when printing numbers.
    #[arg(long, default_value_t = 2)]
    decimals: usize,
}

fn main() {
    if let Err(error) = run() {
        eprintln!("error: {error:#}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();
    let raw = read_input(cli.input.as_deref())?;
    let scores = parser::parse_scores(&raw)?;

    match calculator::calculate(&scores) {
        Some(stats) => {
            println!("{}", formatter::format_table(&stats, cli.decimals));
            Ok(())
        }
        None => bail!("the dataset contains no scores to analyze"),
    }
}

fn read_input(path: Option<&Path>) -> Result<String> {
    match path {
        Some(path) if path.as_os_str() != "-" => std::fs::read_to_string(path)
            .with_context(|| format!("could not read {}", path.display())),
        _ => {
            let mut buffer = String::new();
            std::io::stdin()
                .read_to_string(&mut buffer)
                .context("could not read from standard input")?;
            Ok(buffer)
        }
    }
}
