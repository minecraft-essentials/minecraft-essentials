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
#[cfg(test)]
mod tests;

#[cfg(feature = "custom-auth")]
mod auth;

#[cfg(feature = "custom-auth")]
pub use auth::mojang::AuthInfo as CustomAuthData;

#[cfg(feature = "custom-auth")]
use auth::{code, mojang, oauth, xbox};

#[cfg(feature = "custom-launch")]
use std::{
    io::{BufRead, BufReader},
    path::PathBuf,
    process::{Command, Stdio},
};

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
        auth::oauth(self.port, &self.client_id, client_secret, bedrock_relm).await
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
pub enum AuthType {
    /// OAuth authentication method.
    ///
    /// This variant is used for OAuth 2.0 authentication with Minecraft.
    #[cfg(feature = "custom-auth")]
    Oauth,

    /// Device Code authentication method.
    ///
    /// This variant is used for device code authentication with Minecraft.
    #[cfg(feature = "custom-auth")]
    DeviceCode,
    // /// Minecraft Device Code authentication method.
    // ///
    // /// This variant is specifically designed for Minecraft deafult device code authentication.
    // MinecraftDeviceCode,

    // /// Minecraft OAuth authentication method.
    // ///
    // /// This variant is specifically designed for Minecraft deafult OAuth authentication.
    // MinecraftOAuth,
}

/// Represents a builder for authentication configurations.
///
/// This struct is used to build your authentfcation with your type of authenfication options.
/// Deafults to `AuthType::OAuth` if no other option is specified.
/// It supports methods from `AuthType` and can be used to create a new instance of `Authentication` for your minecraft client launchers.
pub struct AuthenticationBuilder {
    auth_type: AuthType,
    client_id: String,
    port: Option<u16>,
    client_secrect: String,
    bedrockrel: Option<bool>,
}

impl AuthenticationBuilder {
    /// Creates a new instance of `AuthenticationBuilder`.
    pub fn builder() -> Self {
        Self {
            auth_type: AuthType::Oauth,
            client_id: "".to_string(),
            port: None,                     // Default port
            client_secrect: "".to_string(), // Default client_secret
            bedrockrel: None,               // Default bedrockrel
        }
    }
    pub fn of_type(&mut self, auth_type: AuthType) -> &mut Self {
        self.of_type(auth_type)
    }

    pub fn client_id(&mut self, client_id: &str) -> &mut Self {
        self.client_id(client_id);
        self
    }

    pub fn port(&mut self, port: Option<u16>) -> &mut Self {
        self.port = Some(port);
        self
    }

    pub fn client_secret(&mut self, client_secret: &str) -> &mut Self {
        self.client_secret(client_secret);
        self
    }

    pub fn bedrockrel(&mut self, bedrock_rel: bool) -> &mut Self {
        self.bedrockrel(bedrock_rel);
        self
    }

    pub async fn launch(&mut self) -> Result<CustomAuthData, Box<dyn std::error::Error>> {
        match self.auth_type {
            AuthType::Oauth => {
                auth::oauth(
                    self.port.unwrap_or(8000),
                    &self.client_id,
                    &self.client_secrect,
                    self.bedrockrel.is_some(),
                )
                .await
            }
            AuthType::DeviceCode => {}
        }
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
#[cfg(feature = "custom-launch")]
pub struct Launch {
    args: String,
    java_exe: String,
    jre: Option<PathBuf>,
}

#[cfg(feature = "custom-launch")]
impl Launch {
    /// Launches a new instance of the launch function.
    pub fn new(
        args: Vec<String>,
        java_exe: String,
        jre: Option<PathBuf>,
        offline: Option<bool>,
    ) -> Result<Self, errors::LaunchError> {
        let args_final = args.join(" ");
        print!("{}", args_final);

        if offline == Some(true)
            && !args_final.contains("--uuid")
            && !args_final.contains("--token")
        {
            return Err(errors::LaunchError::Requirements(
                "Either --uuid or --token is missing in the arguments.".to_string(),
            ));
        }

        Ok(Self {
            args: args_final,
            java_exe,
            jre,
        })
    }

    /// Returns the launch configuration information.
    ///
    /// This method provides access to the arguments, Java executable path, and the optional Java Runtime Environment (JRE) path
    /// that were used to initialize the `Launch` struct.
    ///
    /// # Returns
    ///
    /// * `(&str, &str, &Option<PathBuf>)` - A tuple containing the final arguments string, the path to the Java executable,
    /// and an optional path to the Java Runtime Environment.
    pub fn info(&self) -> (&str, &str, &Option<PathBuf>) {
        (&self.args, &self.java_exe, &self.jre)
    }

    /// Launches the Java Runtime Environment (JRE) with the specified arguments.
    ///
    /// This method is responsible for starting the Java Runtime Environment
    /// with the arguments provided during the initialization of the `Launch` struct.
    /// It is intended to be used for launching Minecraft or other Java applications.
    ///
    /// Required Args:
    /// - UUID: LauncherUUID
    /// - Token: BearerToken
    ///
    /// # Examples
    ///
    /// ```rust
    /// use minecraft_essentials::Launch;
    /// use std::path::Path;
    ///
    /// let jre_path = Path::new("/path/to/jre").to_path_buf();
    ///
    /// let launcher = Launch::new(vec!["-Xmx1024M --uuid --token".to_string()], "/path/to/java".to_string(), Some(jre_path), None).expect("Expected Launch");  
    /// launcher.launch_jre();
    /// ```
    pub fn launch_jre(&self) -> std::io::Result<()> {
        let command_exe = format!("{} {:?} {}", self.java_exe, self.jre, self.args);
        let mut command = Command::new(command_exe)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        // Optionally, you can handle stdout and stderr in real-time
        if let Some(ref mut stdout) = command.stdout {
            let reader = BufReader::new(stdout);
            for line in reader.lines() {
                println!("{}", line?);
            }
        }

        if let Some(ref mut stderr) = command.stderr {
            let reader = BufReader::new(stderr);
            for line in reader.lines() {
                eprintln!("{}", line?);
            }
        }

        // Wait for the command to finish
        command.wait()?;

        Ok(())
    }
}
