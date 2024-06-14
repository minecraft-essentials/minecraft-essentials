#![doc = include_str!("../README.md")]
#![forbid(unsafe_code, missing_docs)]
#![warn(clippy::pedantic)]
#![allow(deprecated)] // For now we're using deprecated features for both DeviceCode and Oauth impl.

// Modules
pub(crate) mod async_trait_alias;
/// Error handling module for the Minecraft-Essentials library.
///
/// This module contains all the error types and related functionality
/// for error handling within the library.
pub mod errors;
#[cfg(test)]
mod tests;

#[cfg(feature = "launch")]
mod launch;

#[cfg(feature = "auth")]
mod auth;

#[cfg(feature = "auth")]
pub use auth::AuthInfo as CustomAuthData;

#[cfg(feature = "auth")]
use auth::{
    bearer_token,
    microsoft::{authenticate_device, device_authentication_code, ouath, ouath_token, SCOPE},
    xbox::{xbl, xsts},
};

#[cfg(feature = "custom-launch")]
use std::{
    io::{BufRead, BufReader},
    path::PathBuf,
    process::{Command, Stdio},
};

// Constants
pub(crate) const EXPERIMENTAL_MESSAGE: &str =
    "\x1b[33mNOTICE: You are using an experimental feature.\x1b[0m";

/// OAuth 2.0 Authentication
///
/// This struct represents the OAuth authentication process for Minecraft, specifically designed for use with custom Azure applications.
/// It is used to authenticate a user and obtain a token that can be used to launch Minecraft.
#[cfg(feature = "auth")]
pub struct Oauth {
    url: String,
    port: u16,
    client_id: String,
}

#[cfg(feature = "auth")]
#[deprecated(
    note = "The Ouath implementation is deprecated. Please migrate to AuthenticationBuilder and utilize the Oauth type for authentication.",
    since = "0.2.13"
)]
// TODO: REMOVE THIS AT 0.2.15
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
        AuthenticationBuilder::builder()
            .bedrockrel(Some(bedrock_relm))
            .of_type(AuthType::Oauth)
            .client_secret(client_secret)
            .client_id(&self.client_id)
            .port(Some(self.port))
            .launch()
            .await
    }

    /// Launches Oauth using method which fixes 500 issue.
    ///
    /// This method is intended for use with Tauri applications to launch the Ouath Method.
    /// It handles the necessary fixes required
    /// to launch the Oauth within a Tauri application.
    ///
    /// * `bedrock_relm` - A boolean indicating whether to launch the Bedrock Edition of Minecraft.
    /// * `client_secret` - The client secret obtained from the Minecraft authentication service.
    ///
    /// # Returns
    ///
    /// * `Result<CustomAuthData, Box<dyn std::error::Error>>` - A result containing the authentication data or an error if the process fails.

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

/// Represents the type of authentication to be used.
///
/// This enum is used to specify the authentication method for Minecraft client launchers.
/// It supports OAuth, Device Code, Minecraft Device Code, and Minecraft OAuth authentication methods.
#[cfg(feature = "auth")]
pub enum AuthType {
    /// OAuth authentication method.
    ///
    /// This variant is used for OAuth 2.0 authentication with Minecraft.
    #[cfg(feature = "auth")]
    Oauth,

    /// Device Code authentication method.
    ///
    /// This variant is used for device code authentication with Minecraft.
    #[cfg(feature = "auth")]
    DeviceCode,
}

/// Represents a builder for authentication configurations.
///
/// This struct is used to build your authentfcation with your type of authenfication options.
/// Deafults to `AuthType::OAuth` if no other option is specified.
/// It supports methods from `AuthType` and can be used to create a new instance of `Authentication` for your minecraft client launchers.
#[cfg(feature = "auth")]
pub struct AuthenticationBuilder {
    auth_type: AuthType,
    client_id: String,
    port: u16,
    client_secrect: String,
    bedrockrel: bool,
}

#[cfg(feature = "auth")]
impl AuthenticationBuilder {
    /// Creates a new instance of `AuthenticationBuilder`.
    pub fn builder() -> Self {
        Self {
            auth_type: AuthType::Oauth,
            client_id: "".to_string(),
            port: 8000,
            client_secrect: "".to_string(),
            bedrockrel: false,
        }
    }

    /// Type of authentication.
    ///
    /// Sets the type of authentication to be used.
    pub fn of_type(&mut self, auth_type: AuthType) -> &mut Self {
        self.auth_type = auth_type;
        self
    }

    /// Client ID from your application Required for `OAuth` & `DeviceCode`.
    pub fn client_id(&mut self, client_id: &str) -> &mut Self {
        self.client_id = client_id.to_string();
        self
    }

