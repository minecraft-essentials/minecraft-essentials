/*
* Copyright (C) 2024 Mincraft-essnetials

* This program is free software: you can redistribute it and/or modify it
* under the terms of the GNU Affero General Public License as published by
* the Free Software Foundation, either version 3 of the License, or (at your
* option) any later version.

* This program is distributed in the hope that it will be useful, but WITHOUT
* ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
* FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public
* License for more details.

* You should have received a copy of the GNU Affero General Public License
* along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

#![forbid(unsafe_code, missing_docs)]
#![warn(clippy::pedantic)]

use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::error::Error;

/// Defines the Authentification Data that you will recive.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct AuthInfo {
    /// The bearer token that you recive
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

pub async fn token(userhash: &str, xsts_token: &str) -> Result<AuthInfo, Box<dyn Error>> {
    let client = Client::new();
    let identity_token = format!("XBL3.0 x={};{}", userhash, xsts_token);
    let body = json!({
        "identityToken": identity_token
    });

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
