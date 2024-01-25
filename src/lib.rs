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
mod mojang;
mod oauth;
mod pkce;
mod xbox;

use std::io::SeekFrom;

// Imports
use base64::Engine;
pub use code::CodeInfo;
pub use mojang::AuthInfo as AuthData;
use rand::{thread_rng, Rng};
use sha2::{Digest, Sha256};

/// Scopes Required for Xbox Live And Minecraft Authentcation.
pub const SCOPE: &str = "XboxLive.signin offline_access";

/// Minecraft OAuth Authentification Method.
pub struct Oauth {
    url: String,
    port: u16,
    code_verify: Vec<u8>,
    client_id: String,
}

/// Implemation of the oauth.
impl Oauth {
    /// Creates a new instance of Oauth.
    pub fn new(clientid: &str, port: Option<u16>) -> Self {
        // Uses port 8000 by deafult but the optional u16 is avalible if needed.
        let port = port.unwrap_or(8000);

        // Request Mode for params.
        const REQUEST_MODE: &str = "query";
        // Request Type for params.
        const REQUEST_TYPE: &str = "code";

        let code_verify = pkce::verifier(128);
        let code_challange = pkce::code_challenge(&code_verify);

        // Creates the url with the params that microsoft needs.
        let params = format!("client_id={}&response_type={}&redirect_uri=http://localhost:{}&response_mode={}&scope={}&state=12345&code_challenge={}&code_challenge_method=S256", clientid, REQUEST_TYPE, port, REQUEST_MODE, SCOPE, code_challange);
        // Create the url for microsoft authentcation.
        let url = format!(
            "https://login.microsoftonline.com/consumers/oauth2/v2.0/authorize/?{}",
            params
        );

        // Returns the port and url as self.
        Self {
            url,
            port,
            code_verify,
            client_id: clientid.to_string(),
        }
    }

    /// Returns the url from the new instance.
    pub fn url(&self) -> &str {
        &self.url
    }

    /// Launches the Minecraft Oauth Authentifcation proccess.
    /// Note: Please Keep your client Secret Safe.
    pub async fn launch(
        &self,
        bedrockrelm: bool,
        client_secret: &str,
    ) -> Result<AuthData, Box<dyn std::error::Error>> {
        // Launches the temporary http server.
        let http_server = oauth::server(self.port).await?;
        let code_verify_str = String::from_utf8(self.code_verify.clone()).unwrap();
        let token = oauth::token( http_server.code.expect("\x1b[31mXbox Expected code.\x1b[0m").as_str(),&self.client_id,self.port,&code_verify_str,&client_secret,
        ).await?;

        // Launches the Xbox UserHash And Xbl Token Process.
        let xbox = xbox::xbl(&token.access_token).await?;
        // Launches the Xsts Token Process.
        let xts = xbox::xsts_token(
            // Gets the token from the xbox struct.
            xbox.token.as_str(),
            // Gets the userhash from the xbox struct.
            &xbox.display_claims.xui[0].uhs,
            // Gets the bedrockRelm from input.
            bedrockrelm,
        )
        .await?;

        // Checks if bedrockrelm is true if true then returns just the xts token.
        if bedrockrelm == true {
            return Ok(AuthData {
                // Sets the access token to null.
                access_token: "null".to_string(),
                // Sets the uuid to null.
                uuid: "null".to_string(),
                // Sets the expires in to null.
                expires_in: 0,
                // Sets the xts token to the xts token.
                xts_token: Some(xts.token),
            });
        } else {
            // Launches the Mojang Bearer Token Process.
            let mojangtoken = mojang::token(&xbox.display_claims.xui[0].uhs, &xts.token).await?;
            // Returns the minecraft bearer info reponse
            return Ok(mojangtoken);
        }
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

        // Define Outputs.
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
    pub async fn launch(
        &self,
        client_id: &str,
        bedrockrelm: bool,
    ) -> Result<CodeInfo, reqwest::Error> {
        code::authenticate_device(&self.device_code, client_id).await
    }
}
