//! The repository's install declaration a manifest stands for.

use std::fmt::Write as _;

use crate::*;

pub(crate) fn render_install(m: &Manifest) -> String {
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
pub(crate) fn pass_through(name: &str, items: &[toml::Value]) -> String {
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
