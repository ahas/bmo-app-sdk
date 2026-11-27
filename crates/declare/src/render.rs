//! The readings a manifest stands for, rendered whole: one file a reading, each opening on the generated head.

use crate::*;

/// One generated file: its path in the repository, its content, and whether it is run as a
/// script.
#[derive(Debug, PartialEq, Eq)]
pub struct Generated {
    pub path: String,
    pub content: String,
    pub executable: bool,
}

pub(crate) const GENERATED_HEAD: &str = concat!(
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
