//! The removal script a manifest stands for.

use crate::*;

pub(crate) fn render_remove(m: &Manifest) -> String {
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
