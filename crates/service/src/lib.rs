//! The service skeleton every BMO application binary stands on, as stated in the
//! repository's `docs/service.md`: the JSON envelope every answer of the API route tree
//! travels in, the serving loop that stops gracefully on the signals the service manager
//! and a terminal send, and the health verb's check of a running service and its root page.
//!
//! What an application's own domain answers with is the application's; this crate answers
//! with the shape every application's answers share.

mod answer;
mod chain;
mod health;
mod serve;

pub use answer::{
    CodeProblem, Notice, Problem, Refusal, answer, code_problem, not_found, problem, refusal,
};
pub use chain::{Chain, Subject, allows};
pub use health::{check as health_check, check_site as site_check};
pub use serve::{serve, serve_until, shutdown_signal};

/// The error every skeleton part hands back: any boxed error a thread can carry.
pub type BoxError = Box<dyn std::error::Error + Send + Sync>;
