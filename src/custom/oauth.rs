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
                                            let err = OAuthError::SocketReadError(format!("failed to read from socket; err = {:?}", e));
                                            eprintln!("{}", err);
                                            break;
                                        }
                                    };

                                    // Here you would parse the received data into your `Info` struct
                                    // For demonstration, let's assume we have a function `parse_info` that does this
                                    match parse_info(&buf[..n]) {
                                        Ok(info) => {
                                            if let Err(e) = tx.try_send(info) {
                                                let err = OAuthError::ChannelSendError(format!("failed to send data to channel; err = {:?}", e));
                                                eprintln!("{}", err);
                                            }
                                        }
                                        Err(e) => {
                                            let err = OAuthError::ParseInfoError(format!("failed to parse info; err = {:?}", e));
                                            eprintln!("{}", err);
                                        }
                                    }
                                }
                            });
                        }
                        Err(e) => {
                            let err = OAuthError::AcceptConnectionError(format!("failed to accept connection; err = {:?}", e));
                            eprintln!("{}", err);
                        }
                    }
                }
            }
            Err(e) => {
                let err = OAuthError::BindError(format!("failed to bind listener; err = {:?}", e));
                eprintln!("{}", err);
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


async fn run_server(port: u16, tx: mpsc::Sender<Info>) -> Result<(), OAuthError> {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port))
    .await
    .map_err(|e| OAuthError::BindError(format!("failed to bind listener; err = {:?}", e)))?;

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
                                let err = OAuthError::SocketReadError(format!("failed to read from socket; err = {:?}", e));
                                eprintln!("{}", err);
                                break;
                            }
                        };

                        // Here you would parse the received data into your `Info` struct
                        // For demonstration, let's assume we have a function `parse_info` that does this
                        match parse_info(&buf[..n]) {
                            Ok(info) => {
                                if let Err(e) = tx.try_send(info) {
                                    let err = OAuthError::ChannelSendError(format!("failed to send data to channel; err = {:?}", e));
                                    eprintln!("{}", err);
                                }
                            }
                            Err(e) => {
                                let err = OAuthError::ParseInfoError(format!("failed to parse info; err = {:?}", e));
                                eprintln!("{}", err);
                            }
                        }
                    }
                });
            }
            Err(e) => {
                let err = OAuthError::AcceptConnectionError(format!("failed to accept connection; err = {:?}", e));
                eprintln!("{}", err);
            }
        }
    }
}



#[cfg(feature = "tauri")]
pub fn tauri_server(port: u16) -> Result<impl AsyncSendSync<Result<Info, OAuthError>>, OAuthError> {
    let (tx, mut rx) = mpsc::channel::<Info>(1);

    let server = tokio::task::spawn_blocking(move || {
        run_server(port, tx.clone())
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
    let data_str = std::str::from_utf8(data)
        .map_err(|_| OAuthError::ParseError("Invalid UTF-8".to_string()))?;

    // Extract the query string from the HTTP request
    let mut query_start = None;
    while query_start.is_none() {
        query_start = data_str.find('?');
    }
    let query_start =
        query_start.ok_or_else(|| OAuthError::ParseError("No query string found".to_string()))?;
    let query_end = data_str.find('#').unwrap_or_else(|| data_str.len());
    let query_string = &data_str[query_start + 1..query_end];

    // Parse the query string directly
    let query_params: Vec<(String, String)> = url::form_urlencoded::parse(query_string.as_bytes())
        .into_owned()
        .collect();

    // Extract the 'code', 'state', 'error', and 'error_description' parameters
    let code = query_params
        .iter()
        .find_map(|(k, v)| if k == "code" { Some(v.clone()) } else { None });
    let state = query_params.iter().find_map(|(k, v)| {
        if k == "state" {
            // Find the position of "HTTP/1.1\r\n" in the state value
            let http_start = v.find(" HTTP/1.1\r\n");
            // If found, slice the string up to that position
            http_start.map(|pos| v[..pos].to_string())
        } else {
            None
        }
    });
    let error = query_params
        .iter()
        .find_map(|(k, v)| if k == "error" { Some(v.clone()) } else { None });
    let error_description = query_params.iter().find_map(|(k, v)| {
        if k == "error_description" {
            Some(v.clone())
        } else {
            None
        }
    });

    // Construct the Info struct
    let info = Info {
        code,
        state,
        error,
        error_description,
    };
    Ok(info)
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
