//! The manifest as the generator reads it: its fields, its families and what each tool and input declares
//! (docs/manifest.md).

use std::collections::BTreeMap;
use std::fmt::Write as _;

use serde::Deserialize;

/// The manifest file's name at a repository's root.
pub const MANIFEST: &str = "bmo-app.toml";

/// One application's declaration, read from its `bmo-app.toml`.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    /// The major of this schema the manifest is written for: 1.
    pub app: u32,
    /// The application's name, the small alphabet a URL and a unit name take without quoting.
    pub name: String,
    /// One English sentence on what the application is, for unit descriptions and manifests.
    pub description: String,
    /// The prefix its environment variables carry, in capitals.
    pub env_prefix: String,
    /// The directory holding the deploy tree's link and its releases.
    pub deploy_root: String,
    /// The runtime data, kept apart from every release; a console application shares the
    /// machine's root and stands in its `data_folder` under it.
    pub data_root: String,
    /// The address the service binds: `127.0.0.1` where none is named.
    #[serde(default)]
    pub listen_host: Option<String>,
    /// The port the service answers on.
    pub service_port: u16,
    /// The address people reach the application at; required where there is a web app.
    #[serde(default)]
    pub public_url: Option<String>,
    /// The web app, present where the application brings one.
    #[serde(default)]
    pub web: Option<Web>,
    /// The console bundle, present where the application screens through the console.
    #[serde(default)]
    pub console: Option<Console>,
    /// Values only a person has, passed into the install declaration.
    #[serde(default)]
    pub input: Vec<Input>,
    /// Values the install generates, passed into the install declaration.
    #[serde(default)]
    pub token: Vec<Token>,
    /// Settings files the install writes, passed into the install declaration.
    #[serde(default)]
    pub settings: Vec<SettingsFile>,
    /// Lines the install keeps in files under the data root.
    #[serde(default)]
    pub line: Vec<Line>,
    /// Extra `Environment` lines for the service unit, as written; `~/` reads as the home.
    #[serde(default)]
    pub service_env: BTreeMap<String, String>,
    /// Extra `Environment` lines for the page server unit, as written.
    #[serde(default)]
    pub pages_env: BTreeMap<String, String>,
    /// The application's hosted tool, where it ships one.
    #[serde(default)]
    pub tool: Option<Tool>,
    /// The application's skills, passed into the install declaration (docs/manifest.md).
    #[serde(default)]
    pub skill: Vec<toml::Value>,
    /// The application's memories, passed into the install declaration (docs/manifest.md).
    #[serde(default)]
    pub memory: Vec<toml::Value>,
    /// The base libraries the crate takes besides the skeleton, into the testbed's check.
    #[serde(default)]
    pub base_libraries: Vec<String>,
    /// The repository holding the shared versions the manifests and the lock hold to.
    #[serde(default)]
    pub shared_versions: Option<String>,
    /// The ledger of the Korean left in code files, where the korean sweep runs against it.
    #[serde(default)]
    pub korean_ledger: Option<String>,
    /// The addresses the personal-data rule allows, where the personal-data sweep runs.
    #[serde(default)]
    pub personal_data_allowed: Vec<String>,
    /// Lines the suite runs on a fresh tree before any unit.
    #[serde(default)]
    pub test_setup: Vec<String>,
    /// The name the install registers the service port under; `<name>.server` where none.
    #[serde(default)]
    pub port_name: Option<String>,
    /// The folder under the data root holding this application's own data, where the root is
    /// the machine's shared one. Removal takes the folder and never the shared root.
    #[serde(default)]
    pub data_folder: Option<String>,
    /// Where the repository's checkout stands on the machine, the line removal takes from
    /// the checkout register.
    #[serde(default)]
    pub checkout: Option<String>,
}

/// The web app half of an application: a page server beside the service.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Web {
    /// The port the page server answers on, on the loopback alone.
    pub pages_port: u16,
    /// The port a person's `nuxi dev` takes.
    pub dev_port: u16,
}

