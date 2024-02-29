#![forbid(unsafe_code, missing_docs)]
#![warn(clippy::pedantic)]

use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::error::Error;

use crate::async_trait_alias::AsyncSendSync;

/// Defines the Authentification Data that you will recive from mojang.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct AuthInfo {
    /// The bearer token that you recive this is used in Launching, Apis.
    pub access_token: String,
    /// NOT THE PLAYERS UUID! This UUID Is Useful for launching.
    pub uuid: String,
    /// The expiry date of the token.
    pub expires_in: i32,
    /// The xts token that you will be return if bedrockrelm is true.
    pub xts_token: Option<String>,
}

#[derive(Debug, Deserialize)]
struct MojangResponse {
    username: String,
    access_token: String,
    token_type: String,
    expires_in: i32,
}

pub fn token(
    userhash: &str,
    xsts_token: &str,
) -> impl AsyncSendSync<Result<AuthInfo, Box<dyn Error>>> {
    let client = Client::new();
    let identity_token = format!("XBL3.0 x={};{}", userhash, xsts_token);
    let body = json!({
        "identityToken": identity_token
    });
    tokeninternal(client, body)
}

async fn tokeninternal(client: Client, body: Value) -> Result<AuthInfo, Box<dyn Error>> {
    let res = client
        .post("https://api.minecraftservices.com/authentication/login_with_xbox")
        .body(body.to_string())
        .send()
        .await?;

    let response: MojangResponse = res.json().await?;

    if response.token_type != "Bearer" {
        println!("Sorry, we ran into an error in authentication.");
        return Err("Invalid token type".into());
    }

    let access_token = response.access_token;
    let uuid = response.username;
    let expires_in = response.expires_in;

    Ok(AuthInfo {
        uuid: uuid,
        access_token: access_token,
        expires_in: expires_in,
        xts_token: None,
    })
}
