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
tool = "widget"

[web]
pages_port = 7551
dev_port = 3550

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

fn generated(path: &str) -> String {
    let manifest = Manifest::parse(WEB_MANIFEST).expect("the manifest reads");
    render(&manifest)
        .into_iter()
        .find(|file| file.path == path)
        .unwrap_or_else(|| panic!("no reading stands at {path}"))
        .content
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
