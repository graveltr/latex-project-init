use clap::Parser;
use std::fs;

/// Latex notes project initialization
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The name of the project to be created
    #[arg(short, long)]
    project_name: String,

    /// Path to directory to create new project in
    #[arg(short, long, default_value_t = String::from("./"))]
    creation_dir: String,

    /// Path to the latex template file to copy
    #[arg(long, default_value_t = String::from("physics"))]
    template_name: String,

    /// Path to templates directory
    #[arg(long, default_value_t = String::from("/Users/trevor/.latex/"))]
    templates_path: String,

    /// Create exercises subdirectory with file
    #[arg(short, long, default_value_t = false)]
    with_exercises: bool,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let full_dir_path = format!("{}{}{}", args.creation_dir, args.project_name, "/");
    let full_file_path = format!("{}{}{}", full_dir_path, args.project_name, ".tex");

    fs::create_dir(full_dir_path)?;

    let full_template_file_path =
        format!("{}{}{}", args.templates_path, args.template_name, ".tex");
    fs::copy(full_template_file_path, full_file_path)?; // Copy foo.txt to bar.txt
    Ok(())
}