    /// Port for the Temporary https Required for `OAuth``.
    pub fn port(&mut self, port: Option<u16>) -> &mut Self {
        self.port = port.unwrap_or(8000);
        self
    }

    /// Client Secret from your application Required for `OAuth` & `DeviceCode`.
    pub fn client_secret(&mut self, client_secret: &str) -> &mut Self {
        self.client_secrect = client_secret.to_string();
        self
    }

    /// Bedrock relm related that only need xts token not bearer.
    pub fn bedrockrel(&mut self, bedrock_rel: Option<bool>) -> &mut Self {
        self.bedrockrel = bedrock_rel.unwrap_or(false);
        self
    }

    /// Launchs the authentication process.
    pub async fn launch(&mut self) -> Result<CustomAuthData, Box<dyn std::error::Error>> {
        match self.auth_type {
            AuthType::Oauth => {
                let server = ouath(self.port)?.await?;
                let server_token = ouath_token(
                    server
                        .code
                        .expect("\x1b[31mXbox Expected code.\x1b[0m")
                        .as_str(),
                    &self.client_id,
                    self.port,
                    &self.client_secrect,
                )
                .await?;
                let xbl = xbl(&server_token.access_token).await?;
                let xts = xsts(&xbl.token, self.bedrockrel).await?;

                if self.bedrockrel {
                    Ok(CustomAuthData {
                        access_token: None,
                        uuid: None,
                        expires_in: 0,
                        xts_token: Some(xts.token),
                    })
                } else {
                    Ok(bearer_token(&xbl.display_claims.xui[0].uhs, &xts.token).await?)
                }
            }
            AuthType::DeviceCode => {
                print!("{} \n Status: WIP (Work In Progress)", EXPERIMENTAL_MESSAGE);
                let code = device_authentication_code(&self.client_id).await?;
                let code_token = authenticate_device(&code.device_code, &self.client_id).await?;
                let xbl = xbl(&code_token.token).await?;
                let xts = xsts(&xbl.token, self.bedrockrel).await?;

                if self.bedrockrel {
                    Ok(CustomAuthData {
                        access_token: None,
                        uuid: None,
                        expires_in: 0,
                        xts_token: Some(xts.token),
                    })
                } else {
                    Ok(bearer_token(&xbl.display_claims.xui[0].uhs, &xts.token).await?)
                }
            }
        }
    }
}

/// Device Code Authentication
///
/// This struct represents the device code authentication process for Minecraft, specifically designed for use with custom Azure applications.
/// It is used to authenticate a device and obtain a token that can be used to launch Minecraft.
#[cfg(feature = "auth")]
// TODO: REMOVE THIS AT 0.2.15
#[deprecated(
    note = "The Device implementation is deprecated. Please migrate to AuthenticationBuilder and utilize the DeviceCode type for authentication.",
    since = "0.2.13"
)]
pub struct DeviceCode {
    url: String,
    message: String,
    expires_in: u32,
    user_code: String,
    device_code: String,
    client_id: String,
}

#[cfg(feature = "auth")]
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
            let response_data = device_authentication_code(&client_id_str).await?;

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
        let code = authenticate_device(&self.device_code, &self.client_id).await?;
        let xbl = xbl(&code.token).await?;
        let xts = xsts(&xbl.token, bedrock_relm).await?;

        if bedrock_relm {
            Ok(CustomAuthData {
                access_token: None,
                uuid: None,
                expires_in: 0,
                xts_token: Some(xts.token),
            })
        } else {
            Ok(bearer_token(&xbl.display_claims.xui[0].uhs, &xts.token).await?)
        }
    }

    /// Refreshes the device code authentication process.
    ///
    /// This method was previously used to manually refresh the device code authentication process but has been superseded by the `AuthenticationBuilder`. The new approach allows for more flexible and streamlined authentication flow management.
    ///
    /// To refresh the authentication, users should now utilize the `AuthenticationBuilder` and call the `.refresh_type(AuthType::DeviceCode)` method. This method automatically handles the necessary steps to refresh the authentication state according to the selected authentication type.
    ///
    /// # Removal Notice
    ///
    /// Effective immediately, this method is marked as deprecated and will be removed in the next minor version release. Users are strongly encouraged to migrate to using the `AuthenticationBuilder` for refreshing authentication states before upgrading to the next version.
    #[deprecated(
        note = "Effective immediately, please migrate to using the `AuthenticationBuilder` and its `.refresh_type(AuthType::DeviceCode)` method for refreshing the authentication state. This method will be removed in the next minor version release.",
        since = "0.2.13"
    )]
    pub async fn refresh(&self) {
        println!("This method is deprecated and will be removed in the next minor version. Please refer to the updated documentation on using the `AuthenticationBuilder`.");
    }
}
