//! `bmo-app`, the `bmo app` unit command (docs/app-command.md).

use std::process::ExitCode;

fn main() -> ExitCode {
    bmo_app::run(std::env::args_os())
}
