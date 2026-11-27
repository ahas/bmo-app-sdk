//! The systemd units a manifest stands for: the service and, for a web half, its pages.

use std::fmt::Write as _;

use crate::*;

pub(crate) fn render_service_unit(m: &Manifest) -> String {
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

pub(crate) fn render_pages_unit(m: &Manifest, web: &Web) -> String {
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
