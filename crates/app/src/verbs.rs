//! What each `app` verb does (docs/app-command.md): the scaffold run with the steps it
//! leaves said, the readings declared with the generator's own answers and exit codes, the
//! applications the register carries, and one application or all of them checked with the
//! testbed's own check standing behind the declaration check.

use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

use bmo_app_declare::{Check, MANIFEST, Writing, check_readings, write_readings};

use crate::register::{self, Application, Found, Register, Search};

/// The scaffold script, standing in this repository's checkout.
const SCAFFOLD: &str = "bin/new-app.sh";

/// The words `app new` was given: the scaffold's own vocabulary, one for one.
pub struct NewWords<'a> {
    pub name: &'a str,
    pub port: Option<&'a str>,
    pub pages_port: Option<&'a str>,
    pub dev_port: Option<&'a str>,
    pub public_url: Option<&'a str>,
    pub description: Option<&'a str>,
    pub path: Option<&'a Path>,
}

/// Stands a new application up through the scaffold, then says the steps left to a person.
pub fn new(words: NewWords<'_>) -> ExitCode {
    let Some(sdk) = find_sdk() else {
        eprintln!(
            "bmo app new: no bmo-app-sdk checkout stands on this machine: name one in the checkout register or set BMO_APP_SDK"
        );
        return ExitCode::from(2);
    };
    let target = match words.path {
        Some(given) => given.to_path_buf(),
        None => match sdk.parent() {
            Some(beside) => beside.join(words.name),
            None => {
                eprintln!(
                    "bmo app new: {} stands nowhere a sibling checkout can",
                    sdk.display()
                );
                return ExitCode::from(2);
            }
        },
    };

    let mut scaffold = Command::new(sdk.join(SCAFFOLD));
    scaffold.arg("--name").arg(words.name);
    for (flag, value) in [
        ("--port", words.port),
        ("--pages-port", words.pages_port),
        ("--dev-port", words.dev_port),
        ("--public-url", words.public_url),
        ("--description", words.description),
    ] {
        if let Some(value) = value {
            scaffold.arg(flag).arg(value);
        }
    }
    scaffold.arg(&target);

    match scaffold.status() {
        Ok(run) if run.success() => {
            say_steps_left(words.name, &target);
            ExitCode::SUCCESS
        }
        Ok(run) => ExitCode::from(run.code().unwrap_or(1) as u8),
        Err(why) => {
            eprintln!("bmo app new: the scaffold could not be started: {why}");
            ExitCode::FAILURE
        }
    }
}

/// Writes an application's readings from its manifest, or reports which of them the
/// manifest no longer stands for where `check` is asked, answering as the generator does.
pub fn declare(name: Option<&str>, check: bool) -> ExitCode {
    let manifest = match name {
        None => PathBuf::from(MANIFEST),
        Some(wanted) => match register::lookup(wanted) {
            Ok(found) => found.manifest,
            Err(refusal) => {
                eprintln!("bmo app declare: {}", search_said(wanted, &refusal));
                return ExitCode::from(2);
            }
        },
    };
    if check {
        return match check_readings(&manifest) {
            Check::InStep => ExitCode::SUCCESS,
            Check::OutOfStep(off) => {
                for line in off {
                    println!("{line}");
                }
                ExitCode::FAILURE
            }
            Check::Refused(reason) => {
                eprintln!("bmo app declare: {reason}");
                ExitCode::FAILURE
            }
        };
    }
    match write_readings(&manifest) {
        Writing::Written(written) => {
            println!("bmo app declare: wrote the declarations of {written}");
            ExitCode::SUCCESS
        }
        Writing::Refused(reasons) => {
            for reason in reasons {
                eprintln!("bmo app declare: {reason}");
            }
            ExitCode::FAILURE
        }
    }
}

/// The applications this machine carries, one a line: name, checkout, and whether its
/// readings stand in step. The report never fails on what it finds; only a register that
/// does not read is refused.
pub fn list() -> ExitCode {
    let carried = match register::carried() {
        Ok(carried) => carried,
        Err(reason) => {
            eprintln!("bmo app list: the machine's checkout register does not read: {reason}");
            return ExitCode::from(2);
        }
    };
    if carried.is_empty() {
        eprintln!("bmo app list: this machine's checkout register carries no applications");
        return ExitCode::SUCCESS;
    }
    for found in carried {
        match found {
            Found::Unread { checkout, reason } => {
                println!("{}  {}  {reason}", name_of(&checkout), checkout.display());
            }
            Found::Application(application) => {
                let step = match check_readings(&application.manifest) {
                    Check::InStep => "in step".to_string(),
                    Check::OutOfStep(off) => format!("out of step: {}", off.join("; ")),
                    Check::Refused(reason) => reason,
                };
                println!(
                    "{}  {}  {step}",
                    application.name,
                    application.checkout.display()
                );
            }
            Found::NotAnApplication => {}
        }
    }
    ExitCode::SUCCESS
}

