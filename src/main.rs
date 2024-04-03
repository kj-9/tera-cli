//use std::io::{BufRead, Read};

use clap::Parser;
//use anyhow::{Context, Result};
use tera::{Context, Tera};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
#[clap(version = env!("CARGO_PKG_VERSION"))]
struct Cli {
    /// The path to the template file to read
    template: std::path::PathBuf,
    /// The path to the output file
    #[clap(short, long)]
    output: Option<std::path::PathBuf>,
}

fn main() -> std::io::Result<()> {
    let args = Cli::parse();

    // bulk read file content
    let content = std::fs::read_to_string(args.template)?;

    let context = Context::new();
    // add stuff to context
    let result = Tera::one_off(&content, &context, false);

    match args.output {
        Some(output) => {
            std::fs::write(output, result.unwrap())?;
        }
        None => {
            print!("{}", result.unwrap());
        }
    }

    Ok(())
}