/// Where the console bundle is built: inside the release before the flip, or during the
/// build and written into the release being built.
#[derive(Debug, Default, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Built {
    #[default]
    Deploy,
    Build,
}

/// The console half of an application: the bundle the service serves into the console.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Console {
    /// The folder holding the bundle's sources, outside any package workspace.
    pub project: String,
    /// The command that lays the bundle's locked packages.
    pub install: String,
    /// The command that builds the bundle.
    #[serde(default = "default_bundle")]
    pub bundle: String,
    /// Where the bundle is built.
    #[serde(default)]
    pub built: Built,
    /// For `built = "build"`: the variable naming where in the release the bundle goes.
    #[serde(default)]
    pub release_out: Option<String>,
    /// The project's check script, where the testbed's declaration names it.
    #[serde(default)]
    pub check_script: Option<String>,
    /// The bundle is typechecked as a static step too.
    #[serde(default)]
    pub typecheck: bool,
    /// The service answers `console-reach` and the deployment probes the console face.
    #[serde(default)]
    pub reach: bool,
    /// The screen is an app on the delegation chain, published from the release.
    #[serde(default)]
    pub publish: bool,
}

pub(crate) fn default_bundle() -> String {
    "bun run bundle".into()
}

/// A value the install generates and keeps under the data root.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Token {
    pub name: String,
    pub file: String,
    #[serde(default)]
    pub bytes: Option<u64>,
    #[serde(default)]
    pub encoding: Option<String>,
}

/// A settings file the install writes from its declared text whole.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SettingsFile {
    pub file: String,
    pub text: String,
}

/// One line the install keeps in a file under the data root.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Line {
    pub file: String,
    pub key: String,
    pub value: String,
}

/// The application's hosted tool (docs/manifest.md).
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Tool {
    /// The registered name, what the tool host and the removal find it by.
    pub name: String,
    /// The folder holding its manifest, manual and schemas; `tools/<name>` where none.
    #[serde(default)]
    pub dir: Option<String>,
    /// The executable placed for it; the service's own binary where none.
    #[serde(default)]
    pub binary: Option<String>,
    /// The label it registers under; its name where none.
    #[serde(default)]
    pub service: Option<String>,
    /// The capability its entries ask; `room.tools` where none.
    #[serde(default)]
    pub capability: Option<String>,
    /// The deployment runs the service's `tool-env` verb before the flip.
    #[serde(default)]
    pub env: bool,
}

impl Tool {
    pub(crate) fn dir(&self) -> String {
        self.dir
            .clone()
            .unwrap_or_else(|| format!("tools/{}", self.name))
    }

    pub(crate) fn binary<'a>(&'a self, application: &'a str) -> &'a str {
        self.binary.as_deref().unwrap_or(application)
    }

    pub(crate) fn service(&self) -> &str {
        self.service.as_deref().unwrap_or(&self.name)
    }

    pub(crate) fn capability(&self) -> &str {
        self.capability.as_deref().unwrap_or("room.tools")
    }
}

/// A value only a person has, declared the way the install declaration declares it.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Input {
    pub name: String,
    /// Where it is kept under the data root.
    pub file: String,
    /// What it is and where a person finds it, in English.
    pub what: String,
    #[serde(default)]
    pub secret: bool,
    #[serde(default)]
    pub optional: bool,
    #[serde(default)]
    pub default: Option<String>,
}

