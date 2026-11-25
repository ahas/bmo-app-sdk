//! The application manifest `bmo-app.toml` and what it stands for: one declaration per
//! application whose generated readings are that repository's `testbed.toml`,
//! `install.toml`, systemd units, removal script and web scaffold, as the repository's
//! `docs/manifest.md` states.
//!
//! The manifest holds only what differs per application; everything a generated file says
//! beyond the manifest is the mechanism every application's says alike.

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
    /// The runtime data, kept apart from every release.
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
    /// Values only a person has, passed into the install declaration.
    #[serde(default)]
    pub input: Vec<Input>,
    /// Extra `Environment` lines for the service unit, as written; `~/` reads as the home.
    #[serde(default)]
    pub service_env: BTreeMap<String, String>,
    /// Extra `Environment` lines for the page server unit, as written.
    #[serde(default)]
    pub pages_env: BTreeMap<String, String>,
    /// The application's hosted tool's registered name, where it registers one: its removal
    /// takes it out of the tool host before the tree goes.
    #[serde(default)]
    pub tool: Option<String>,
    /// The service's skills, passed into the install declaration (docs/manifest.md).
    #[serde(default)]
    pub skill: Vec<toml::Value>,
    /// The service's memories, passed into the install declaration (docs/manifest.md).
    #[serde(default)]
    pub memory: Vec<toml::Value>,
}

/// The web app half of an application: a page server beside the service.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Web {
    /// The port the page server answers on, on the loopback alone.
    pub pages_port: u16,
    /// The port a person's `nuxi dev` takes.
    pub dev_port: u16,
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
        if let Some(web) = &self.web {
            if self.public_url.is_none() {
                return Err("a web app needs public_url, the address people reach it at".into());
            }
            if web.pages_port == self.service_port || web.dev_port == self.service_port {
                return Err("every port an application stands on is its own".into());
            }
        }
        Ok(())
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
    if manifest.web.is_some() {
        files.push(Generated {
            path: format!(
                "deploy/{}",
                manifest.pages_unit().expect("a web app's pages unit")
            ),
            content: render_pages_unit(manifest),
            executable: false,
        });
        files.push(Generated {
            path: "web/scripts/check.sh".into(),
            content: include_str!("../../../templates/web-check.sh").into(),
            executable: true,
        });
        files.push(Generated {
            path: "web/app-defaults.mjs".into(),
            content: render_app_defaults(manifest),
            executable: false,
        });
    }
    files
}

fn render_testbed(m: &Manifest) -> String {
    let mut out = String::from(GENERATED_HEAD);
    let _ = write!(
        out,
        "\ntestbed = 0\nname = {name}\n\n# The shared base whose generator and skeletons this repository stands on.\nupstreams = [\"bmo-app-sdk\"]\n\n[test]\n",
        name = quote(&m.name)
    );
    out.push_str(match &m.web {
        Some(_) => "languages = [\"cargo\", \"bun\", \"browser\"]\n",
        None => "languages = [\"cargo\"]\n",
    });
    if m.web.is_some() {
        let _ = write!(
            out,
            "\n[projects]\n# The web app is a project of its own, its locked packages read by its own check script.\ndiscover = false\npaths = [\"web\"]\n"
        );
    }
    let _ = write!(
        out,
        "\n[check]\n# The base library the service crate takes by a path beside this checkout.\nbase_libraries = [\"bmo-app-service\"]\n"
    );
    let _ = write!(out, "\n[build]\nbinaries = [{}]\n", quote(&m.name));
    if m.web.is_some() {
        out.push_str(
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
        );
    }
    let _ = write!(
        out,
        "\n# The service's deploy tree and its data, kept apart from every release.\n\
         [deploy]\n\
         root = {}\nlink = \"deploy\"\ndata_root = {}\n\
         units = [\"deploy/*.service\"]\nhealth = {}\n",
        quote(&m.deploy_root),
        quote(&m.data_root),
        quote(&format!("{}/deploy/bin/{} health", m.deploy_root, m.name)),
    );
    out
}

fn render_install(m: &Manifest) -> String {
    let mut out = String::from(GENERATED_HEAD);
    let _ = write!(
        out,
        "\n# What installing this application needs on a machine, read by the testbed's install verb.\n\
         install = 1\n\nrestart = \"systemctl --user restart {}\"\n\n[[needs]]\nprogram = \"cargo\"\n\
         what = \"Builds the service; the Rust toolchain from https://rustup.rs.\"\n\n[[needs]]\nprogram = \"cc\"\n\
         what = \"The C toolchain Cargo links the binary with; the build-essential package.\"\n",
        m.units_line()
    );
    if m.web.is_some() {
        out.push_str(
            "\n[[needs]]\nprogram = \"bun\"\nwhat = \"Runs the page server and builds the web app; the Bun runtime from https://bun.sh.\"\n",
        );
    }
    let _ = write!(
        out,
        "\n[[port]]\nname = \"{}.server\"\nnumber = {}\n",
        m.name, m.service_port
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
    let (deploy_home, data_home) = m.home_paths();
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
        None => out.push_str("After=network-online.target\n"),
    }
    let prefix = &m.env_prefix;
    let listen = m.listen();
    let _ = write!(
        out,
        "\n[Service]\nType=simple\nWorkingDirectory={deploy_home}/deploy\n\
         Environment={prefix}_LISTEN={listen}\nEnvironment={prefix}_DATA_DIR={data_home}\n",
    );
    if let Some(web) = &m.web {
        let pages = web.pages_port;
        let public = m.public_url.as_deref().unwrap_or_default();
        let _ = write!(
            out,
            "Environment={prefix}_SITE_DIR={deploy_home}/deploy/site/public\n\
             Environment={prefix}_PAGES_URL=http://127.0.0.1:{pages}\n\
             Environment={prefix}_PUBLIC_URL={public}\n",
        );
    }
    out.push_str(&m.env_lines(&m.service_env));
    let _ = write!(
        out,
        "ExecStart={deploy_home}/deploy/bin/{} serve\nRestart=always\nRestartSec=2\n\n\
         [Install]\nWantedBy=default.target\n\
         # The repository's one service: stopped, restarted and watched by every deployment, which waits on its health.\n\
         X-BMO-Deploy=bot\n",
        m.name,
    );
    out
}

fn render_pages_unit(m: &Manifest) -> String {
    let (deploy_home, _) = m.home_paths();
    let web = m.web.as_ref().expect("a pages unit stands for a web app");
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

fn render_app_defaults(m: &Manifest) -> String {
    let web = m.web.as_ref().expect("defaults stand for a web app");
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
        .replace("@@tool@@", m.tool.as_deref().unwrap_or_default())
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
