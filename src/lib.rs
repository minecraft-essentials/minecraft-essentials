#![doc = include_str!("../README.md")]
#![forbid(unsafe_code, missing_docs)]
#![warn(clippy::pedantic)]

// Modules
pub(crate) mod async_trait_alias;
/// Error handling module for the Minecraft-Essentials library.
///
/// This module contains all the error types and related functionality
/// for error handling within the library.
pub mod errors;

#[cfg(feature = "custom-auth")]
mod custom;

#[cfg(feature = "custom-auth")]
use custom::{code, mojang, oauth, xbox};

pub use custom::mojang::AuthInfo as CustomAuthData;

// Constants
pub(crate) const SCOPE: &str = "XboxLive.signin%20XboxLive.offline_access";
pub(crate) const EXPERIMENTAL_MESSAGE: &str =
    "\x1b[33mNOTICE: You are using an experimental feature.\x1b[0m";

/// OAuth 2.0 Authentication
///
/// This struct represents the OAuth authentication process for Minecraft, specifically designed for use with custom Azure applications.
/// It is used to authenticate a user and obtain a token that can be used to launch Minecraft.
#[cfg(feature = "custom-auth")]
pub struct Oauth {
    url: String,
    port: u16,
    client_id: String,
}

#[cfg(feature = "custom-auth")]
impl Oauth {
    /// Initializes a new `Oauth` instance.
    ///
    /// This method sets up the OAuth authentication process by constructing the authorization URL
    /// and storing the client ID and port for later use.
    ///
    /// # Arguments
    ///
    /// * `client_id` - The client ID obtained from the Minecraft authentication service.
    /// * `port` - An optional port number for the local server. Defaults to 8000 if not provided.
    ///
    /// # Returns
    ///
    /// * `Self` - A new instance of `Oauth` configured with the provided client ID and port.
    pub fn new(client_id: &str, port: Option<u16>) -> Self {
        let port = port.unwrap_or(8000);
        let params = format!("client_id={}&response_type=code&redirect_uri=http://localhost:{}&response_mode=query&scope={}&state=12345", client_id, port, SCOPE);
        let url = format!(
            "https://login.microsoftonline.com/consumers/oauth2/v2.0/authorize/?{}",
            params
        );

        Self {
            url,
            port,
            client_id: client_id.to_string(),
        }
    }

    /// Retrieves the authorization URL.
    ///
    /// This method returns the URL that the user needs to visit to authorize the application.
    ///
    /// # Returns
    ///
    /// * `&str` - The authorization URL.
    pub fn url(&self) -> &str {
        &self.url
    }

    /// Launches Minecraft using the OAuth authentication process.
    ///
    /// This method completes the OAuth authentication process by launching a local server to
    /// receive the authorization code, exchanging it for an access token, and then using this token
    /// to launch Minecraft. The method supports both Bedrock Edition and Java Edition of Minecraft.
    ///
    /// # Arguments
    ///
    /// * `bedrock_relm` - A boolean indicating whether to launch the Bedrock Edition of Minecraft.
    /// * `client_secret` - The client secret obtained from the Minecraft authentication service.
    ///
    /// # Returns
    ///
    /// * `Result<CustomAuthData, Box<dyn std::error::Error>>` - A result containing the authentication data or an error if the process fails.
    pub async fn launch(
        &self,
        bedrock_relm: bool,
        client_secret: &str,
    ) -> Result<CustomAuthData, Box<dyn std::error::Error>> {
        let http_server = oauth::server(self.port)?.await?;
        let token = oauth::token(
            http_server
                .code
                .expect("\x1b[31mXbox Expected code.\x1b[0m")
                .as_str(),
            &self.client_id,
            self.port,
            client_secret,
        )
        .await?;

        let xbox = xbox::xbl(&token.access_token).await?;
        let xts = xbox::xsts_token(&xbox.token, bedrock_relm).await?;

        if bedrock_relm {
            Ok(CustomAuthData {
                access_token: "null".to_string(),
                uuid: "null".to_string(),
                expires_in: 0,
                xts_token: Some(xts.token),
            })
        } else {
            Ok(mojang::token(&xbox.display_claims.xui[0].uhs, &xts.token).await?)
        }
    }

