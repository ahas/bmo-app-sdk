//! The world every test of the command stands in: its own folder under the machine's
//! temporary directory, with a home, a data root, a checkout register and a bin of its own,
//! so no run ever reaches the real machine's register. The folder is taken down whole when
//! the world ends.

#![allow(dead_code)]

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A service-only application manifest, the least one the generator takes.
pub const MANIFEST: &str = r#"app = 1
name = "qa-applet"
description = "An applet kept for a test"
env_prefix = "APPLET"
deploy_root = "~/.local/share/qa-applet"
data_root = "~/.local/share/qa-applet-data"
service_port = 7700
"#;

/// The world, and the places inside it the command is told to read.
pub struct World {
    pub root: PathBuf,
    pub home: PathBuf,
    pub data: PathBuf,
    pub bin: PathBuf,
    pub register: PathBuf,
}

impl World {
    /// A fresh world named for its test; whatever stood under that name before is gone.
    pub fn new(name: &str) -> World {
        let root = std::env::temp_dir().join(format!("bmo-app-test-{name}"));
        let _ = fs::remove_dir_all(&root);
        let world = World {
            home: root.join("home"),
            data: root.join("data"),
            bin: root.join("bin"),
            register: root.join("checkouts.toml"),
            root,
        };
        for place in [&world.home, &world.data, &world.bin] {
            fs::create_dir_all(place).expect("the world stands");
        }
        world
    }

    /// The world's `PATH`: its own bin first, where stubs stand, the system's folders after.
    fn path(&self) -> String {
        format!("{}:/usr/bin:/bin", self.bin.display())
    }
}

impl Drop for World {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

/// The machine's register over these paths, written as one `checkouts` list.
pub fn register(world: &World, entries: &[String]) {
    let lines: Vec<String> = entries
        .iter()
        .map(|entry| format!("  \"{entry}\","))
        .collect();
    fs::write(
        &world.register,
        format!("checkouts = [\n{}\n]\n", lines.join("\n")),
    )
    .expect("the register is written");
}

/// A registered checkout as an application: its folder and its manifest. The readings stand
/// once a verb writes them.
pub fn application(world: &World, dir_name: &str, manifest: &str) -> PathBuf {
    let checkout = world.root.join(dir_name);
    fs::create_dir_all(&checkout).expect("the checkout stands");
    fs::write(checkout.join(bmo_app_declare::MANIFEST), manifest).expect("the manifest is written");
    checkout
}

/// An application under its own name, as the manifest's name matches the test's.
pub fn named_application(world: &World, name: &str) -> PathBuf {
    let manifest = MANIFEST.replace("qa-applet", name);
    application(world, name, &manifest)
}

/// Stands an executable shell script at `place` answering with `code`.
pub fn stub_script(place: &Path, body: &str) {
    fs::create_dir_all(place.parent().expect("the script has a folder"))
        .expect("the script's folder stands");
    fs::write(place, format!("#!/bin/sh\n{body}\n")).expect("the script is written");
    fs::set_permissions(place, fs::Permissions::from_mode(0o755)).expect("the script runs");
}

/// A stand-in SDK checkout: the scaffold stub records the words it was sent, one a line, and
/// ends with `code`.
pub fn fake_sdk(world: &World, code: i32) -> (PathBuf, PathBuf) {
    let sdk = world.root.join("sdk");
    fs::create_dir_all(sdk.join("bin")).expect("the sdk stands");
    fs::write(sdk.join("Cargo.toml"), "[workspace]\nmembers = []\n")
        .expect("the manifest is written");
    let record = world.root.join("scaffold-args.txt");
    stub_script(
        &sdk.join("bin/new-app.sh"),
        &format!(
            "printf '%s\\n' \"$@\" > '{}'\nexit {code}",
            record.display()
        ),
    );
    (sdk, record)
}

/// A run of the command and what it left behind.
pub struct Run {
    pub code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
}

/// The command run with these words inside `place`, in the world's element.
pub fn run(words: &[&str], world: &World, place: &Path) -> Run {
    run_with(words, world, place, &[])
}

/// The command run with these words, with further environment over the world's own.
pub fn run_with(words: &[&str], world: &World, place: &Path, extra: &[(&str, PathBuf)]) -> Run {
    let mut command = Command::new(env!("CARGO_BIN_EXE_bmo-app"));
    command
        .args(words)
        .current_dir(place)
        .env("HOME", &world.home)
        .env("BMO_DATA_DIR", &world.data)
        .env("BMO_TESTBED_CHECKOUTS", &world.register)
        .env_remove("BMO_APP_SDK")
        .env("PATH", world.path());
    for (name, value) in extra {
        command.env(name, value);
    }
    let answered = command.output().expect("the command runs");
    Run {
        code: answered.status.code(),
        stdout: String::from_utf8_lossy(&answered.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&answered.stderr).into_owned(),
    }
}
