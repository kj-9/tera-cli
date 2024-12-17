use clap::CommandFactory;

#[path = "src/main.rs"]
mod cli;

fn main() -> std::io::Result<()> {
    print!("hello1");
    let out_dir =
        std::path::PathBuf::from(std::env::var_os("OUT_DIR").ok_or(std::io::ErrorKind::NotFound)?);

    let cmd = cli::Cli::command();

    let man = clap_mangen::Man::new(cmd);
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;

    println!("{}", String::from_utf8_lossy(&buffer));
    println!("{}", out_dir.join("mybin.1").display());
    std::fs::write(out_dir.join("mybin.1"), buffer)?;

    Ok(())
}
