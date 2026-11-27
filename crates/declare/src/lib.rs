//! The application manifest `bmo-app.toml` and what it stands for: one declaration per
//! application whose generated readings are that repository's `testbed.toml`,
//! `install.toml`, systemd units, removal script and web scaffold, as the repository's
//! `docs/manifest.md` states.
//!
//! The manifest holds only what differs per application; everything a generated file says
//! beyond the manifest is the mechanism every application's says alike. An application
//! belongs to one family (docs/manifest.md): a web half, a console half, or the service
//! alone, and the family decides what the readings carry.

mod helpers;
mod install;
mod manifest;
mod readings;
mod remove;
mod render;
mod testbed;
mod units;
mod web;

pub(crate) use helpers::*;
pub(crate) use install::*;
pub use manifest::*;
pub use readings::*;
pub(crate) use remove::*;
pub use render::*;
pub(crate) use testbed::*;
pub(crate) use units::*;
pub(crate) use web::*;
