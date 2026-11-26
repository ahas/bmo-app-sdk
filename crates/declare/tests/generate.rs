//! The manifest read whole and its readings: what stands in them, and what a manifest that
//! does not hold together never gets generated from.

use bmo_app_declare::{Manifest, render};

const WEB_MANIFEST: &str = r#"
app = 1
name = "qa-widget"
description = "A widget, kept for a test"
env_prefix = "WIDGET"
deploy_root = "~/.local/share/qa-widget"
data_root = "~/.local/share/qa-widget-data"
service_port = 7550
public_url = "https://widget.example"
service_env = { WIDGET_KEY_FILE = "~/.local/share/bmo/data/env/widget-key" }
pages_env = { NUXT_PUBLIC_GOOGLE_SITE_VERIFICATION = "" }

[web]
pages_port = 7551
dev_port = 3550

[tool]
name = "widget"

[[input]]
name = "widget-key"
file = "env/widget-key"
what = "The service key only a person has."
secret = true
optional = true

[[skill]]
name = "widget-usage"
file = "skills/widget.md"

[[memory]]
key = "widget-not-gadget"
type = "check"
text = "A widget request goes to the widget service."
"#;

/// A console application standing as the platform's console family declares itself: the
/// bundle's project and build, the install's tokens, settings and lines, a tool of its own
/// and the machine's shared data root.
const CONSOLE_MANIFEST: &str = r#"
app = 1
name = "qa-console"
description = "A console screen kept for a test"
env_prefix = "QA_CONSOLE"
deploy_root = "~/.local/share/qa-console"
data_root = "~/.local/share/bmo/data"
data_folder = "qa-console"
service_port = 7560
port_name = "console.server"
checkout = "~/repos/ahas/bmo/services/console"
base_libraries = ["bmo-console-face"]
shared_versions = "bmo-chat"
korean_ledger = "korean-ledger.json"
personal_data_allowed = ["dev@example"]
test_setup = ["cd client && bun install --frozen-lockfile"]

[console]
project = "client"
install = "bun install --frozen-lockfile"
check_script = "check"
built = "build"
release_out = "QA_CONSOLE_OUT"
typecheck = true
reach = true
publish = true

[tool]
name = "console-helper"
dir = "tools/console"
binary = "console-tool"
env = true

[[token]]
name = "console.service"
file = "env/console-service.token"

[[token]]
name = "console.secrets"
file = "env/console-secrets.key"
bytes = 32
encoding = "base64"

[[settings]]
file = "env/console.toml"
text = """
[server]
listen = "127.0.0.1:{{port.console.server}}"
token = "{{token.console.service}}"

[store]
path = "{{data_root}}/qa-console/console.db"
"""

[[line]]
file = "env/web-console.env"
key = "CONSOLE_ADDRESS"
value = "http://127.0.0.1:{{port.console.server}}"
"#;

fn generated_for(manifest: &str, path: &str) -> String {
    let manifest = Manifest::parse(manifest).expect("the manifest reads");
    render(&manifest)
        .into_iter()
        .find(|file| file.path == path)
        .unwrap_or_else(|| panic!("no reading stands at {path}"))
        .content
}

fn generated(path: &str) -> String {
    generated_for(WEB_MANIFEST, path)
}

#[test]
fn a_web_manifest_reads_the_platforms_own_declaration() {
    let testbed = generated("testbed.toml");
    let parsed: toml::Value = toml::from_str(&testbed).expect("the reading is valid TOML");
    assert_eq!(parsed["name"].as_str(), Some("qa-widget"));
    assert_eq!(
        parsed["upstreams"].as_array().map(Vec::len),
        Some(1),
        "the base stands as the application's one upstream"
    );
    let languages = parsed["test"]["languages"].as_array().expect("languages");
    for wanted in ["cargo", "bun", "browser"] {
        assert!(
            languages.iter().any(|l| l.as_str() == Some(wanted)),
            "{wanted} is tested"
        );
    }
    assert_eq!(parsed["projects"]["paths"][0].as_str(), Some("web"));
    assert_eq!(
        parsed["check"]["base_libraries"][0].as_str(),
        Some("bmo-app-service")
    );
    assert_eq!(parsed["build"]["binaries"][0].as_str(), Some("qa-widget"));
    assert_eq!(parsed["deploy"]["link"].as_str(), Some("deploy"));
    assert_eq!(
        parsed["deploy"]["health"].as_str(),
        Some("~/.local/share/qa-widget/deploy/bin/qa-widget health")
    );
    let steps = parsed["build"]["step"].as_array().expect("build steps");
    assert_eq!(steps.len(), 2, "the site build and its placement stand");
    assert!(
        parsed["deploy"].get("step").is_none(),
        "the web family carries no deploy steps of its own"
    );
}

