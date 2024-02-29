#![forbid(unsafe_code, missing_docs)]
#![warn(clippy::pedantic)]

use displaydoc::Display;
use thiserror::Error;
/// The `TokenError` enum represents potential errors that can occur during token operations.
#[derive(Display, Error, Debug)]
pub enum TokenError {
    /// Response Failed: {0}
    ResponseError(String),
}

/// The `XboxError` enum represents potential errors that can occur during Xbox-related operations.
#[derive(Display, Error, Debug)]
pub enum XboxError {
    /// Response Failed: {0}
    ResponseError(String),
}

/// The `XTSError` enum represents potential errors that can occur during XTS-related operations.
#[derive(Display, Error, Debug)]
pub enum XTSError {
    /// Response Failed: {0}
    ResponseError(String),
}

/// The `OAuthError` enum represents potential errors that can occur during OAuth authentication.
#[derive(Display, Error, Debug)]
pub enum OAuthError {
    /// Failed to authenticate: Response: {0}
    AuthenticationFailure(String),
    /// Binding error: {0}
    BindError(String),
}