    /// Refreshes the OAuth authentication process.
    ///
    /// This method is used to refresh the access token using the refresh token.
    ///
    /// # Arguments
    ///
    /// * `refresh_token` - The refresh token obtained from the Minecraft authentication service.
    /// * `client_id` - The client ID obtained from the Minecraft authentication service.
    /// * `port` - An optional port number for the local server. Defaults to 8000 if not provided.
    /// * `client_secret` - The client secret obtained from the Minecraft authentication service.
    ///
    /// # Returns
    ///
    /// * `Result<CustomAuthData, Box<dyn std::error::Error>>` - A result containing the refreshed authentication data or an error if the process fails.
    #[cfg(feature = "refresh")]
    pub async fn refresh(
        &self,
        refresh_token: &str,
        client_id: &str,
        port: Option<u16>,
        client_secret: &str,
    ) -> Result<CustomAuthData, Box<dyn std::error::Error>> {
        let port = port.unwrap_or(8000);
        let token = oauth::token(refresh_token, client_id, port, client_secret).await?;
        Ok(token)
    }
}

/// Device Code Authentication
///
/// This struct represents the device code authentication process for Minecraft, specifically designed for use with custom Azure applications.
/// It is used to authenticate a device and obtain a token that can be used to launch Minecraft.
#[cfg(feature = "custom-auth")]
pub struct DeviceCode {
    url: String,
    message: String,
    expires_in: u32,
    user_code: String,
    device_code: String,
    client_id: String,
}

#[cfg(feature = "custom-auth")]
impl DeviceCode {
    /// Initializes a new `DeviceCode` instance.
    ///
    /// This method starts the device code authentication process by making an asynchronous request
    /// to the authentication server. It returns a future that resolves to a `Result` containing the
    /// `DeviceCode` instance on success or an error if the request fails.
    ///
    /// # Arguments
    ///
    /// * `client_id` - The client ID obtained from the Minecraft authentication service.
    ///
    /// # Returns
    ///
    /// * `impl async_trait_alias::AsyncSendSync<Result<Self, reqwest::Error>>` - A future that resolves to a `Result` containing the `DeviceCode` instance or an error.
    pub fn new(
        client_id: &str,
    ) -> impl async_trait_alias::AsyncSendSync<Result<Self, reqwest::Error>> {
        println!("{}", EXPERIMENTAL_MESSAGE);
        let client_id_str = client_id.to_string();
        async move {
            let response_data = code::device_authentication_code(&client_id_str).await?;

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

    /// Provides pre-launch information.
    ///
    /// This method returns a tuple containing the verification URL, the message to display to the user,
    /// the expiration time of the device code, and the user code. This information is useful for guiding
    /// the user through the device code authentication process.
    ///
    /// # Returns
    ///
    /// * `(&str, &str, u32, &str)` - A tuple containing the verification URL, the message, the expiration time, and the user code.
    pub fn preinfo(&self) -> (&str, &str, u32, &str) {
        (&self.url, &self.message, self.expires_in, &self.user_code)
    }

    /// Launches Minecraft using the device code authentication process.
    ///
    /// This method completes the device code authentication process by authenticating the device
    /// and obtaining a token. It then uses this token to launch Minecraft. The method supports both
    /// Bedrock Edition and Java Edition of Minecraft.
    ///
    /// # Arguments
    ///
    /// * `bedrock_relm` - A boolean indicating whether to launch the Bedrock Edition of Minecraft.
    ///
    /// # Returns
    ///
    /// * `Result<CustomAuthData, Box<dyn std::error::Error>>` - A result containing the authentication data or an error if the process fails.
    pub async fn launch(
        &self,
        bedrock_relm: bool,
    ) -> Result<CustomAuthData, Box<dyn std::error::Error>> {
        let token = code::authenticate_device(&self.device_code, &self.client_id).await?;
        let xbox = xbox::xbl(&token.token).await?;
        let xts = xbox::xsts_token(&xbox.token, bedrock_relm).await?;

        if bedrock_relm {
            Ok(CustomAuthData {
                access_token: "null".to_string(),
                uuid: "null".to_string(),
                expires_in: 0,
                xts_token: Some(xts.token),
            })
        } else {
            Ok(mojang::token(&xbox.display_claims.xui[0].uhs, &xts.token).await?)
        }
    }

    /// Refreshes the device code authentication process.
    ///
    /// This method is marked as experimental and currently does not perform any actions.
    ///
    /// # Note
    ///
    /// This method is intended for future use when implementing refresh functionality for the device code authentication process.
    pub async fn refresh(&self) {
        println!("{}", EXPERIMENTAL_MESSAGE);
    }
}

/// `Launch` struct represents the configuration for launching a Minecraft client.
///
/// This struct holds the arguments required to launch the Minecraft client. The arguments are passed as a single string,
/// which can include various options supported by the Minecraft client.
pub struct Launch {
    args: String,
}

pub(crate) const ADOPITUM_API_URL: &str = "https://api.adopitum.com";

impl Launch {
    /// Launches a new instance of the launch function.
    pub fn new(clientid: &str, uuid: &str, username: &str, server: Option<&str>) {
        let mut arg_vec: Vec<String> = Vec::new();

        arg_vec.push(format!("--clientid {}", clientid));
        arg_vec.push(format!("--uuid {}", uuid));
        arg_vec.push(format!("--username {}", username));

        if let Some(server) = server {
            arg_vec.push(format!("--server {}", server))
        }
    }

    /// Launches a new instance of the launch function with custom args.
    pub fn new_customargs(args: &str) {}

    /// This function downloads java for you minecraft client if needed you can do a custom java via a link.
    ///
    /// Java: Defaults to Temurin JRE {{version}}
    pub fn download_java(java_version: &str, download_url: Option<&str>) {
       
    }
}

// Tests
#[cfg(test)]
mod tests;

/// Deprecated Refresh Bearer
///
/// This struct is deprecated and intended for future use. It was previously used for refreshing tokens
/// in the context of Minecraft authentication. However, it has been deprecated in favor of using the
/// `oauth::refresh` or `devicecode::refresh` functions for refreshing tokens.
///
/// # Note
///
/// This functionality will be removed in a future release. Developers are advised to use the
/// recommended refresh functions instead.
#[cfg(feature = "deperacted")]
#[deprecated(
    since = "0.2.8",
    note = "This functionality has been deprecated. Please use the `oauth::refresh` or `devicecode::refresh` functions for refreshing tokens in the future. This feature will be removed in 0.2.10"
)]
pub struct RefreshBearer {
    refresh_token: String,
    client_id: String,
    port: u16,
    client_secret: String,
}

