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

#![doc = include_str!("../README.md")]
#![forbid(unsafe_code, missing_docs)]
#![warn(clippy::pedantic)]

mod server;
mod token;
mod xbox;
mod xts;
mod code;

// External Imports
use rand::Rng;

// Local Imports
pub use server::Info as ServerInfo;

/// Minecraft OAuth Authentification Method. 
pub struct Oauth {
    url: String,
    port: u16,
}



/// Implemation of the oauth.
impl Oauth {
    /// Create the oauth url.
    pub fn new(clientid: &str) -> Self {
        // Randomized port part.
        let mut rng = rand::thread_rng();
        let port = rng.gen_range(25535..=65535);

        let url = format!("https://login.microsoftonline.com/common/oauth2/v2.0/authorize?client_id={}&response_type=code&redirect_uri=http://localhost:{}&response_mode=query&scope=openid%20offline_access%20https%3A%2F%2Fgraph.microsoft.com%2Fmail.read&state=12345", clientid, port);

        // Returns the port and url as self.
        Self { url, port }
    }

    /// Returns the URL of the OAuth object.
    pub fn url(&self) -> &str {
        &self.url
    }

    /// The launch function
    pub async fn launch(&self) -> std::io::Result<ServerInfo> {
        // Launches the temporary http server.
        server::launch(self.port).await
    }
}

/// Minecraft Device Code Authentification Method. 
pub struct DeviceCode {
    url: String,
    message: String,
    expires_in: u32,
    user_code: String,
    device_code: String,
}


/// Implemation of the device code.
impl DeviceCode {
    /// Proccess to get the code.
    pub async fn new(client_id: &str) -> Result<Self, reqwest::Error> {
        pub const CONTENT_TYPE: &str = "application/x-www-form-urlencoded";
        let response_data = code::code(client_id, CONTENT_TYPE).await?;
     
        Ok(Self {
            url: response_data.verification_uri,
            message: response_data.message,
            expires_in: response_data.expires_in,
            user_code: response_data.user_code,
            device_code: response_data.device_code,
        })
     }

    /// The prelaunch stuff. 
    pub fn prelaunch(&self) -> (&str, &str, u32, &str) {
        (&self.url, &self.message, self.expires_in, &self.user_code)
    }

    /// The launch function
    pub async fn launch(&self, client_id: &str) {
        let _ = code::auth(&self.device_code, client_id).await;
    }
}



/// Defines the Authentification Data that you will recive.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthData {
    /// The bearer token that you recive
    pub access_token: String,
    /// NOT THE PLAYERS UUID! This UUID Is Useful for launching.
    pub uuid: String,
    /// The expiry date of the token.
    pub expires_in: i32,
}