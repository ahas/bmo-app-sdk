//! The application manifest `bmo-app.toml` and what it stands for: one declaration per
//! application whose generated readings are that repository's `testbed.toml`,
//! `install.toml`, systemd units, removal script and web scaffold, as the repository's
//! `docs/manifest.md` states.
//!
//! The manifest holds only what differs per application; everything a generated file says
//! beyond the manifest is the mechanism every application's says alike. An application
//! belongs to one family (docs/manifest.md): a web half, a console half, or the service
//! alone, and the family decides what the readings carry.

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

fn default_bundle() -> String {
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
    fn dir(&self) -> String {
        self.dir
            .clone()
            .unwrap_or_else(|| format!("tools/{}", self.name))
    }

    fn binary<'a>(&'a self, application: &'a str) -> &'a str {
        self.binary.as_deref().unwrap_or(application)
    }

    fn service(&self) -> &str {
        self.service.as_deref().unwrap_or(&self.name)
    }

    fn capability(&self) -> &str {
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

    fn check(&self) -> Result<(), String> {
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

    fn home_paths(&self) -> (String, String) {
        (
            self.deploy_root.replace("~/", "%h/"),
            self.data_root.replace("~/", "%h/"),
        )
    }

    fn env_lines(&self, extra: &BTreeMap<String, String>) -> String {
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
    fn settings_condition(&self) -> Option<String> {
        self.console.as_ref()?;
        let first = self.settings.first()?;
        Some(format!(
            "{}/{}",
            self.data_root.replace("~/", "%h/"),
            first.file
        ))
    }
}

/// One generated file: its path in the repository, its content, and whether it is run as a
/// script.
#[derive(Debug, PartialEq, Eq)]
pub struct Generated {
    pub path: String,
    pub content: String,
    pub executable: bool,
}

const GENERATED_HEAD: &str = concat!(
    "# Generated from bmo-app.toml by bmo-app-sdk's bmo-app-declare.\n",
    "# Change the manifest and run `bmo-app-declare bmo-app.toml` again; do not hand-edit this file.\n",
);

/// Every file the manifest stands for, in the order they are written.
pub fn render(manifest: &Manifest) -> Vec<Generated> {
    let mut files = vec![
        Generated {
            path: "testbed.toml".into(),
            content: render_testbed(manifest),
            executable: false,
        },
        Generated {
            path: "install.toml".into(),
            content: render_install(manifest),
            executable: false,
        },
        Generated {
            path: format!("deploy/{}", manifest.service_unit()),
            content: render_service_unit(manifest),
            executable: false,
        },
        Generated {
            path: "scripts/remove.sh".into(),
            content: render_remove(manifest),
            executable: true,
        },
    ];
    if let Some(web) = &manifest.web {
        files.push(Generated {
            path: format!(
                "deploy/{}",
                manifest.pages_unit().expect("a web app has a pages unit")
            ),
            content: render_pages_unit(manifest, web),
            executable: false,
        });
        files.push(Generated {
            path: "web/scripts/check.sh".into(),
            content: include_str!("../../../templates/web-check.sh").into(),
            executable: true,
        });
        files.push(Generated {
            path: "web/app-defaults.mjs".into(),
            content: render_app_defaults(manifest, web),
            executable: false,
        });
    }
    files
}

/// The testbed's declaration: what stands alike in every family, then the family's own lines.
fn render_testbed(m: &Manifest) -> String {
    let mut out = String::from(GENERATED_HEAD);
    let _ = write!(
        out,
        "\ntestbed = 0\nname = {name}\n\n# The shared base whose generator and skeletons this repository stands on.\nupstreams = [\"bmo-app-sdk\"]\n\n[test]\n",
        name = quote(&m.name)
    );
    out.push_str(match m.family() {
        Family::Web(_) => "languages = [\"cargo\", \"bun\", \"browser\"]\n",
        Family::Console(_) => "languages = [\"cargo\", \"bun\"]\n",
        Family::Plain => "languages = [\"cargo\"]\n",
    });
    if !m.test_setup.is_empty() {
        out.push_str("# What a fresh tree lacks, laid before any unit runs.\nsetup = [");
        out.push_str(
            &m.test_setup
                .iter()
                .map(|line| quote(line))
                .collect::<Vec<_>>()
                .join(", "),
        );
        out.push_str("]\n");
    }
    match m.family() {
        Family::Web(_) => out.push_str(
            "\n[projects]\n# The web app is a project of its own, its locked packages read by its own check script.\ndiscover = false\npaths = [\"web\"]\n",
        ),
        Family::Console(console) => {
            let _ = write!(
                out,
                "\n[projects]\n# The console bundle is a project of its own, its manifest and lockfile outside any package workspace.\npaths = [{}]\n",
                quote(&console.project)
            );
            if let Some(script) = &console.check_script {
                let _ = writeln!(out, "check_script = {}", quote(script));
            }
        }
        Family::Plain => {}
    }
    let console_tool = match m.family() {
        Family::Console(_) => m.tool.as_ref(),
        _ => None,
    };
    if let Some(tool) = console_tool {
        let _ = write!(
            out,
            "\n# The tool the service places on the tool host.\n[tools]\ndir = {}\n",
            quote(tool_root(&tool.dir()))
        );
    }
    out.push_str(&render_check(m));
    let mut binaries = vec![quote(&m.name)];
    if let Some(tool) = console_tool
        && let Some(binary) = &tool.binary
    {
        // The tool's own binary builds beside the service's.
        binaries.push(quote(binary));
    }
    let _ = write!(out, "\n[build]\nbinaries = [{}]\n", binaries.join(", "));
    match m.family() {
        Family::Web(_) => out.push_str(
            "\n# The web app, built in the build tree and copied into the release being built as its\n\
             # site folder: the release beside the build tree, named by the first twelve hex digits\n\
             # of the commit.\n\
             [[build.step]]\n\
             id = \"site\"\n\
             dir = \"web\"\n\
             command = \"export PATH=\\\"$HOME/.bun/bin:/usr/local/bin:$PATH\\\" && bun install --frozen-lockfile && bun run build\"\n\n\
             [[build.step]]\n\
             id = \"place\"\n\
             command = \"site=\\\"$(git rev-parse --show-toplevel)/../deploy-releases/$(git rev-parse HEAD | cut -c1-12)/site\\\" && mkdir -p \\\"$site\\\" && cp -a web/.output/. \\\"$site/\\\"\"\n",
        ),
        Family::Console(console) if console.built == Built::Build => {
            let place = format!(
                "{}=\"$(git rev-parse --show-toplevel)/../deploy-releases/$(git rev-parse HEAD | cut -c1-12)/console\" {}",
                console.release_out.as_deref().unwrap_or_default(),
                console.bundle
            );
            let _ = write!(
                out,
                "\n# The console bundle, installed and built in the build tree and written into the\n\
                 # release being built, where the service reads its console folder from.\n\
                 [[build.step]]\n\
                 id = \"console-install\"\n\
                 dir = {}\n\
                 command = {}\n\n\
                 [[build.step]]\n\
                 id = \"console-bundle\"\n\
                 dir = {}\n\
                 lane = \"wide\"\n\
                 command = {}\n",
                quote(&console.project),
                quote(&console.install),
                quote(&console.project),
                quote(&place)
            );
        }
        _ => {}
    }
    out.push_str(&render_deploy(m));
    out
}

/// The parent of a tool's folder, the root the testbed's `[tools]` section names.
fn tool_root(dir: &str) -> &str {
    dir.rsplit_once('/').map_or(dir, |(root, _)| root)
}

/// The static-check section: the sweeps, the ledgers, the base libraries and the family's
/// own steps.
fn render_check(m: &Manifest) -> String {
    let mut out = String::from("\n[check]\n");
    let mut quick: Vec<&str> = Vec::new();
    if m.korean_ledger.is_some() {
        quick.push("korean");
    }
    if matches!(m.family(), Family::Console(_)) {
        quick.extend(["layout", "dependencies", "cargo-profile"]);
    }
    if m.shared_versions.is_some() {
        quick.push("shared-versions");
    }
    if !m.personal_data_allowed.is_empty() {
        quick.push("personal-data");
    }
    if !quick.is_empty() {
        let _ = write!(
            out,
            "# The sweeps that read the tree before anything runs.\nquick = [{}]\n",
            quick
                .iter()
                .map(|name| quote(name))
                .collect::<Vec<_>>()
                .join(", ")
        );
    }
    if !m.personal_data_allowed.is_empty() {
        let _ = write!(
            out,
            "# The addresses the personal-data rule allows.\npersonal_data_allowed = [{}]\n",
            m.personal_data_allowed
                .iter()
                .map(|address| quote(address))
                .collect::<Vec<_>>()
                .join(", ")
        );
    }
    if let Some(ledger) = &m.korean_ledger {
        let _ = write!(
            out,
            "# The debt of Korean left in a code file; a file it does not name is allowed none.\nkorean_ledger = {}\n",
            quote(ledger)
        );
    }
    let mut libraries = vec![quote("bmo-app-service")];
    libraries.extend(m.base_libraries.iter().map(|name| quote(name)));
    let _ = write!(
        out,
        "# The base library the service crate takes by a path beside this checkout.\nbase_libraries = [{}]\n",
        libraries.join(", ")
    );
    if let Some(repository) = &m.shared_versions {
        let _ = write!(
            out,
            "\n# The repository whose root holds the shared versions the manifests and the lock hold to.\n[check.shared_versions]\nrepository = {}\n",
            quote(repository)
        );
    }
    if let Family::Console(console) = m.family()
        && console.typecheck
    {
        let _ = write!(
            out,
            "\n# The console bundle is typechecked against its own install, as its check script does.\n[[check.step]]\nid = \"{}-typecheck\"\ndir = {}\ncommand = {}\nwhen = [{}]\n",
            console.project,
            quote(&console.project),
            quote(&format!("{} && bun run typecheck", console.install)),
            quote(&format!("{}/**", console.project))
        );
    }
    out
}

/// The deploy section and the family's steps, in the order they stand in the reading.
fn render_deploy(m: &Manifest) -> String {
    let mut out = String::from(
        "\n# The service's deploy tree and its data, kept apart from every release.\n[deploy]\n",
    );
    let _ = write!(
        out,
        "root = {}\nlink = \"deploy\"\ndata_root = {}\n",
        quote(&m.deploy_root),
        quote(&m.data_root)
    );
    if matches!(m.family(), Family::Console(_))
        && let Some(first) = m.settings.first()
    {
        let _ = write!(
            out,
            "# The deployed settings each release links beside its own.\nenv_links = [{{ link = \"settings.local.toml\", target = {} }}]\n",
            quote(&first.file)
        );
    }
    let _ = write!(
        out,
        "units = [\"deploy/*.service\"]\nhealth = {}\n",
        quote(&format!("{}/deploy/bin/{} health", m.deploy_root, m.name))
    );
    let Family::Console(console) = m.family() else {
        return out;
    };
    if console.reach {
        let _ = write!(
            out,
            "\n# The screen the service brings to a console, whose face the deployment probes once\n\
             # the service is up.\n[deploy.console]\nreach = {}\n",
            quote(&format!("bin/{} console-reach", m.name))
        );
    }
    if console.built == Built::Deploy {
        let _ = write!(
            out,
            "\n# The console bundle the service serves from the release: its locked packages laid\n\
             # and its folder built inside the release, where the settings' console folder reads it.\n\
             [[deploy.step]]\nid = \"console-bundle\"\nphase = \"before-flip\"\ncommand = '''\nset -eu\ncd {}\n{}\n{}\n'''\n",
            console.project, console.install, console.bundle
        );
    }
    if console.publish {
        out.push_str(
            "\n# The app the console screen is, published on the delegation chain once the service\n\
             # answers ready, from the declaration the release carries.\n\
             [[deploy.step]]\nid = \"publish-app\"\nphase = \"after-health\"\ncommand = '''\nset -eu\nbmo pocket app deploy/app.toml\n'''\n",
        );
    }
    let Some(tool) = m.tool.as_ref() else {
        return out;
    };
    let dir = tool.dir();
    if tool.env {
        let _ = write!(
            out,
            "\n# The env file the tool and the other units read the service's address and token from.\n[[deploy.step]]\nid = \"tool-env\"\nphase = \"before-flip\"\ncommand = {}\n",
            quote(&format!("bin/{} tool-env", m.name))
        );
    }
    let _ = write!(
        out,
        "\n# The tool placed in its folder's .placed/, the folder the tool host reads: the executable\n\
         # the release's bin/ holds, its manifest, its manual and the schemas beside it.\n\
         [[deploy.step]]\nid = \"place-{}-tool\"\nphase = \"before-flip\"\ncommand = '''\nset -eu\nrm -rf {d}/.placed\nmkdir -p {d}/.placed\ncp bin/{binary} {d}/tool.toml {d}/manual.md {d}/*.json {d}/.placed/\n'''\n",
        tool.name,
        binary = tool.binary(&m.name),
        d = dir
    );
    let _ = write!(
        out,
        "\n# The placed tool registered with the tool host under this service's name, carrying the\n\
         # capability its entries ask for.\n[[deploy.step]]\nid = \"register-{name}-tool\"\nphase = \"after-health\"\ncommand = '''\nset -eu\nbmo pocket put {dir}/.placed --placed --service {service} --allow {capability}\n'''\n",
        name = tool.name,
        service = tool.service(),
        capability = tool.capability(),
        dir = dir
    );
    out
}

fn render_install(m: &Manifest) -> String {
    let mut out = String::from(GENERATED_HEAD);
    let _ = write!(
        out,
        "\n# What installing this application needs on a machine, read by the testbed's install verb.\n\
         install = 1\n\nrestart = \"systemctl --user restart {}\"\n\n[[needs]]\nprogram = \"cargo\"\n",
        m.units_line()
    );
    let builds_tool = matches!(m.family(), Family::Console(_))
        && m.tool.as_ref().is_some_and(|tool| tool.binary.is_some());
    out.push_str(if builds_tool {
        "what = \"Builds the service and its tool; the Rust toolchain from https://rustup.rs.\"\n"
    } else {
        "what = \"Builds the service; the Rust toolchain from https://rustup.rs.\"\n"
    });
    if !matches!(m.family(), Family::Console(_)) {
        out.push_str(
            "\n[[needs]]\nprogram = \"cc\"\nwhat = \"The C toolchain Cargo links the binary with; the build-essential package.\"\n",
        );
    }
    if matches!(m.family(), Family::Web(_) | Family::Console(_)) {
        out.push_str("\n[[needs]]\nprogram = \"bun\"\n");
        out.push_str(match m.family() {
            Family::Web(_) =>
                "what = \"Runs the page server and builds the web app; the Bun runtime from https://bun.sh.\"\n",
            _ => "what = \"Builds the console bundle at every release; the Bun runtime from https://bun.sh.\"\n",
        });
    }
    let _ = write!(
        out,
        "\n[[port]]\nname = {}\nnumber = {}\n",
        quote(&m.port_key()),
        m.service_port
    );
    if let Some(web) = &m.web {
        let _ = write!(
            out,
            "\n[[port]]\nname = \"{}.pages\"\nnumber = {}\n",
            m.name, web.pages_port
        );
    }
    for input in &m.input {
        let _ = write!(
            out,
            "\n[[input]]\nname = {}\nfile = {}\nwhat = {}\n",
            quote(&input.name),
            quote(&input.file),
            quote(&input.what)
        );
        if input.secret {
            out.push_str("secret = true\n");
        }
        if input.optional {
            out.push_str("optional = true\n");
        }
        if let Some(default) = &input.default {
            let _ = writeln!(out, "default = {}", quote(default));
        }
    }
    for token in &m.token {
        let _ = write!(
            out,
            "\n[[token]]\nname = {}\nfile = {}\n",
            quote(&token.name),
            quote(&token.file)
        );
        if let Some(bytes) = token.bytes {
            let _ = writeln!(out, "bytes = {bytes}");
        }
        if let Some(encoding) = &token.encoding {
            let _ = writeln!(out, "encoding = {}", quote(encoding));
        }
    }
    for settings in &m.settings {
        let text = settings.text.trim_end_matches('\n');
        let _ = write!(
            out,
            "\n# The deployed settings the release links beside its own.\n[[settings]]\nfile = {}\ntext = \"\"\"\n{}\n\"\"\"\n",
            quote(&settings.file),
            text
        );
    }
    for line in &m.line {
        let _ = write!(
            out,
            "\n[[line]]\nfile = {}\nkey = {}\nvalue = {}\n",
            quote(&line.file),
            quote(&line.key),
            quote(&line.value)
        );
    }
    out.push_str(&pass_through("skill", &m.skill));
    out.push_str(&pass_through("memory", &m.memory));
    out
}

/// A declared table family written on as it was read: the install declaration grows its
/// readers elsewhere (docs/manifest.md), and the generator only carries what stands.
fn pass_through(name: &str, items: &[toml::Value]) -> String {
    let mut out = String::new();
    for item in items {
        let _ = write!(
            out,
            "\n[[{name}]]\n{}",
            toml::to_string(item).unwrap_or_default()
        );
    }
    out
}

fn render_service_unit(m: &Manifest) -> String {
    let (deploy_home, _) = m.home_paths();
    let mut out = format!(
        "# Generated from bmo-app.toml by bmo-app-sdk's bmo-app-declare.\n\
         # The {name} service, run from its deploy tree.\n\
         [Unit]\nDescription={name}: {description}\n",
        name = m.name,
        description = m.description,
    );
    match &m.pages_unit() {
        Some(pages) => {
            let _ = write!(out, "After=network-online.target {pages}\nWants={pages}\n");
        }
        None => {
            out.push_str("After=network-online.target\n");
            if let Some(condition) = m.settings_condition() {
                let _ = write!(
                    out,
                    "# Without its deployed settings the service has no credential, so it does not\n\
                     # start where the deployment wrote none.\nConditionPathExists={condition}\n"
                );
            }
        }
    }
    let prefix = &m.env_prefix;
    let _ = write!(
        out,
        "\n[Service]\nType=simple\nWorkingDirectory={deploy_home}/deploy\n",
    );
    match m.family() {
        Family::Web(web) => {
            let _ = write!(
                out,
                "Environment={prefix}_LISTEN={listen}\nEnvironment={prefix}_DATA_DIR={data}\n\
                 Environment={prefix}_SITE_DIR={deploy_home}/deploy/site/public\n\
                 Environment={prefix}_PAGES_URL=http://127.0.0.1:{pages}\n\
                 Environment={prefix}_PUBLIC_URL={public}\n",
                listen = m.listen(),
                data = m.data_root.replace("~/", "%h/"),
                pages = web.pages_port,
                public = m.public_url.as_deref().unwrap_or_default(),
            );
        }
        Family::Plain => {
            let _ = write!(
                out,
                "Environment={prefix}_LISTEN={listen}\nEnvironment={prefix}_DATA_DIR={data}\n",
                listen = m.listen(),
                data = m.data_root.replace("~/", "%h/"),
            );
        }
        Family::Console(_) => {}
    }
    out.push_str(&m.env_lines(&m.service_env));
    let _ = write!(
        out,
        "ExecStart={deploy_home}/deploy/bin/{} serve\nRestart=always\nRestartSec=2\n",
        m.name,
    );
    if matches!(m.family(), Family::Console(_)) {
        out.push_str(
            "# A stop waits for the service to end on its own, however long its open streams and\n\
             # in-hand requests take; it is never escalated to a kill.\nTimeoutStopSec=infinity\n",
        );
    }
    let _ = write!(
        out,
        "\n[Install]\nWantedBy=default.target\n\
         # The repository's one service: stopped, restarted and watched by every deployment, which waits on its health.\n\
         X-BMO-Deploy=bot\n",
    );
    out
}

fn render_pages_unit(m: &Manifest, web: &Web) -> String {
    let (deploy_home, _) = m.home_paths();
    let mut out = format!(
        "# Generated from bmo-app.toml by bmo-app-sdk's bmo-app-declare.\n\
         # The {name} page server: the web app rendered on the server, run by Bun on the loopback,\n\
         # every page handed to it by {name}.service.\n\
         [Unit]\nDescription={name} pages: the server-rendered web app behind {name}.service\n\
         After=network-online.target\n\n[Service]\nType=simple\nWorkingDirectory={deploy_home}/deploy/site\n\
         Environment=HOST=127.0.0.1\nEnvironment=PORT={pages}\nEnvironment=NUXT_API_BASE=http://127.0.0.1:{service}\n\
         Environment=NUXT_PUBLIC_SITE_URL={public}\n",
        name = m.name,
        pages = web.pages_port,
        service = m.service_port,
        public = m.public_url.as_deref().unwrap_or_default(),
    );
    out.push_str(&m.env_lines(&m.pages_env));
    let _ = write!(
        out,
        "ExecStart=%h/.bun/bin/bun {deploy_home}/deploy/site/server/index.mjs\n\
         Restart=always\nRestartSec=2\n\n[Install]\nWantedBy=default.target\n\
         # Restarted by every deployment just before the repository's one service, which hands it every page.\n\
         X-BMO-Deploy=restart\n",
    );
    out
}

fn render_app_defaults(m: &Manifest, web: &Web) -> String {
    include_str!("../../../templates/app-defaults.mjs")
        .replace("@@name@@", &m.name)
        .replace(
            "@@public_url@@",
            m.public_url.as_deref().unwrap_or_default(),
        )
        .replace(
            "@@api_base@@",
            &format!("http://127.0.0.1:{}", m.service_port),
        )
        .replace("@@dev_port@@", &web.dev_port.to_string())
}

fn render_remove(m: &Manifest) -> String {
    include_str!("../../../templates/remove.sh")
        .replace("@@name@@", &m.name)
        .replace("@@units@@", &m.units_line())
        .replace(
            "@@tool@@",
            m.tool.as_ref().map_or("", |tool| tool.name.as_str()),
        )
        .replace("@@data_folder@@", m.data_folder.as_deref().unwrap_or(""))
        .replace("@@checkout@@", m.checkout.as_deref().unwrap_or(""))
        .replace("@@deploy_root@@", &m.deploy_root.replace("~/", "$HOME/"))
        .replace("@@data_root@@", &m.data_root.replace("~/", "$HOME/"))
}

/// A Rust string as a TOML basic string.
fn quote(value: &str) -> String {
    let mut out = String::from('"');
    for ch in value.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\t' => out.push_str("\\t"),
            '\r' => out.push_str("\\r"),
            _ => out.push(ch),
        }
    }
    out.push('"');
    out
}
