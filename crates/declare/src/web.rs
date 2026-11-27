//! The web scaffold's defaults a manifest stands for.

use crate::*;

pub(crate) fn render_app_defaults(m: &Manifest, web: &Web) -> String {
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
