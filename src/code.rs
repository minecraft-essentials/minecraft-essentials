#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]

use crate::{async_trait_alias::*, SCOPE};
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
    let client = Client::new();

    device_internal(client, request_url, body)
}

pub async fn device_internal(
    client: Client,
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

    authenticate_internal(request_url, body, client)
}

async fn authenticate_internal(
    request_url: String,
    body: String,
    client: Client,
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
