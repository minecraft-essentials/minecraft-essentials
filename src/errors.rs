use std::fmt::Display;
use std::error::Error;
use core::fmt;


/// OAuth Token Error
#[derive(Debug)]
pub struct TokenError {}

impl Error for TokenError {}

impl Display for TokenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Token Error")
    }
}


/// Xbox Error
#[derive(Debug)]
pub struct XboxError {}

impl Error for XboxError {}

impl Display for XboxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Xbox Error")
    }
}


/// XTSError
#[derive(Debug)]
pub struct XTSError {}

impl Error for XTSError {}

impl Display for XTSError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "XTS Error")
    }
}


