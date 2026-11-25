//! `bmo-app-declare`: reads an application's `bmo-app.toml` and writes the repository's
//! generated declarations beside it, or says which of them the manifest no longer stands for.

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use bmo_app_declare::{Generated, Manifest};
use clap::Parser;

#[derive(Parser)]
#[command(
    name = "bmo-app-declare",
    about = "Generate an application's declarations from its bmo-app.toml manifest"
)]
struct Cli {
    /// Say which generated files the manifest no longer stands for, writing nothing.
    #[arg(long)]
    check: bool,
    /// The manifest to read.
    manifest: PathBuf,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let text = match std::fs::read_to_string(&cli.manifest) {
        Ok(text) => text,
        Err(error) => {
            eprintln!(
                "bmo-app-declare: cannot read {}: {error}",
                cli.manifest.display()
            );
            return ExitCode::FAILURE;
        }
    };
    let manifest = match Manifest::parse(&text) {
        Ok(manifest) => manifest,
        Err(reason) => {
            eprintln!("bmo-app-declare: {}: {reason}", cli.manifest.display());
            return ExitCode::FAILURE;
        }
    };
    let root = cli.manifest.parent().map_or_else(
        || Path::new(".").to_path_buf(),
        |parent| parent.to_path_buf(),
    );
    let mut failed = false;
    for file in bmo_app_declare::render(&manifest) {
        let path = root.join(&file.path);
        if cli.check {
            match std::fs::read_to_string(&path) {
                Ok(current) if current == file.content => {}
                Ok(_) => {
                    println!("out of date: {}", file.path);
                    failed = true;
                }
                Err(_) => {
                    println!("missing: {}", file.path);
                    failed = true;
                }
            }
            continue;
        }
        if let Some(parent) = path.parent()
            && let Err(error) = std::fs::create_dir_all(parent)
        {
            eprintln!("bmo-app-declare: {}: {error}", parent.display());
            failed = true;
            continue;
        }
        if let Err(error) = write_generated(&path, &file) {
            eprintln!("bmo-app-declare: {}: {error}", path.display());
            failed = true;
        }
    }
    if failed {
        return ExitCode::FAILURE;
    }
    if !cli.check {
        println!(
            "bmo-app-declare: wrote the declarations of {}",
            manifest.name
        );
    }
    ExitCode::SUCCESS
}

fn write_generated(path: &Path, file: &Generated) -> std::io::Result<()> {
    std::fs::write(path, &file.content)?;
    if file.executable {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o755))?;
    }
    Ok(())
}