#[test]
fn the_install_reading_carries_ports_inputs_and_the_declared_families() {
    let install = generated("install.toml");
    let parsed: toml::Value = toml::from_str(&install).expect("the reading is valid TOML");
    assert_eq!(parsed["install"].as_integer(), Some(1));
    assert_eq!(
        parsed["restart"].as_str(),
        Some("systemctl --user restart qa-widget.service qa-widget-pages.service")
    );
    let ports = parsed["port"].as_array().expect("ports");
    assert_eq!(ports[0]["name"].as_str(), Some("qa-widget.server"));
    assert_eq!(ports[0]["number"].as_integer(), Some(7550));
    assert_eq!(ports[1]["name"].as_str(), Some("qa-widget.pages"));
    assert_eq!(parsed["input"][0]["name"].as_str(), Some("widget-key"));
    assert_eq!(parsed["input"][0]["secret"].as_bool(), Some(true));
    assert_eq!(parsed["skill"][0]["name"].as_str(), Some("widget-usage"));
    assert_eq!(
        parsed["memory"][0]["key"].as_str(),
        Some("widget-not-gadget")
    );
}

#[test]
fn the_units_stand_as_the_platform_watches_them() {
    let service = generated("deploy/qa-widget.service");
    assert!(service.contains("X-BMO-Deploy=bot"));
    assert!(service.contains("Environment=WIDGET_LISTEN=127.0.0.1:7550"));
    assert!(service.contains("Environment=WIDGET_DATA_DIR=%h/.local/share/qa-widget-data"));
    assert!(
        service.contains("Environment=WIDGET_KEY_FILE=%h/.local/share/bmo/data/env/widget-key")
    );
    assert!(service.contains("ExecStart=%h/.local/share/qa-widget/deploy/bin/qa-widget serve"));
    assert!(service.contains("After=network-online.target qa-widget-pages.service"));
    assert!(
        !service.contains("TimeoutStopSec"),
        "a web unit stops by the plain rule"
    );

    let pages = generated("deploy/qa-widget-pages.service");
    assert!(pages.contains("X-BMO-Deploy=restart"));
    assert!(pages.contains("Environment=PORT=7551"));
    assert!(pages.contains("Environment=NUXT_API_BASE=http://127.0.0.1:7550"));
    assert!(pages.contains("Environment=NUXT_PUBLIC_GOOGLE_SITE_VERIFICATION="));
}

#[test]
fn the_removal_script_guards_the_roots_it_bakes() {
    let removal = generated("scripts/remove.sh");
    assert!(removal.contains("NAME=\"qa-widget\""));
    assert!(removal.contains("DEPLOY_ROOT=\"$HOME/.local/share/qa-widget\""));
    assert!(removal.contains("TOOL=\"widget\""));
    // The script is shell: its syntax is checked where it runs, so it must at least stand
    // between the guards the document states.
    assert!(removal.contains("guard \"$DEPLOY_ROOT\""));
    assert!(removal.contains("bmo pocket take $TOOL"));
    assert!(removal.contains("purge-data"));
    // A manifest naming no data folder and no checkout keeps the whole-root purge and the
    // name-keyed register line.
    assert!(removal.contains("DATA_FOLDER=\"\""));
    assert!(removal.contains("CHECKOUT=\"\""));
    assert!(removal.contains("rm -rf '$DATA_ROOT'"));
}

