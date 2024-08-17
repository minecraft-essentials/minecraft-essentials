use displaydoc::Display;

mod projects;
mod version_files;
mod versions;

const MODRINTH_API: &str = "https://api.modrinth.com/v2";

/// Errors that can occur while interacting with the Modrinth API.
#[derive(Display, Debug, thiserror::Error)]
pub enum ModrinthErrors {
    /// The request was not authenticated with a Access Token.
    AuthenticationRequired,
    /// You are being rate limited.
    RateLimited,
}