/// The family a manifest declares, which decides what the readings carry.
#[derive(Debug, PartialEq, Eq)]
pub enum Family<'a> {
    Web(&'a Web),
    Console(&'a Console),
    Plain,
}

impl Manifest {
    /// Reads a manifest from text, refusing a file that does not hold together.
    pub fn parse(text: &str) -> Result<Self, String> {
        let manifest: Manifest = toml::from_str(text).map_err(|error| error.to_string())?;
        manifest.check()?;
        Ok(manifest)
    }

    pub(crate) fn check(&self) -> Result<(), String> {
        if self.app != 1 {
            return Err(format!(
                "app = {} is not schema major 1, which is all this generator reads",
                self.app
            ));
        }
        if self.name.is_empty()
            || !self
                .name
                .bytes()
                .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
        {
            return Err(format!(
                "name {:?} is not in the small alphabet every reader finds it by",
                self.name
            ));
        }
        if self.env_prefix.is_empty()
            || !self
                .env_prefix
                .bytes()
                .all(|byte| byte.is_ascii_uppercase() || byte.is_ascii_digit() || byte == b'_')
        {
            return Err(format!("env_prefix {:?} is not capitals", self.env_prefix));
        }
        for root in [&self.deploy_root, &self.data_root] {
            if !root.starts_with("~/") && !root.starts_with('/') {
                return Err(format!(
                    "root {root:?} stands under neither the home nor the root"
                ));
            }
        }
        match (&self.web, &self.console) {
            (Some(_), Some(_)) => {
                return Err(
                    "an application declares at most one half: [web] and [console] together is none's family"
                        .into(),
                );
            }
            (Some(web), None) => {
                if web.pages_port == self.service_port || web.dev_port == self.service_port {
                    return Err("every port an application stands on is its own".into());
                }
                if self.public_url.is_none() {
                    return Err("a web app needs public_url, the address people reach it at".into());
                }
            }
            (None, Some(console)) => {
                if console.project.is_empty() || console.install.is_empty() {
                    return Err(
                        "a console half names its project and the command that lays its packages"
                            .into(),
                    );
                }
                if console.built == Built::Build && console.release_out.is_none() {
                    return Err(
                        "a bundle built during the build names release_out, the variable the release's place travels in".into(),
                    );
                }
            }
            _ => {}
        }
        if let Some(tool) = &self.tool
            && (tool.name.is_empty()
                || !tool
                    .name
                    .bytes()
                    .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-'))
        {
            return Err(format!(
                "tool name {:?} is not in the small alphabet the tool host files it by",
                tool.name
            ));
        }
        if let Some(port_name) = &self.port_name
            && (!port_name.ends_with(".server") || port_name.len() <= ".server".len())
        {
            return Err(format!(
                "port_name {port_name:?} is not a name of the shape <key>.server"
            ));
        }
        Ok(())
    }

    pub fn family(&self) -> Family<'_> {
        match (&self.web, &self.console) {
            (Some(web), None) => Family::Web(web),
            (None, Some(console)) => Family::Console(console),
            _ => Family::Plain,
        }
    }

    pub fn service_unit(&self) -> String {
        format!("{}.service", self.name)
    }

    pub fn pages_unit(&self) -> Option<String> {
        self.web
            .as_ref()
            .map(|_| format!("{}-pages.service", self.name))
    }

    /// The address the service binds.
    pub fn listen(&self) -> String {
        format!(
            "{}:{}",
            self.listen_host
                .clone()
                .unwrap_or_else(|| "127.0.0.1".into()),
            self.service_port
        )
    }

    /// The name the install registers the service port under.
    pub fn port_key(&self) -> String {
        self.port_name
            .clone()
            .unwrap_or_else(|| format!("{}.server", self.name))
    }

    pub(crate) fn home_paths(&self) -> (String, String) {
        (
            self.deploy_root.replace("~/", "%h/"),
            self.data_root.replace("~/", "%h/"),
        )
    }

    pub(crate) fn env_lines(&self, extra: &BTreeMap<String, String>) -> String {
        let mut lines = String::new();
        for (key, value) in extra {
            let _ = writeln!(lines, "Environment={key}={}", value.replace("~/", "%h/"));
        }
        lines
    }

    /// The unit names a restart takes, spaced.
    pub fn units_line(&self) -> String {
        match self.pages_unit() {
            Some(pages) => format!("{} {}", self.service_unit(), pages),
            None => self.service_unit(),
        }
    }

    /// The data root's file the console unit waits on before it starts.
    pub(crate) fn settings_condition(&self) -> Option<String> {
        if !matches!(self.family(), Family::Console(_)) {
            return None;
        }
        let first = self.settings.first()?;
        Some(format!(
            "{}/{}",
            self.data_root.replace("~/", "%h/"),
            first.file
        ))
    }
}
