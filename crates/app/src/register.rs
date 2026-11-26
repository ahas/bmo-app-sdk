//! The machine's checkout register and the applications standing in it (docs/app-command.md):
//! the file `BMO_TESTBED_CHECKOUTS` names, else the data root's `testbed/checkouts.toml`. The
//! testbed's own document states the format; this reads what the applications stand in: a
//! `checkouts` list whose entries are paths, each a string or a table naming its `path`.

use std::path::{Path, PathBuf};

use bmo_app_declare::MANIFEST;

/// What the register answers.
pub enum Register {
    /// Every checkout it names, resolved to where each stands, in the order named.
    Checkouts(Vec<PathBuf>),
    /// The register does not read: the reason, worded for a person. A register that stands
    /// nowhere holds no checkouts and answers `Checkouts` empty.
    Refused(String),
}

/// One application this machine carries.
pub struct Application {
    /// The name its manifest gives it.
    pub name: String,
    /// The checkout the manifest names where it names one; the register's line otherwise.
    pub checkout: PathBuf,
    /// Where its manifest stands.
    pub manifest: PathBuf,
}

/// What reading a registered checkout answers about the application it may carry.
pub enum Found {
    /// A manifest stands there and reads.
    Application(Application),
    /// No manifest stands there: not an application.
    NotAnApplication,
    /// A manifest stands there and does not read.
    Unread { checkout: PathBuf, reason: String },
}

/// Why a named application was not found.
pub enum Search {
    /// No application of that name stands among the register's checkouts.
    NotFound,
    /// The register does not read: the reason.
    Refused(String),
}

/// Where the register stands.
fn path() -> PathBuf {
    if let Some(named) = std::env::var_os("BMO_TESTBED_CHECKOUTS") {
        return PathBuf::from(named);
    }
    data_root().join("testbed").join("checkouts.toml")
}

/// Reads the register: the checkouts it names, or why it does not read.
pub fn read() -> Register {
    let file = path();
    let text = match std::fs::read_to_string(&file) {
        Ok(text) => text,
        Err(why) if why.kind() == std::io::ErrorKind::NotFound => {
            return Register::Checkouts(Vec::new());
        }
        Err(why) => return Register::Refused(format!("cannot read {}: {why}", file.display())),
    };
    let document: toml::Table = match text.parse() {
        Ok(document) => document,
        Err(why) => return Register::Refused(format!("{}: {why}", file.display())),
    };
    let entries = match document.get("checkouts") {
        Some(toml::Value::Array(entries)) => entries,
        Some(_) => {
            return Register::Refused(format!("{}: `checkouts` is not a list", file.display()));
        }
        None => {
            return Register::Refused(format!("{}: no `checkouts` list stands", file.display()));
        }
    };
    let mut resolved = Vec::new();
    for (place, entry) in entries.iter().enumerate() {
        let given = match entry {
            toml::Value::String(given) => Some(given.as_str()),
            toml::Value::Table(fields) => fields.get("path").and_then(toml::Value::as_str),
            _ => None,
        };
        let Some(given) = given else {
            return Register::Refused(format!(
                "{}: checkout {} names no path",
                file.display(),
                place + 1
            ));
        };
        match resolve(given, &file) {
            Ok(checkout) => resolved.push(checkout),
            Err(reason) => return Register::Refused(format!("{}: {reason}", file.display())),
        }
    }
    Register::Checkouts(resolved)
}

/// What the checkout at `checkout` answers about the application it carries.
pub fn application(checkout: &Path) -> Found {
    let manifest = checkout.join(MANIFEST);
    if !manifest.is_file() {
        return Found::NotAnApplication;
    }
    match bmo_app_declare::read_manifest(&manifest) {
        Ok(declared) => Found::Application(Application {
            name: declared.name,
            checkout: declared
                .checkout
                .as_deref()
                .map(expand)
                .unwrap_or_else(|| checkout.to_path_buf()),
            manifest,
        }),
        Err(reason) => Found::Unread {
            checkout: checkout.to_path_buf(),
            reason,
        },
    }
}

/// Every application, and every manifest that does not read, among the register's checkouts,
/// in register order; the reason the register does not read where it does not.
pub fn carried() -> Result<Vec<Found>, String> {
    match read() {
        Register::Checkouts(checkouts) => Ok(checkouts
            .iter()
            .map(|checkout| application(checkout))
            .filter(|found| !matches!(found, Found::NotAnApplication))
            .collect()),
        Register::Refused(reason) => Err(reason),
    }
}

/// The application named `wanted`, among the register's checkouts.
pub fn lookup(wanted: &str) -> Result<Application, Search> {
    match read() {
        Register::Checkouts(checkouts) => checkouts
            .iter()
            .find_map(|checkout| match application(checkout) {
                Found::Application(found) if found.name == wanted => Some(found),
                _ => None,
            })
            .ok_or(Search::NotFound),
        Register::Refused(reason) => Err(Search::Refused(reason)),
    }
}

// A path the register names: `~/` reads under the home, an absolute path stands as given,
// and anything else beside the register file.
fn resolve(given: &str, register: &Path) -> Result<PathBuf, String> {
    if let Some(rest) = given.strip_prefix("~/") {
        return Ok(home()?.join(rest));
    }
    let path = PathBuf::from(given);
    if path.is_absolute() {
        return Ok(path);
    }
    match register.parent() {
        Some(folder) => Ok(folder.join(path)),
        None => Ok(path),
    }
}

/// A path a manifest names: `~/` reads under the home; anything else stands as written.
pub fn expand(given: &str) -> PathBuf {
    match given.strip_prefix("~/") {
        Some(rest) => home()
            .map(|home| home.join(rest))
            .unwrap_or_else(|_| PathBuf::from(given)),
        None => PathBuf::from(given),
    }
}

// Where the machine's data root stands.
fn data_root() -> PathBuf {
    match std::env::var_os("BMO_DATA_DIR") {
        Some(root) => PathBuf::from(root),
        None => match home() {
            Ok(home) => home.join(".local/share/bmo/data"),
            Err(_) => PathBuf::from(".local/share/bmo/data"),
        },
    }
}

fn home() -> Result<PathBuf, String> {
    std::env::var_os("HOME")
        .filter(|given| !given.is_empty())
        .map(PathBuf::from)
        .ok_or_else(|| "no home directory to resolve `~/`".to_string())
}
