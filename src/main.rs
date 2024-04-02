//use std::io::{BufRead, Read};

use clap::Parser;
//use anyhow::{Context, Result};
use tera::{Tera, Context};


/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The path to the template file to read
    template: std::path::PathBuf,
}


fn main() -> std::io::Result<()> {


    let args = Cli::parse();

    // bulk read file content
    let content = std::fs::read_to_string(args.template)?;


    let context = Context::new();
    // add stuff to context
    let result = Tera::one_off(&content, &context, false);

    print!("{}", result.unwrap());

    Ok(())

}