#[test]
fn the_web_readings_stand_inside_the_application() {
    let check = generated("web/scripts/check.sh");
    assert!(check.contains("bun install --frozen-lockfile"));
    assert!(check.contains("bun run typecheck"));
    let defaults = generated("web/app-defaults.mjs");
    assert!(defaults.contains("apiBase: \"http://127.0.0.1:7550\""));
    assert!(defaults.contains("siteUrl: \"https://widget.example\""));
    assert!(defaults.contains("port: 3550"));
    let manifest = Manifest::parse(WEB_MANIFEST).expect("the manifest reads");
    let files = render(&manifest);
    for path in [
        "testbed.toml",
        "install.toml",
        "deploy/qa-widget.service",
        "deploy/qa-widget-pages.service",
        "scripts/remove.sh",
        "web/scripts/check.sh",
        "web/app-defaults.mjs",
    ] {
        assert!(files.iter().any(|f| f.path == path), "{path} is generated");
    }
}

#[test]
fn a_service_only_application_declares_no_web_halves() {
    let manifest = Manifest::parse(
        r#"
app = 1
name = "qa-plain"
description = "No pages, just the service"
env_prefix = "PLAIN"
deploy_root = "/srv/qa-plain"
data_root = "~/.local/share/qa-plain"
service_port = 7600
"#,
    )
    .expect("the manifest reads");
    let files = render(&manifest);
    assert!(
        !files.iter().any(|f| f.path.contains("pages")),
        "no page server stands where no web half was declared"
    );
    let testbed: toml::Value = toml::from_str(
        &files
            .iter()
            .find(|f| f.path == "testbed.toml")
            .expect("the declaration")
            .content,
    )
    .expect("valid TOML");
    assert_eq!(
        testbed["test"]["languages"].as_array().map(Vec::len),
        Some(1)
    );
    assert!(testbed.get("projects").is_none());
    assert!(testbed["build"].get("step").is_none());
    let install: toml::Value = toml::from_str(
        &files
            .iter()
            .find(|f| f.path == "install.toml")
            .expect("the declaration")
            .content,
    )
    .expect("valid TOML");
    assert_eq!(install["port"].as_array().map(Vec::len), Some(1));
    let needs = install["needs"].as_array().expect("needs");
    assert!(
        !needs.iter().any(|n| n["program"].as_str() == Some("bun")),
        "a service without a web half needs no Bun"
    );
    assert!(
        files
            .iter()
            .find(|f| f.path == "scripts/remove.sh")
            .expect("the removal")
            .content
            .contains("DEPLOY_ROOT=\"/srv/qa-plain\""),
        "a /srv root is kept whole where it was declared"
    );
}

