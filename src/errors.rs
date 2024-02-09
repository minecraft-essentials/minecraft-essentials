use thiserror::Error;



#[derive(Error, Debug)]
pub enum TokenError {
    #[error("Response Failed: {0}")]
    ResponseError(String),
}



#[derive(Error, Debug)]
pub enum XboxError {
    #[error("Response Failed: {0}")]
    ResponseError(String)
}


#[derive(Error, Debug)]
pub enum XTSError {
    #[error("Response Failed: {0}")]
    ResponseError(String)
}




#[derive(Error, Debug)]
pub enum OAuthError {
    #[error("Failed to authenticate: Response: {0}")]
    AuthenticationFailure(String),
    #[error("Binding error: {0}")]
    BindError(String),
}

