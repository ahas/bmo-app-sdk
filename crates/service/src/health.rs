//! The health verb's checks, which the deployment asks until they answer ready.

use std::time::Duration;

use crate::BoxError;

/// How long one request of a check may take before the check says the service is not answering.
const REQUEST_TIMEOUT: Duration = Duration::from_secs(5);

/// Answers `Ok` once the service at the health address answers it with 2xx.
pub async fn check(health_url: &str) -> Result<(), BoxError> {
    asked(health_url).await
}

/// Answers `Ok` once the service answers its health address and the root page of that
/// address's site with 2xx. The root page travels the whole front door: where an
/// application's pages render behind a page server of its own, a service that answers health
/// but cannot hand a page on fails this check, which is what the deployment is waiting to learn.
pub async fn check_site(health_url: &str) -> Result<(), BoxError> {
    asked(health_url).await?;
    let root = reqwest::Url::parse(health_url)?.join("/")?;
    asked(root.as_str()).await
}

async fn asked(url: &str) -> Result<(), BoxError> {
    let client = reqwest::Client::builder()
        .timeout(REQUEST_TIMEOUT)
        .build()?;
    client.get(url).send().await?.error_for_status()?;
    Ok(())
}