#[test]
fn a_console_manifest_reads_the_familys_own_testbed() {
    let testbed = generated_for(CONSOLE_MANIFEST, "testbed.toml");
    let parsed: toml::Value = toml::from_str(&testbed).expect("the reading is valid TOML");
    let languages = parsed["test"]["languages"].as_array().expect("languages");
    assert_eq!(
        languages
            .iter()
            .map(|v| v.as_str().unwrap())
            .collect::<Vec<_>>(),
        vec!["cargo", "bun"],
        "the console family tests Rust and Bun, no browser project"
    );
    assert_eq!(
        parsed["test"]["setup"][0].as_str(),
        Some("cd client && bun install --frozen-lockfile")
    );
    assert_eq!(parsed["projects"]["paths"][0].as_str(), Some("client"));
    assert_eq!(parsed["projects"]["check_script"].as_str(), Some("check"));
    assert_eq!(parsed["tools"]["dir"].as_str(), Some("tools"));
    let quick = parsed["check"]["quick"].as_array().expect("quick sweeps");
    assert_eq!(
        quick
            .iter()
            .map(|v| v.as_str().unwrap())
            .collect::<Vec<_>>(),
        vec![
            "korean",
            "layout",
            "dependencies",
            "cargo-profile",
            "shared-versions",
            "personal-data"
        ]
    );
    assert_eq!(
        parsed["check"]["korean_ledger"].as_str(),
        Some("korean-ledger.json")
    );
    assert_eq!(
        parsed["check"]["personal_data_allowed"][0].as_str(),
        Some("dev@example")
    );
    let libraries = parsed["check"]["base_libraries"]
        .as_array()
        .expect("libraries");
    assert_eq!(
        libraries
            .iter()
            .map(|v| v.as_str().unwrap())
            .collect::<Vec<_>>(),
        vec!["bmo-app-service", "bmo-console-face"]
    );
    assert_eq!(
        parsed["check"]["shared_versions"]["repository"].as_str(),
        Some("bmo-chat")
    );
    let steps = parsed["check"]["step"].as_array().expect("static steps");
    assert_eq!(steps[0]["id"].as_str(), Some("client-typecheck"));
    assert_eq!(
        parsed["build"]["binaries"].as_array().map(Vec::len),
        Some(2)
    );
    assert_eq!(
        parsed["deploy"]["env_links"][0]["target"].as_str(),
        Some("env/console.toml")
    );
    assert_eq!(
        parsed["deploy"]["console"]["reach"].as_str(),
        Some("bin/qa-console console-reach")
    );
    let steps = parsed["deploy"]["step"].as_array().expect("deploy steps");
    assert_eq!(
        steps
            .iter()
            .map(|s| s["id"].as_str().unwrap())
            .collect::<Vec<_>>(),
        vec![
            "publish-app",
            "tool-env",
            "place-console-helper-tool",
            "register-console-helper-tool"
        ],
        "the bundle builds during the build, so no deploy bundle step stands; publication, env, placement and registration follow"
    );
    assert_eq!(
        steps[3]["command"].as_str().unwrap(),
        "set -eu\nbmo pocket put tools/console/.placed --placed --service console-helper --allow room.tools\n"
    );
    let build_steps = parsed["build"]["step"].as_array().expect("build steps");
    assert_eq!(build_steps[0]["id"].as_str(), Some("console-install"));
    assert_eq!(build_steps[1]["id"].as_str(), Some("console-bundle"));
    assert_eq!(build_steps[1]["lane"].as_str(), Some("wide"));
    assert!(
        build_steps[1]["command"]
            .as_str()
            .unwrap()
            .starts_with("QA_CONSOLE_OUT=\"$(git rev-parse --show-toplevel)/../deploy-releases/"),
        "the bundle's place in the release travels in the variable the manifest names"
    );
}

#[test]
fn a_console_deploy_build_carries_the_bundle_before_the_flip() {
    let built_at_deploy = CONSOLE_MANIFEST.replace("built = \"build\"", "built = \"deploy\"");
    let testbed = generated_for(&built_at_deploy, "testbed.toml");
    let parsed: toml::Value = toml::from_str(&testbed).expect("valid TOML");
    assert!(
        parsed["build"].get("step").is_none(),
        "no build-time bundle pair stands"
    );
    let steps = parsed["deploy"]["step"].as_array().expect("deploy steps");
    assert_eq!(steps[0]["id"].as_str(), Some("console-bundle"));
    assert_eq!(steps[0]["phase"].as_str(), Some("before-flip"));
    assert!(
        steps[0]["command"]
            .as_str()
            .unwrap()
            .contains("cd client\nbun install --frozen-lockfile\nbun run bundle"),
        "the bundle is laid and built inside the release"
    );
}

