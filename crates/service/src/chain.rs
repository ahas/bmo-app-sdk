//! The delegation chain as an application asks it (`docs/service.md`, 「The delegation chain」): the base library
//! `bmo-chain` handed on, and the one judgement every application makes alike, that what the chain does not
//! allow is refused.

pub use bmo_chain::{Chain, Subject};

/// Whether the chain allows the flag `key` for `subject`: true only where it resolves the flag true. An undeclared
/// kind, a subject the chain does not hold, a service that is not named or does not answer, and a flag it leaves
/// unsaid with no default are each a refusal, the superadministrator's call among them.
pub async fn allows(chain: &Chain, key: &str, subject: Subject) -> bool {
    if !chain.named() {
        return false;
    }

    match chain.resolve(key, subject).await {
        Ok(bmo_chain::Resolution::Resolved(answer)) => bmo_chain::flag(Some(&answer), false),
        Ok(_) | Err(_) => false,
    }
}
