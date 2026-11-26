//! `bmo-app`: the `bmo app` unit command (docs/app-command.md). Application work in one
//! place: the verbs front this repository's scaffold and generator, carry the words those
//! tools already take, and answer with their output and their exit codes. The machine's
//! applications are read from its checkout register and the manifests standing there, never
//! from a list kept here.

pub mod register;
pub mod verbs;

use std::ffi::OsString;
use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand};

/// The nouns this command declares and the mark each carries: the applications, their
/// manifests and the checkout register stand on the operations machine, so a work node asks
/// through the bridge.
const NOUNS: [(&str, &str); 4] = [
    ("app new", "home"),
    ("app declare", "home"),
    ("app list", "home"),
    ("app check", "home"),
];

#[derive(Parser)]
#[command(
    name = "bmo-app",
    about = "Application work in one place: new, declare, list, check"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// The nouns this command declares, in the entry command's protocol.
    Nouns,
    /// Application work, found through this machine's checkout register.
    App {
        #[command(subcommand)]
        verb: Verb,
    },
}

#[derive(Subcommand)]
enum Verb {
    /// Stand a new application checkout up from this repository's scaffold.
    New {
        /// The application's name.
        name: String,
        /// The port the service answers on.
        #[arg(long)]
        port: Option<String>,
        /// The port the page server answers on.
        #[arg(long = "pages-port")]
        pages_port: Option<String>,
        /// The port a person's dev server takes.
        #[arg(long = "dev-port")]
        dev_port: Option<String>,
        /// The address people reach the application at; the web half stands where this does.
        #[arg(long = "public-url")]
        public_url: Option<String>,
        /// One sentence on what the application is.
        #[arg(long)]
        description: Option<String>,
        /// Where the checkout stands; beside this repository's, named for the application, where none is given.
        path: Option<PathBuf>,
    },
    /// Write an application's readings from its manifest, or check them with --check.
    Declare {
        /// The application; the checkout the command runs in where none is named.
        name: Option<String>,
        /// Report which readings the manifest no longer stands for, writing nothing.
        #[arg(long)]
        check: bool,
    },
    /// The applications this machine carries, each with whether its readings are in step.
    List,
    /// The declaration check plus the repository's own check, for one application or all.
    Check {
        /// The application; every one the register carries where none is named.
        name: Option<String>,
    },
}

/// Reads the words `bmo` handed this command and answers as the verb's own tool does.
pub fn run(arguments: impl IntoIterator<Item = OsString>) -> ExitCode {
    let cli = Cli::try_parse_from(arguments).unwrap_or_else(|refusal| refusal.exit());
    match cli.command {
        Command::Nouns => answer_nouns(),
        Command::App { verb } => match verb {
            Verb::New {
                name,
                port,
                pages_port,
                dev_port,
                public_url,
                description,
                path,
            } => verbs::new(verbs::NewWords {
                name: &name,
                port: port.as_deref(),
                pages_port: pages_port.as_deref(),
                dev_port: dev_port.as_deref(),
                public_url: public_url.as_deref(),
                description: description.as_deref(),
                path: path.as_deref(),
            }),
            Verb::Declare { name, check } => verbs::declare(name.as_deref(), check),
            Verb::List => verbs::list(),
            Verb::Check { name } => verbs::check(name.as_deref()),
        },
    }
}

// The noun lines the entry command's protocol asks for; the answer reads nothing else.
fn answer_nouns() -> ExitCode {
    for (noun, mark) in NOUNS {
        println!("{noun}\t{mark}");
    }
    ExitCode::SUCCESS
}
