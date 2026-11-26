//! `bmo-app-declare`: reads an application's `bmo-app.toml` and writes the repository's
//! generated declarations beside it, or says which of them the manifest no longer stands for.

use std::path::PathBuf;
use std::process::ExitCode;

use bmo_app_declare::{Check, Writing, check_readings, write_readings};
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
    if cli.check {
        return match check_readings(&cli.manifest) {
            Check::InStep => ExitCode::SUCCESS,
            Check::OutOfStep(off) => {
                for line in off {
                    println!("{line}");
                }
                ExitCode::FAILURE
            }
            Check::Refused(reason) => {
                eprintln!("bmo-app-declare: {reason}");
                ExitCode::FAILURE
            }
        };
    }
    match write_readings(&cli.manifest) {
        Writing::Written(name) => {
            println!("bmo-app-declare: wrote the declarations of {name}");
            ExitCode::SUCCESS
        }
        Writing::Refused(reasons) => {
            for reason in reasons {
                eprintln!("bmo-app-declare: {reason}");
            }
            ExitCode::FAILURE
        }
    }
}
