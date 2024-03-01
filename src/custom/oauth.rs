#![forbid(unsafe_code, missing_docs)]
#![warn(clippy::pedantic)]

use reqwest::Client;
use serde::Deserialize;
use tokio::{io::AsyncReadExt, net::TcpListener, sync::mpsc};

use crate::{
    async_trait_alias::*,
    errors::{OAuthError, TokenError},
    SCOPE,
};

/// Infomation from the temporary http server.
#[derive(Deserialize, Debug)]
pub struct Info {
    /// The code
    pub code: Option<String>,
    /// The state
    pub state: Option<String>,
    // Error
    error: Option<String>,
    // Error description
    error_description: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Token {
    pub token_type: String,
    pub scope: String,
    pub expires_in: u16,
    pub ext_expires_in: u16,
    pub access_token: String,
    pub refresh_token: String,
}

pub fn server(port: u16) -> Result<impl AsyncSendSync<Result<Info, OAuthError>>, OAuthError> {
    let (tx, mut rx) = mpsc::channel::<Info>(1);

    let server = tokio::spawn(async move {
        match TcpListener::bind(format!("127.0.0.1:{}", port)).await {
            Ok(listener) => {
                loop {
                    match listener.accept().await {
                        Ok((mut socket, _)) => {
                            let tx = tx.clone();
                            tokio::spawn(async move {
                                let mut buf = [0; 1024];
                                loop {
                                    let n = match socket.read(&mut buf).await {
                                        Ok(n) if n == 0 => break,
                                        Ok(n) => n,
                                        Err(e) => {
                                            eprintln!("failed to read from socket; err = {:?}", e);
                                            break;
                                        }
                                    };
                                
                                    // Here you would parse the received data into your `Info` struct
                                    // For demonstration, let's assume we have a function `parse_info` that does this
                                    match parse_info(&buf[..n]) {
                                        Ok(info) => {
                                            if let Err(e) = tx.try_send(info) {
                                                eprintln!("failed to send data to channel; err = {:?}", e);
                                            }
                                        },
                                        Err(e) => {
                                            eprintln!("failed to parse info; err = {:?}", e);
                                        }
                                    }
                                }
                            });
                        },
                        Err(e) => {
                            eprintln!("failed to accept connection; err = {:?}", e);
                        }
                    }
                }
            },
            Err(e) => {
                eprintln!("failed to bind listener; err = {:?}", e);
            }
        }
    });
    

    Ok(async move {
        let info = rx.recv().await.expect("server did not receive params");

        if info.error.as_ref().map_or(false, |s| !s.is_empty())
            && info
                .error_description
                .as_ref()
                .map_or(false, |s| !s.is_empty())
        {
            let err = OAuthError::AuthenticationFailure(info.error_description.unwrap());
            Err(err)
        } else {
            server.abort();

            Ok(info)
        }
    })
}

fn parse_info(data: &[u8]) -> Result<Info, OAuthError> {
    // Assuming the data is in a format that can be directly deserialized into Info
    // For demonstration, let's assume the data is a JSON string
    let data_str = std::str::from_utf8(data).map_err(|_| OAuthError::ParseError("Invalid UTF-8".to_string()))?;
    serde_json::from_str::<Info>(data_str).map_err(|_| OAuthError::ParseError("Failed to parse Info".to_string()))
}


pub fn token(
    code: &str,
    client_id: &str,
    port: u16,
    client_secret: &str,
) -> impl AsyncSendSync<Result<Token, TokenError>> {
    let url = format!("https://login.microsoftonline.com/consumers/oauth2/v2.0/token");
    let client = Client::new();
    let body = format!(
      "client_id={}&scope={}&redirect_uri=http://localhost:{}&grant_type=authorization_code&code={}&client_secret={}", 
      client_id, SCOPE, port, code, client_secret);

    async move {
        'out: {
            let result = client.post(url).body(body).send().await;

            let std::result::Result::Ok(response) = result else {
                println!("Part 1");
                break 'out Err(TokenError::ResponseError(
                    "Failed to send request".to_string(),
                ));
            };

            let text = response
                .text()
                .await
                .map_err(|_| TokenError::ResponseError("Failed to send request".to_string()))?;
            let std::result::Result::Ok(token) = serde_json::from_str::<Token>(&text) else {
                break 'out Err(TokenError::ResponseError(
                    "Failed to send request, Check your Client Secret.".to_string(),
                ));
            };
            std::result::Result::Ok(token)
        }
    }
}
