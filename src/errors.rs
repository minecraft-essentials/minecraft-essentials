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
    /// Authentcation Failed: {0}
    AuthenticationFailure(String),
    /// Parsing Failed: {0}
    ParseError(String),
    /// Binding error: {0}
    BindError(String),
    /// Socket Read Error: {0}
    SocketReadError(String),
    /// Failed to send data to channel: {0}
    ChannelSendError(String),
    /// Failed to parse info: {0}
    ParseInfoError(String),

}

/// The `LaunchError` enum represents potential errors that can occur during Launching minecraft.
#[derive(Display, Error, Debug)]
pub enum LaunchError {
    /// Launch Requirements Failed: {0}
    Requirements(String),
}
