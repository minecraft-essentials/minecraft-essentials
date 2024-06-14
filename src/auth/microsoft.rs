#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]

use crate::{async_trait_alias::*, errors::AuthErrors};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::{io::AsyncReadExt, net::TcpListener, sync::mpsc};

pub(crate) const SCOPE: &str = "XboxLive.signin%20XboxLive.offline_access";

/// Temporary http server Infomation.
#[derive(Deserialize, Debug)]
pub struct OuathInfo {
    /// The code
    pub code: Option<String>,
    /// State of Oauth (Default: 12345)
    pub state: Option<String>,
    error: Option<String>,
    error_description: Option<String>,
}

pub fn ouath(port: u16) -> Result<impl AsyncSendSync<Result<OuathInfo, AuthErrors>>, AuthErrors> {
    let (tx, mut rx) = mpsc::channel::<OuathInfo>(1);

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
                                            let err = AuthErrors::SocketReadError(format!(
                                                "failed to read from socket; err = {:?}",
                                                e
                                            ));
                                            eprintln!("{}", err);
                                            break;
                                        }
                                    };

                                    // Here you would parse the received data into your `Info` struct
                                    // For demonstration, let's assume we have a function `parse_info` that does this
                                    match parse_oauth(&buf[..n]) {
                                        Ok(info) => {
                                            if let Err(e) = tx.try_send(info) {
                                                let err = AuthErrors::ChannelSendError(format!(
                                                    "failed to send data to channel; err = {:?}",
                                                    e
                                                ));
                                                eprintln!("{}", err);
                                            }
                                        }
                                        Err(e) => {
                                            let err = AuthErrors::ParseInfoError(format!(
                                                "failed to parse info; err = {:?}",
                                                e
                                            ));
                                            eprintln!("{}", err);
                                        }
                                    }
                                }
                            });
                        }
                        Err(e) => {
                            let err = AuthErrors::AcceptConnectionError(format!(
                                "failed to accept connection; err = {:?}",
                                e
                            ));
                            eprintln!("{}", err);
                        }
                    }
                }
            }
            Err(e) => {
                let err = AuthErrors::BindError(format!("failed to bind listener; err = {:?}", e));
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
            let err = AuthErrors::AuthenticationFailure(info.error_description.unwrap());
            Err(err)
        } else {
            server.abort();

            Ok(info)
        }
    })
}

fn parse_oauth(data: &[u8]) -> Result<OuathInfo, AuthErrors> {
    let data_str = std::str::from_utf8(data)
        .map_err(|_| AuthErrors::ParseError("Invalid UTF-8".to_string()))?;

    let mut query_start = None;
    while query_start.is_none() {
        query_start = data_str.find('?');
    }
    let query_start =
        query_start.ok_or_else(|| AuthErrors::ParseError("No query string found".to_string()))?;
    let query_end = data_str.find('#').unwrap_or_else(|| data_str.len());
    let query_string = &data_str[query_start + 1..query_end];

    let query_params: Vec<(String, String)> = url::form_urlencoded::parse(query_string.as_bytes())
        .into_owned()
        .collect();
    let code = query_params
        .iter()
        .find_map(|(k, v)| if k == "code" { Some(v.clone()) } else { None });
    let state = query_params.iter().find_map(|(k, v)| {
        if k == "state" {
            let http_start = v.find(" HTTP/1.1\r\n");
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

    let info = OuathInfo {
        code,
        state,
        error,
        error_description,
    };
    Ok(info)
}

/// OAuth Auth Token Infomation.
#[derive(Deserialize, Debug)]
pub struct OuathToken {
    pub token_type: String,
    pub scope: String,
    pub expires_in: u16,
    pub ext_expires_in: u16,
    pub access_token: String,
    pub refresh_token: String,
}

pub fn ouath_token(
    code: &str,
    client_id: &str,
    port: u16,
    client_secret: &str,
) -> impl AsyncSendSync<Result<OuathToken, AuthErrors>> {
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
                break 'out Err(AuthErrors::ResponseError(
                    "Failed to send request".to_string(),
                ));
            };

            let text = response
                .text()
                .await
                .map_err(|_| AuthErrors::ResponseError("Failed to send request".to_string()))?;
            let std::result::Result::Ok(token) = serde_json::from_str::<OuathToken>(&text) else {
                break 'out Err(AuthErrors::ResponseError(
                    "Failed to send request, Check your Client Secret.".to_string(),
                ));
            };
            std::result::Result::Ok(token)
        }
    }
}

// Device Code

#[derive(Debug, Serialize, Deserialize)]
pub struct CodeResponse {
    pub device_code: String,
    pub user_code: String,
    pub verification_uri: String,
    pub expires_in: u32,
    pub interval: u16,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticationResponse {
    pub expires_in: u16,
    access_token: String,
}

/// Defines expiry and token
#[derive(Debug)]
pub struct CodeInfo {
    /// Provides expiry
    pub expires_in: u16,
    /// Provides token
    pub token: String,
}

pub fn device_authentication_code(
    client_id: &str,
) -> impl AsyncSendSync<Result<CodeResponse, reqwest::Error>> {
    let request_url =
        format!("https://login.microsoftonline.com/consumers/oauth2/v2.0/devicecode",);
    let body = format!("client_id={}&scope={}", client_id, SCOPE);
    let client = reqwest::Client::new();

    device_internal(client, request_url, body)
}

async fn device_internal(
    client: reqwest::Client,
    request_url: String,
    body: String,
) -> Result<CodeResponse, reqwest::Error> {
    let response = client.post(request_url).body(body).send().await?;

    let response_data: CodeResponse = response.json().await?;

    Ok(response_data)
}

pub fn authenticate_device(
    device_code: &str,
    client_id: &str,
) -> impl AsyncSendSync<Result<CodeInfo, reqwest::Error>> {
    let client = Client::new();
    let request_url = format!("https://login.microsoftonline.com/common/consumers/v2.0/token",);

    let body = format!(
        "grant_type=urn:ietf:params:oauth:grant-type:device_code&client_id={}&device_code={}",
        client_id, device_code
    );

    authenticate_device_internal(request_url, body, client)
}

async fn authenticate_device_internal(
    request_url: String,
    body: String,
    client: reqwest::Client,
) -> Result<CodeInfo, reqwest::Error> {
    let request = client
        .post(request_url)
        .body(body)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .send()
        .await?;

    let response_data: AuthenticationResponse = request.json().await?;

    let expires_in = response_data.expires_in;
    let token = response_data.access_token;

    Ok(CodeInfo { expires_in, token })
}
