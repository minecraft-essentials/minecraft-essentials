#![doc = include_str!("../README.md")]
#![forbid(unsafe_code, missing_docs)]
#![warn(clippy::pedantic)]

pub(crate) mod async_trait_alias;
mod code;
mod errors;
mod mojang;
mod oauth;
mod xbox;

// Imports
pub use mojang::AuthInfo as AuthData;

/// Scopes Required for Xbox Live And Minecraft Authentcation.
pub(crate) const SCOPE: &str = "XboxLive.signin%20XboxLive.offline_access";
pub(crate) const EXPERIEMNTAL_MESSAGE: &str = "\x1b[33mNOTICE: You are using and Experiemntal Feature.\x1b[0m";

/// Minecraft OAuth Authentification Method.
pub struct Oauth {
    url: String,
    port: u16,
    client_id: String,
}

/// Minecraft Device Code Authentification Method.
pub struct DeviceCode {
    url: String,
    message: String,
    expires_in: u32,
    user_code: String,
    device_code: String,
    client_id: String,
}


/// The Method to refresh your mincraft bearer token.
#[cfg(feature = "renew")]
pub struct RefreshBearer {
    refresh_token: String,
    client_id: String,
    port: u16,
    client_secret: String
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

        // Creates the url with the params that microsoft needs.
        let params = format!("client_id={}&response_type={}&redirect_uri=http://localhost:{}&response_mode={}&scope={}&state=12345", clientid, REQUEST_TYPE, port, REQUEST_MODE, SCOPE);
        // Create the url for microsoft authentcation.
        let url = format!(
            "https://login.microsoftonline.com/consumers/oauth2/v2.0/authorize/?{}",
            params
        );

        // Returns the port and url as self.
        Self {
            url,
            port,
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
        let http_server = oauth::server(self.port)?.await?;
        let token = oauth::token(
            http_server
                .code
                .expect("\x1b[31mXbox Expected code.\x1b[0m")
                .as_str(),
            &self.client_id,
            self.port,
            &client_secret,
        )
        .await?;

        // Launches the Xbox UserHash And Xbl Token Process.
        let xbox = xbox::xbl(&token.access_token).await?;
        // Launches the Xsts Token Process.
        let xts = xbox::xsts_token(
            // Gets the token from the xbox struct.
            &xbox.token,
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
            // Returns just the access Token and UUID For Luanching
            return Ok(mojang::token(&xbox.display_claims.xui[0].uhs, &xts.token).await?);
        }
    }
}

/// Implemation of the device code.
impl DeviceCode {
    /// Proccess to get the code.
    pub fn new(
        client_id: &str,
    ) -> impl async_trait_alias::AsyncSendSync<Result<Self, reqwest::Error>> {
        println!(
            "{}",
            EXPERIEMNTAL_MESSAGE
        );
        // Function to start a new device code.
        let client_id_str = client_id.to_string();
        async move {
            let response_data = code::device_authentication_code(&client_id_str).await?;

            // Returns the outputs as self.
            Ok(Self {
                url: response_data.verification_uri,
                message: response_data.message,
                expires_in: response_data.expires_in,
                user_code: response_data.user_code,
                device_code: response_data.device_code,
                client_id: client_id_str,
            })
        }
    }

    /// To Recive details for the device code.
    pub fn prelaunch(&self) -> (&str, &str, u32, &str) {
        (&self.url, &self.message, self.expires_in, &self.user_code)
    }

    /// Launches the device code authentifcation.
    pub async fn launch(&self, bedrockrelm: bool) -> Result<AuthData, Box<dyn std::error::Error>> {
        let token = code::authenticate_device(&self.device_code, &self.client_id).await?;
        let xbox = xbox::xbl(&token.token).await?;
        let xts = xbox::xsts_token(&xbox.token, bedrockrelm).await?;
        if bedrockrelm == true {
            return Ok(AuthData {
                access_token: "null".to_string(),
                uuid: "null".to_string(),
                expires_in: 0,
                // Sets the xts token to the xts token.
                xts_token: Some(xts.token),
            });
        } else {
            return Ok(mojang::token(&xbox.display_claims.xui[0].uhs, &xts.token).await?);
        }
    }
}

#[cfg(feature = "renew")]
impl RefreshBearer {
    /// Creates a new instance of refreshing the bearer token.
    pub fn new(refresh_token: &str, client_id: &str, port: Option<u16>, client_secret: &str) -> Self {
        let port = port.unwrap_or(8000);
        Self {
           refresh_token: refresh_token.to_string(),
           client_id: client_id.to_string(),
           port: port,
           client_secret: client_secret.to_string(),
        }

    }

    /// Launches the new instance with the oauth metrhod from Previous Oauth Method Refresh Token
    pub async fn launch_oauth(&self) {
        let token = oauth::token(&self.refresh_token, &self.client_id, self.port, &self.client_secret);
    }

    /// Launches the new instance with the device code metrhod from Previous Device Code Method Refresh Token
    pub async fn launch_devicecode(&self) {
        println!(
            "{}",
            EXPERIEMNTAL_MESSAGE
        );
    }
}

/// Tests for the Framework for development
#[cfg(test)]
mod tests {
    use super::*;
    use dotenv_vault::dotenv;
    use std::env;

    #[tokio::test]
    async fn test_oauth_url() {
        let _ = dotenv();
        let client_id = env::var("Client_ID").expect("Expected Client ID");
        let oauth = Oauth::new(&client_id, None);
        let params = format!("client_id={}&response_type=code&redirect_uri=http://localhost:8000&response_mode=query&scope={}&state=12345", client_id, SCOPE);
        let expected_url = format!(
            "https://login.microsoftonline.com/consumers/oauth2/v2.0/authorize/?{}",
            params
        );
        assert_eq!(oauth.url(), expected_url);
    }

    #[tokio::test]
    async fn test_device_code_prelaunch() {
        let _ = dotenv();
        let client_id = env::var("Client_ID").expect("Expected Client ID.");
        let device_code = DeviceCode::new(&client_id).await.unwrap();

        // Act
        let (url, message, expires_in, user_code) = device_code.prelaunch();

        // Assert
        assert_eq!(url, device_code.url);
        assert_eq!(message, device_code.message);
        assert_eq!(expires_in, device_code.expires_in);
        assert_eq!(user_code, device_code.user_code);
    }
}
