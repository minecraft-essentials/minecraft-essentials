/* Minecraft-Essentials
 * Copyright (C) 2024 minecraft-essentials
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License v3.0
 * along with this program.
 */

#![forbid(unsafe_code, missing_docs)]
#![warn(clippy::pedantic)]

use reqwest::{header, Client, Error};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct Xui {
    pub uhs: String,
}

#[derive(Deserialize)]
pub struct DisplayClaims {
    pub xui: Vec<Xui>,
}

#[derive(Deserialize)]
pub struct XblOutput {
    pub token: String,
    pub display_claims: DisplayClaims,
}

pub async fn xbl(code: &str) -> Result<XblOutput, Error> {
    let url = format!("https://user.auth.xboxlive.com/user/authenticate");
    let client = Client::new();
    let rps_ticket = format!("d={}", code);
    let mut headers = header::HeaderMap::new();
    headers.insert(
        "Content-Type",
        header::HeaderValue::from_static("application/json"),
    );
    headers.insert(
        "Accept",
        header::HeaderValue::from_static("application/json"),
    );
    let body = json!({
       "Properties": {
           "AuthMethod": "RPS",
           "SiteName": "user.auth.xboxlive.com",
           "RpsTicket": rps_ticket,
       },
       "RelyingParty": "http://auth.xboxlive.com",
       "TokenType": "JWT"
    });
    let response = client
        .post(url)
        .body(body.to_string())
        .headers(headers)
        .send()
        .await?;

    let launch_output: XblOutput = response.json().await?;
    Ok(launch_output)
}

#[derive(Deserialize)]
pub struct XtsOutput {
    pub token: String,
    pub display_claims: DisplayClaims,
}

pub async fn xsts_token(
    xblToken: &str,
    userhash: &str,
    bedrockRel: bool,
) -> Result<XtsOutput, Error> {
    let url = format!("https://user.auth.xboxlive.com/user/authenticate");
    let bedrock_party = "https://pocket.realms.minecraft.net/";
    let java_party = "rp://api.minecraftservices.com/";
    let party = if bedrockRel == true {
        bedrock_party
    } else {
        java_party
    };

    let client = Client::new();
    let mut headers = header::HeaderMap::new();
    headers.insert(
        "Content-Type",
        header::HeaderValue::from_static("application/json"),
    );
    headers.insert(
        "Accept",
        header::HeaderValue::from_static("application/json"),
    );
    let body = json!({
       "Properties": {
           "SandboxId": "RETAIL",
           "UserTokens": [
             xblToken
           ]
       },
       "RelyingParty": party,
       "TokenType": "JWT"
    });
    let response = client
        .post(url)
        .body(body.to_string())
        .headers(headers)
        .send()
        .await?;

    let launch_output: XtsOutput = response.json().await?;

    if !launch_output.display_claims.xui[0].uhs.contains(userhash) {
        panic!("An error may have hapened at xts token.");
    } else {
        Ok(launch_output)
    }
}
