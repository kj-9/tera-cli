//use std::io::{BufRead, Read};

use clap::Parser;
//use anyhow::{Context, Result};
use tera::{Context, Tera};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
#[clap(version = env!("CARGO_PKG_VERSION"))]
struct Cli {
    /// The path to the template file to read
    template_dir: std::path::PathBuf,
    /// The path to the output file
    output_dir: std::path::PathBuf,
}

fn main() -> std::io::Result<()> {
    let args = Cli::parse();

    if !args.template_dir.is_dir() {
        eprintln!("{} is not a directory", args.template_dir.display());
        std::process::exit(1);
    }

    if !args.output_dir.is_dir() {
        eprintln!("{} is not a directory", args.output_dir.display());
        std::process::exit(1);
    }

    let context = Context::new();

    for entry in std::fs::read_dir(&args.template_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let content = std::fs::read_to_string(&path)?;
            // add stuff to context
            let result = Tera::one_off(&content, &context, false);

            std::fs::write(
                args.output_dir.join(path.file_name().unwrap()),
                result.unwrap(),
            )?;
        }
    }

    Ok(())
}
