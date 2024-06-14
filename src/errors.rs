#![forbid(unsafe_code, missing_docs)]
#![warn(clippy::pedantic)]

use displaydoc::Display;
use thiserror::Error;

/// The `AuthErrors` represents potential errors that can occur during Authentication.
#[derive(Display, Error, Debug)]
pub enum AuthErrors {
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
    /// Failed to accept connection: {0}
    AcceptConnectionError(String),
    /// Response Failed: {0}
    ResponseError(String),
}

/// The `LaunchErrors` enum represents potential errors that can occur during Launch-related operations.
#[derive(Display, Error, Debug)]
pub enum LaunchErrors {
    /// Unsupported Device: {0}
    UnsupportedDevice(String),
    /// Failed to fetch username {0}
    UsernameFetchError(String),
    /// Unsuported Archtechure: {0} switch to another cpu archtechure and try again.
    UnsupportedArchitecture(String),
    /// Response Failed: {0}
    ResponseError(String),
    /// Requirements not reached: {0}
    Requirements(String),
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