/// The declaration check plus the repository's own check, for one application or for every
/// one the register carries.
pub fn check(name: Option<&str>) -> ExitCode {
    let carried: Vec<Found> = match name {
        Some(wanted) => match register::lookup(wanted) {
            Ok(application) => vec![Found::Application(application)],
            Err(refusal) => {
                eprintln!("bmo app check: {}", search_said(wanted, &refusal));
                return ExitCode::from(2);
            }
        },
        None => match register::carried() {
            Ok(carried) => carried,
            Err(reason) => {
                eprintln!("bmo app check: the machine's checkout register does not read: {reason}");
                return ExitCode::from(2);
            }
        },
    };
    if carried.is_empty() {
        eprintln!(
            "bmo app check: this machine's checkout register carries no applications to check"
        );
        return ExitCode::from(2);
    }
    let mut failed = false;
    for found in carried {
        match found {
            Found::Unread { checkout, reason } => {
                println!(
                    "bmo app check: {}: the manifest does not read: {reason}",
                    checkout.display()
                );
                failed = true;
            }
            Found::Application(application) => failed |= check_one(&application),
            Found::NotAnApplication => {}
        }
    }
    if failed {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

// Both checks of one application: its readings against its manifest, then the repository's
// own check run inside that checkout by the testbed's check verb. Answers whether it failed.
fn check_one(application: &Application) -> bool {
    println!("bmo app check: {}", application.name);
    let mut failed = false;
    match check_readings(&application.manifest) {
        Check::InStep => println!("  declarations: in step"),
        Check::OutOfStep(off) => {
            println!("  declarations: out of step");
            for line in off {
                println!("    {line}");
            }
            failed = true;
        }
        Check::Refused(reason) => {
            println!("  declarations: {reason}");
            failed = true;
        }
    }
    let repository = Command::new("bmo-testbed")
        .arg("check")
        .current_dir(&application.checkout)
        .status();
    match repository {
        Ok(run) if run.success() => {
            println!("  {}: the repository's own check passed", application.name)
        }
        Ok(run) => {
            println!(
                "  {}: the repository's own check failed (exit {})",
                application.name,
                run.code().unwrap_or(1)
            );
            failed = true;
        }
        Err(why) => {
            println!(
                "  {}: the repository's own check could not be started: {why}",
                application.name
            );
            failed = true;
        }
    }
    failed
}

// This repository's checkout, the way the scaffold finds its own: the one `BMO_APP_SDK`
// names where it is set, else the register entry that carries the scaffold.
fn find_sdk() -> Option<PathBuf> {
    if let Some(named) = std::env::var_os("BMO_APP_SDK") {
        let given = PathBuf::from(named);
        if !given.as_os_str().is_empty() {
            return Some(given);
        }
    }
    match register::read() {
        Register::Checkouts(checkouts) => checkouts
            .into_iter()
            .find(|checkout| carries_scaffold(checkout)),
        Register::Refused(_) => None,
    }
}

fn carries_scaffold(checkout: &Path) -> bool {
    checkout.join(SCAFFOLD).is_file() && checkout.join("Cargo.toml").is_file()
}

fn say_steps_left(name: &str, target: &Path) {
    println!(
        "bmo app new: {name} stands at {}. The steps left to a person, in order:",
        target.display()
    );
    println!("  1. Write the domain in the new checkout.");
    println!("  2. Check it: run `bmo check` inside the checkout.");
    println!("  3. Give it a remote under the organisation and push `main`.");
    println!(
        "  4. Register the checkout: add its path to the machine's checkout register and its row to the set index's presets."
    );
    println!(
        "  5. Register the repository: `bmo repository add {name} --path {}`.",
        target.display()
    );
    println!("  6. Ask for the install and the first release through the supervisor.");
}

// What a checkout holds its name by: the last piece of its path.
fn name_of(checkout: &Path) -> String {
    checkout
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| checkout.display().to_string())
}

// Why the application named `wanted` was not found, worded for a person.
fn search_said(wanted: &str, refusal: &Search) -> String {
    match refusal {
        Search::NotFound => {
            format!("no application named {wanted} stands in this machine's checkouts")
        }
        Search::Refused(reason) => {
            format!("the machine's checkout register does not read: {reason}")
        }
    }
}
