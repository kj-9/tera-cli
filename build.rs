use clap::CommandFactory;

#[path = "src/main.rs"]
#[allow(dead_code)]
mod cli;

fn main() -> std::io::Result<()> {
    println!("Starting build");

    let out_dir =
        std::path::PathBuf::from(std::env::var_os("OUT_DIR").ok_or(std::io::ErrorKind::NotFound)?);

    println!("OUT_DIR: {:?}", out_dir);

    let cmd = cli::Cli::command();

    let man = clap_mangen::Man::new(cmd);
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;

    let out_file = out_dir.join("tera-cli.1");
    println!("{:?}", out_file);
    std::fs::write(out_file, buffer)?;

    println!("Build script completed successfully");

    Ok(())
}
