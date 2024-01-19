#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]

use reqwest::Client;
use serde::{Deserialize, Serialize};

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

pub async fn device_authentication_code(client_id: &str) -> Result<CodeResponse, reqwest::Error> {
    let request_url = format!(
        "https://login.microsoftonline.com/common/oauth2/v2.0/devicecode?client_id={}",
        client_id
    );

    let client = Client::new();
    let response = client
        .post(request_url)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .send()
        .await?;

    let response_data: CodeResponse = response.json().await?;

    Ok(response_data)
}

pub async fn authenticate_device(
    device_code: &str,
    client_id: &str,
) -> Result<(u16, String), reqwest::Error> {
    let client = Client::new();
    let request_url = format!(
        "https://login.microsoftonline.com/common/oauth2/v2.0/token?grant_type=urn:ietf:params:oauth:grant-type:device_code&client_id={}&device_code={}",
        client_id,
        device_code
    );

    let request = client
        .post(request_url)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .send()
        .await?;

    let response_data: AuthenticationResponse = request.json().await?;

    let expires_in = response_data.expires_in;
    let token = response_data.access_token;

    Ok((expires_in, token))
}
