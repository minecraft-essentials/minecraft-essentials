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



use reqwest::Client;
use std::error::Error as Error;
use serde::Deserialize;
use serde_json::json;


use crate::AuthData;

#[derive(Debug, Deserialize)]
struct MojangResponse {
   username: String,
   access_token: String,
   token_type: String,
   expires_in: i32,
}



pub async fn mojang(userhash: &str, xsts_token: &str) -> Result<AuthData, Box<dyn Error>> {
    let client = Client::new();
    let identity_token = format!("XBL3.0 x={};{}", userhash, xsts_token);
    let body = json!({
        "identityToken": identity_token
    });
    
    let res = client.post("https://api.minecraftservices.com/authentication/login_with_xbox")
        .json(&body)
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
 
    Ok(AuthData { uuid, access_token, expires_in })
 }