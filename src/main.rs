use clap::Parser;
use tera::{Context, Tera};

use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
#[clap(version = env!("CARGO_PKG_VERSION"))]
struct Cli {
    /// The path to the template file to read
    template_dir: std::path::PathBuf,
    /// The path to the output file
    output_dir: std::path::PathBuf,
    /// Watch for changes in template_dir
    #[clap(short, long)]
    watch: bool,
}

fn main() -> std::io::Result<()> {
    let args = Cli::parse();

    // check if the template_dir is a directory
    if !args.template_dir.is_dir() {
        eprintln!("{} is not a directory", args.template_dir.display());
        std::process::exit(1);
    }

    // check if the output_dir is a directory
    if !args.output_dir.is_dir() {
        eprintln!("{} is not a directory", args.output_dir.display());
        std::process::exit(1);
    }

    if args.watch {
        println!("Watching for changes in {}", args.template_dir.display());
        if let Err(error) = watch(&args.template_dir, &args.output_dir) {
            println!("Error: {error:?}");
        }

        return Ok(());
    }

    let context = Context::new();

    for entry in std::fs::read_dir(&args.template_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let output_file = args.output_dir.join(path.file_name().unwrap());
            render_template(&path, &output_file, &context)?;
        }
    }

    Ok(())
}

// function to render and write the template
// takes template file, output file and context as arguments
fn render_template(
    template_file: &std::path::Path,
    output_file: &std::path::Path,
    context: &tera::Context,
) -> std::io::Result<()> {
    let content = std::fs::read_to_string(template_file)?;
    let result = Tera::one_off(&content, context, false);

    match result {
        Ok(result) => {
            println!("Rendering template: {template_file:?} to {output_file:?}");
            std::fs::write(output_file, result)?;
            Ok(())
        }
        Err(error) => {
            eprintln!("Error rendering template: {error:?}");
            Ok(())
        }
    }
}

fn watch(template_dir: &Path, output_dir: &Path) -> notify::Result<()> {
    let full_tepmlate_dir = template_dir.canonicalize()?;

    let (tx, rx) = std::sync::mpsc::channel();

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(template_dir, RecursiveMode::Recursive)?;

    for res in rx {
        match res {
            Ok(event) => match event.kind {
                EventKind::Modify(_) => {
                    println!("Modify: {event:?}");
                    let modified_path = event.paths.first().unwrap();

                    // get relative path of the file to the path
                    let relative_path = modified_path.strip_prefix(&full_tepmlate_dir).unwrap();
                    let output_file = Path::new(&output_dir).join(relative_path);
                    let context = Context::new();

                    println!("Rendering template: {modified_path:?} to {output_file:?}");
                    render_template(modified_path, &output_file, &context)?;
                }
                _ => {
                    println!("Event: {event:?}");
                }
            },
            Err(error) => println!("Error: {error:?}"),
        }
    }

    Ok(())
}
