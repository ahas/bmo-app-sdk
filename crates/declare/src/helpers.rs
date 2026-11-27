//! The small writers every rendering shares: quoted lists, steps and script commands.

use std::fmt::Write as _;

/// A TOML inline array body: the values quoted and comma-separated.
pub(crate) fn quoted_list<'a>(values: impl IntoIterator<Item = &'a str>) -> String {
    values.into_iter().map(quote).collect::<Vec<_>>().join(", ")
}

/// One testbed deploy step: its name, its phase and its already-wrapped command.
pub(crate) fn deploy_step(id: &str, phase: &str, command: &str) -> String {
    format!(
        "[[deploy.step]]\nid = {}\nphase = {}\ncommand = {command}\n",
        quote(id),
        quote(phase)
    )
}

/// One testbed build step: its name, its folder, its lane where it builds wide, and its
/// already-wrapped command.
pub(crate) fn build_step(id: &str, dir: &str, lane: Option<&str>, command: &str) -> String {
    let mut out = format!("[[build.step]]\nid = {}\ndir = {}\n", quote(id), quote(dir));
    if let Some(lane) = lane {
        let _ = writeln!(out, "lane = {}", quote(lane));
    }
    let _ = writeln!(out, "command = {command}");
    out
}

/// A step command that is a shell script, kept literal so the text travels as written.
pub(crate) fn script_command(lines: &[&str]) -> String {
    format!("'''\n{}\n'''", lines.join("\n"))
}

/// A Rust string as a TOML basic string.
pub(crate) fn quote(value: &str) -> String {
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
