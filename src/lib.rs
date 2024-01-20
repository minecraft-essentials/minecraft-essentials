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


#![doc = include_str!("../README.md")]
#![forbid(unsafe_code, missing_docs)]
#![warn(clippy::pedantic)]

mod code;
mod server;
mod token;
mod xbox;

// External Imports
use rand::Rng;

// Local Imports
pub use code::Info as CodeInfo;
pub use server::Info as ServerInfo;

/// Xbox Live Authentification Scope.
pub const SCOPE: &str = "XboxLive.signin";

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

        let url = format!("https://login.microsoftonline.com/consumers/oauth2/v2.0/authorize?client_id={}&response_type=code&redirect_uri=http://localhost:{}&response_mode=query&scope={}&state=12345", clientid, port, SCOPE);

        // Returns the port and url as self.
        Self { url, port }
    }

    /// Returns the URL of the OAuth object.
    pub fn url(&self) -> &str {
        &self.url
    }

    /// The launch function
    pub async fn launch(&self, bedrockrelm: bool) -> std::io::Result<ServerInfo> {
        // Launches the temporary http server.
        server::launch(self.port).await
        // Runs the xbox function with the code and state.
    }
}

/// Minecraft Device Code Authentification Method.
pub struct DeviceCode {
    /// Returns the url
    pub url: String,
    /// Returns the instuctions
    pub message: String,
    /// Provides expires
    pub expires_in: u32,
    /// The code you give to the user
    pub user_code: String,
    /// Device code for the Code:authenticate_device Proccess
    pub device_code: String,
}

/// Implemation of the device code.
impl DeviceCode {
    /// Proccess to get the code.
    pub async fn new(client_id: &str) -> Result<Self, reqwest::Error> {
        let response_data = code::device_authentication_code(client_id).await?;

        // Defines all of the outputs.
        let url = response_data.verification_uri;
        let message = response_data.message;
        let expires_in = response_data.expires_in;
        let user_code = response_data.user_code;
        let device_code = response_data.device_code;

        // Returns the outputs as self.
        Ok(Self {
            url,
            message,
            expires_in,
            user_code,
            device_code,
        })
    }

    /// The prelaunch stuff.
    pub fn prelaunch(&self) -> (&str, &str, u32, &str) {
        (&self.url, &self.message, self.expires_in, &self.user_code)
    }

    /// The launch function
    pub async fn launch(&self, client_id: &str, bedrockrelm: bool) -> Result<CodeInfo, reqwest::Error> {
        code::authenticate_device(&self.device_code, client_id).await
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