#[cfg(feature = "deperacted")]
impl RefreshBearer {
    /// Initializes a new `RefreshBearer` instance.
    ///
    /// This method was used to set up the refresh process by storing the refresh token, client ID,
    /// port, and client secret for later use.
    ///
    /// # Arguments
    ///
    /// * `refresh_token` - The refresh token obtained from the Minecraft authentication service.
    /// * `client_id` - The client ID obtained from the Minecraft authentication service.
    /// * `port` - An optional port number for the local server. Defaults to 8000 if not provided.
    /// * `client_secret` - The client secret obtained from the Minecraft authentication service.
    ///
    /// # Returns
    ///
    /// * `Self` - A new instance of `RefreshBearer` configured with the provided refresh token, client ID, port, and client secret.
    pub fn new(
        refresh_token: &str,
        client_id: &str,
        port: Option<u16>,
        client_secret: &str,
    ) -> Self {
        let port = port.unwrap_or(8000);
        Self {
            refresh_token: refresh_token.to_string(),
            client_id: client_id.to_string(),
            port: port,
            client_secret: client_secret.to_string(),
        }
    }

    /// A placeholder method for future refresh functionality.
    ///
    /// This method is marked as deprecated and does not perform any actions. It is intended to be replaced
    /// by the recommended refresh functions for OAuth and device code authentication processes.
    ///
    /// # Returns
    ///
    /// * `()` - No return value.
    pub async fn launch_oauth(&self) {
        // Placeholder implementation
    }

    /// A placeholder method for future refresh functionality.
    ///
    /// This method is marked as deprecated and does not perform any actions. It is intended to be replaced
    /// by the recommended refresh functions for OAuth and device code authentication processes.
    ///
    /// # Returns
    ///
    /// * `()` - No return value.
    pub async fn launch_devicecode(&self) {
        // Placeholder implementation
    }
}
