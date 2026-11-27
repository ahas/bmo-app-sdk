//! The readings on disk: read a manifest, check the files beside it stand as it renders them, or write them.

use std::path::{Path, PathBuf};

use crate::*;

/// Whether an application's readings stand as its manifest has them; nothing is written.
#[derive(Debug, PartialEq, Eq)]
pub enum Check {
    /// Every reading stands where the manifest has it.
    InStep,
    /// Every reading the manifest no longer stands for, each worded for a person.
    OutOfStep(Vec<String>),
    /// The manifest stands nowhere, does not read, or does not hold together.
    Refused(String),
}

/// What came of writing an application's readings beside its manifest.
#[derive(Debug, PartialEq, Eq)]
pub enum Writing {
    /// Every reading went down; the application's name.
    Written(String),
    /// Why the readings did not go down, each reason worded for a person — the manifest's own
    /// failure among them, first, where it is one.
    Refused(Vec<String>),
}

/// Reads the manifest at `path`, or says why it does not read, in words for a person.
pub fn read_manifest(path: &Path) -> Result<Manifest, String> {
    let text = std::fs::read_to_string(path)
        .map_err(|why| format!("cannot read {}: {why}", path.display()))?;
    Manifest::parse(&text).map_err(|reason| format!("{}: {reason}", path.display()))
}

/// Reads the manifest at `path` and tells which of its readings, if any, it no longer stands
/// for, writing nothing.
pub fn check_readings(path: &Path) -> Check {
    let manifest = match read_manifest(path) {
        Ok(manifest) => manifest,
        Err(reason) => return Check::Refused(reason),
    };
    let root = beside(path);
    let mut off = Vec::new();
    for file in render(&manifest) {
        match std::fs::read_to_string(root.join(&file.path)) {
            Ok(current) if current == file.content => {}
            Ok(_) => off.push(format!("out of date: {}", file.path)),
            Err(_) => off.push(format!("missing: {}", file.path)),
        }
    }
    if off.is_empty() {
        Check::InStep
    } else {
        Check::OutOfStep(off)
    }
}

/// Reads the manifest at `path` and writes every reading it stands for beside it.
pub fn write_readings(path: &Path) -> Writing {
    let manifest = match read_manifest(path) {
        Ok(manifest) => manifest,
        Err(reason) => return Writing::Refused(vec![reason]),
    };
    let root = beside(path);
    let mut refused = Vec::new();
    for file in render(&manifest) {
        let target = root.join(&file.path);
        if let Some(parent) = target.parent()
            && let Err(why) = std::fs::create_dir_all(parent)
        {
            refused.push(format!("{}: {why}", parent.display()));
            continue;
        }
        if let Err(why) = write_generated(&target, &file) {
            refused.push(format!("{}: {why}", target.display()));
        }
    }
    if refused.is_empty() {
        Writing::Written(manifest.name)
    } else {
        Writing::Refused(refused)
    }
}

// The directory the readings of a manifest at `path` stand beside.
pub(crate) fn beside(path: &Path) -> PathBuf {
    path.parent()
        .map_or_else(|| PathBuf::from("."), Path::to_path_buf)
}

// One reading down at `path`, executable where the manifest's script.
pub(crate) fn write_generated(path: &Path, file: &Generated) -> std::io::Result<()> {
    std::fs::write(path, &file.content)?;
    if file.executable {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o755))?;
    }
    Ok(())
}