#[test]
fn the_console_install_reading_carries_tokens_settings_and_lines() {
    let install = generated_for(CONSOLE_MANIFEST, "install.toml");
    let parsed: toml::Value = toml::from_str(&install).expect("the reading is valid TOML");
    let needs = parsed["needs"].as_array().expect("needs");
    let programs = needs
        .iter()
        .map(|n| n["program"].as_str().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(
        programs,
        vec!["cargo", "bun"],
        "the console family needs no C line"
    );
    assert!(
        needs[0]["what"].as_str().unwrap().contains("and its tool"),
        "the tool's own binary builds too"
    );
    assert_eq!(parsed["port"][0]["name"].as_str(), Some("console.server"));
    assert_eq!(parsed["port"].as_array().map(Vec::len), Some(1));
    assert_eq!(parsed["token"][0]["name"].as_str(), Some("console.service"));
    assert_eq!(parsed["token"][1]["bytes"].as_integer(), Some(32));
    assert_eq!(parsed["token"][1]["encoding"].as_str(), Some("base64"));
    assert_eq!(
        parsed["settings"][0]["file"].as_str(),
        Some("env/console.toml")
    );
    assert!(
        parsed["settings"][0]["text"]
            .as_str()
            .unwrap()
            .contains("listen = \"127.0.0.1:{{port.console.server}}\""),
        "the settings text travels whole, placeholders and all"
    );
    assert_eq!(parsed["line"][0]["key"].as_str(), Some("CONSOLE_ADDRESS"));
}

#[test]
fn a_console_unit_waits_on_its_settings_and_never_on_a_kill() {
    let unit = generated_for(CONSOLE_MANIFEST, "deploy/qa-console.service");
    assert!(
        unit.contains("ConditionPathExists=%h/.local/share/bmo/data/env/console.toml"),
        "the unit does not start without its deployed settings"
    );
    assert!(unit.contains("TimeoutStopSec=infinity"));
    assert!(
        !unit.contains("Environment=QA_CONSOLE_LISTEN"),
        "a console application reads settings files, so the unit sets no listen environment"
    );
    assert!(unit.contains("ExecStart=%h/.local/share/qa-console/deploy/bin/qa-console serve"));
    assert!(unit.contains("X-BMO-Deploy=bot"));
}

#[test]
fn a_console_removal_takes_its_folder_and_its_declared_register_line() {
    let removal = generated_for(CONSOLE_MANIFEST, "scripts/remove.sh");
    assert!(removal.contains("DATA_FOLDER=\"qa-console\""));
    assert!(removal.contains("CHECKOUT=\"~/repos/ahas/bmo/services/console\""));
    assert!(removal.contains("TOOL=\"console-helper\""));
    assert!(
        removal.contains(r#"rm -rf \"$DATA_ROOT/$DATA_FOLDER\""#),
        "the folder under the shared root is what the purge takes"
    );
    assert!(
        !removal.contains("rm -rf '$DATA_ROOT'\n"),
        "a shared data root is never taken whole by an application naming its folder"
    );
    assert!(removal.contains("${CHECKOUT#"));
}

#[test]
fn a_manifest_that_does_not_hold_together_is_refused_naming_why() {
    for (broken, wanted) in [
        (
            WEB_MANIFEST.replace("name = \"qa-widget\"", "name = \"QA Widget\""),
            "small alphabet",
        ),
        (
            WEB_MANIFEST.replace("env_prefix = \"WIDGET\"", "env_prefix = \"widget\""),
            "capitals",
        ),
        (WEB_MANIFEST.replace("app = 1", "app = 7"), "schema"),
        (
            WEB_MANIFEST.replace(
                "deploy_root = \"~/.local/share/qa-widget\"",
                "deploy_root = \"elsewhere\"",
            ),
            "home",
        ),
        (
            WEB_MANIFEST.replace("pages_port = 7551", "pages_port = 7550"),
            "its own",
        ),
        (
            WEB_MANIFEST.replace(
                "[tool]\nname = \"widget\"",
                "[console]\nproject = \"c\"\ninstall = \"i\"",
            ),
            "one half",
        ),
        (
            CONSOLE_MANIFEST.replace("release_out = \"QA_CONSOLE_OUT\"\n", ""),
            "release_out",
        ),
        (
            CONSOLE_MANIFEST.replace("port_name = \"console.server\"", "port_name = \"console\""),
            ".server",
        ),
        (
            CONSOLE_MANIFEST.replace("name = \"console-helper\"", "name = \"Console Helper\""),
            "small alphabet",
        ),
    ] {
        let reason = Manifest::parse(&broken)
            .err()
            .unwrap_or_else(|| panic!("{wanted} should have been refused"))
            .to_lowercase();
        assert!(
            reason.contains(&wanted.to_lowercase()),
            "the refusal names {wanted}, said: {reason}"
        );
    }
}
