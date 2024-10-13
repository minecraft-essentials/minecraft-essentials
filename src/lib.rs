#![doc = include_str!("../README.md")]
#![forbid(unsafe_code, missing_docs)]
#![warn(clippy::pedantic)]

// Modules
/// Error handling module for the Minecraft-Essentials library.
///
/// This module contains all the error types and related functionality
/// for error handling within the library.
pub mod errors;
pub(crate) mod trait_alias;

/// Structs module for the Minecraft-Essentials library.
///
/// This module contains all the structs and related functionality
/// for structs within the library.
pub mod structs;
#[cfg(test)]
mod tests;

#[cfg(feature = "launch")]
/// Launch module for the Minecraft-Essentials library.
pub mod launch;

#[cfg(feature = "auth")]
mod auth;

#[cfg(feature = "modrinth")]
mod modrinth;

use std::path::PathBuf;

use auth::microsoft::CodeResponse;
#[cfg(feature = "auth")]
pub use auth::AuthInfo as CustomAuthData;

#[cfg(feature = "auth")]
use auth::{
    bearer_token,
    microsoft::{authenticate_device, device_authentication_code, ouath, ouath_token, SCOPE},
    xbox::{xbl, xsts},
};
#[cfg(feature = "launch")]
use launch::JavaJRE;
use serde::{Deserialize, Serialize};
use trait_alias::Optional;

// Constants
pub(crate) const EXPERIMENTAL_MESSAGE: &str =
    "\x1b[33mNOTICE: You are using an experimental feature.\x1b[0m";

#[cfg(feature = "launch")]
pub(crate) const MANIFEST_URL: &str =
    "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";

/// OAuth 2.0 Authentication
///
/// This struct represents the OAuth authentication process for Minecraft, specifically designed for use with custom Azure applications.
/// It is used to authenticate a user and obtain a token that can be used to launch Minecraft.
#[cfg(feature = "auth")]
#[deprecated(
    note = "The Ouath implementation is deprecated. Please migrate to AuthenticationBuilder and utilize the Oauth type for authentication.",
    since = "0.2.12"
)]
pub struct Oauth {
    port: u16,
    client_id: String,
}

#[cfg(feature = "auth")]
#[deprecated(
    note = "The Ouath implementation is deprecated. Please migrate to AuthenticationBuilder and utilize the Oauth type for authentication.",
    since = "0.2.12"
)]
// TODO: REMOVE THIS AT 0.2.14
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
        Self {
            client_id: client_id.to_string(),
            port,
        }
    }

    /// Retrieves the authorization URL.
    ///
    /// This method returns the URL that the user needs to visit to authorize the application.
    ///
    /// # Returns
    ///
    /// * `&str` - The authorization URL.
    pub async fn url(&self) -> String {
        let mut builder = AuthenticationBuilder::builder();
        builder
            .port(self.port)
            .client_id(&self.client_id)
            .of_type(AuthType::Oauth);
        let auth_info = builder
            .get_info()
            .await
            .ouath_url
            .expect("Expected OAuth URL");
        auth_info.clone()
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
#[derive(PartialEq, Debug)]
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

/// Represents authentication information.
///
/// This struct holds the necessary information for authentication processes,
/// such as device codes and OAuth URLs that you'll recive.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthInfo {
    /// Device Code Info that you will recive based on AuthType::DeviceCode.
    pub device_code: Option<CodeResponse>,
    /// OAuth URL that you will recive based on AuthType::OAuth.
    pub ouath_url: Option<String>,
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
    pub fn port<T: Optional<u16>>(&mut self, port: T) -> &mut Self {
        let port = match port.into() {
            Some(port) => port,
            None => 8000,
        };
        self.port = port;
        self
    }

    /// Client Secret from your application Required for `OAuth` & `DeviceCode`.
    pub fn client_secret(&mut self, client_secret: &str) -> &mut Self {
        self.client_secrect = client_secret.to_string();
        self
    }

    /// Bedrock relm related that only need xts token not bearer.
    pub fn bedrockrel<T: Optional<bool>>(&mut self, bedrock_rel: T) -> &mut Self {
        let bedrock_rel = match bedrock_rel.into() {
            Some(bedrock_rel) => bedrock_rel,
            None => false,
        };
        self.bedrockrel = bedrock_rel;
        self
    }

    /// Gets the code for device code method
    pub async fn get_info(&mut self) -> AuthInfo {
        if self.auth_type == AuthType::DeviceCode {
            let code = device_authentication_code(&self.client_id).await.unwrap();
            AuthInfo {
                device_code: Some(code),
                ouath_url: None,
            }
        } else {
            let url = format!("https://login.microsoftonline.com/consumers/oauth2/v2.0/authorize/?client_id={}&response_type=code&redirect_uri=http://localhost:{}&response_mode=query&scope={}&state=12345", self.client_id, self.port, SCOPE);
            AuthInfo {
                device_code: None,
                ouath_url: Some(url),
            }
        }
    }

    /// Launchs the authentication process.
    pub async fn launch(&mut self) -> Result<CustomAuthData, Box<dyn std::error::Error>> {
        dbg!(&self.auth_type, &self.client_id);
        match self.auth_type {
            AuthType::Oauth => {
                dbg!(&self.client_secrect, self.port);
                print!("{}", self.client_id);
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

/// A builder that launches minecraft or your own custom client.
#[cfg(feature = "launch")]
pub struct LaunchBuilder {
    args: Vec<String>,
    java_path: Option<PathBuf>,
    client: Option<PathBuf>,
    mods: Option<Vec<PathBuf>>,
}

/// Launch Directories for minecraft
#[cfg(feature = "launch")]
pub struct LauncherDirs {
    /// Game Directory
    pub game_dir: PathBuf,
    /// Assets Directory
    pub assets_dir: PathBuf,
    /// Libraries Directory
    pub libraries_dir: PathBuf,
    /// Natives Directory
    pub natives_dir: PathBuf,
    /// Java Directory
    pub java_dir: PathBuf,
}

#[cfg(feature = "launch")]
impl LaunchBuilder {
    /// Create a new instanve of `LaunchBuilder`.
    pub fn init() -> Self {
        Self {
            args: Vec::new(),
            java_path: None,
            client: None,
            mods: None,
        }
    }

    /// Set the Java Arguments for the Minecraft.
    pub fn args(&mut self, args: Vec<String>) -> &mut Self {
        dbg!(&args);
        self.args = args;
        self
    }

    /// Set the Java Path for the Minecraft.
    /// FOR CUSTOM JAVA ONLY
    pub fn java(&mut self, path: Option<PathBuf>) -> &mut Self {
        self.java_path = path;
        self
    }

    /// Set for Custom Minecraft Client
    pub fn client(&mut self, client: Option<PathBuf>) -> &mut Self {
        self.client = client;
        self
    }

    /// Set for Custom Minecraft Mods to include in the launch.s
    pub fn mods(&mut self, mods: Option<Vec<PathBuf>>) -> &mut Self {
        self.mods = mods;
        self
    }

    /// Launches the Minecraft/Your Client!
    /// `jre` is required if you use custom java.
    pub async fn launch(&mut self, jre: Option<JavaJRE>, dirs: Option<LauncherDirs>) {
        if cfg!(target_os = "macos") {
            self.args.push(format!("-XstartOnFirstThread"));
        }
    }
}

/// Launch Args
///
/// This struct is used to build the launch arguments for your Minecraft client.
/// It provides a builder-like interface for setting various launch arguments.
#[cfg(feature = "launch")]
pub struct LaunchArgs {
    args: Vec<String>,
}

#[cfg(feature = "launch")]
impl LaunchArgs {
    /// Creates a new LaunchArgs builder
    pub fn builder() -> Self {
        Self { args: Vec::new() }
    }

    /// Sets general game settings
    /// window_size: width, height
    /// launcher_branding: name, version
    /// version_input: version, version_type
    /// memory: min, max (MB)
    pub fn game_settings(
        &mut self,
        window_size: Option<(u32, u32)>,
        launcher_branding: Option<(String, String)>,
        mut dirs: LauncherDirs,
        version_input: Option<(String, String)>,
        memory: (u16, Option<u16>),
    ) {
        self.args.push(format!("--Xms{}m", memory.0));
        if let Some(branding) = launcher_branding {
            self.args
                .push(format!("-Dminecraft.launcher.brand={}", branding.0));
            self.args
                .push(format!("--Dminecraft.launcher.version={}", branding.1));
        }

        if let Some(version) = version_input {
            self.args.push(format!("--version {}", version.0));
            self.args.push(format!("--versionType {}", version.1));
        }

        if let Some(window_size) = window_size {
            self.args.push(format!("--width {}", window_size.0));
            self.args.push(format!("--height {}", window_size.1));
        }

        if let Some(memory) = memory.1 {
            self.args.push(format!("--Xmx{}M", memory));
        }

        self.args
            .push(format!("--gameDir {}", dirs.game_dir.display()));
        self.args
            .push(format!("--assetsDir {}", dirs.assets_dir.display()));
        self.args.push(format!(
            "--assetsIndex {:?}",
            dirs.assets_dir.push("/index.json")
        ));
        self.args
            .push(format!("--librariesDir {}", dirs.libraries_dir.display()));
        self.args
            .push(format!("--nativesDir {}", dirs.natives_dir.display()));
        self.args
            .push(format!("--javaDir {}", dirs.java_dir.display()));
    }
}

/// Device Code Authentication
///
/// This struct represents the device code authentication process for Minecraft, specifically designed for use with custom Azure applications.
/// It is used to authenticate a device and obtain a token that can be used to launch Minecraft.
#[cfg(feature = "auth")]
// TODO: REMOVE THIS AT 0.2.14
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
#[deprecated(
    note = "The Ouath implementation is deprecated. Please migrate to AuthenticationBuilder and utilize the Oauth type for authentication.",
    since = "0.2.12"
)]
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
    /// * `impl trait_alias::AsyncSendSync<Result<Self, reqwest::Error>>` - A future that resolves to a `Result` containing the `DeviceCode` instance or an error.
    pub fn new(client_id: &str) -> impl trait_alias::AsyncSendSync<Result<Self, reqwest::Error>> {
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
        since = "0.2.12"
    )]
    pub async fn refresh(&self) {
        println!("This method is deprecated and will be removed in the next minor version. Please refer to the updated documentation on using the `AuthenticationBuilder`.");
    }
}

/// Modrinth API Implementation for your Minecraft Modpack Launcher.
pub struct Modrinth {
    access_token: String,
    user_agent: String,
}

#[cfg(feature = "modrinth")]
/// Github Repo Settings for Modrinth
pub struct GithubModrinth {
    /// Used as a user Agent to uniquely identify your app or something.
    pub owner: String,
    /// Used as a user Agent to uniquely identify your app or something.
    pub repo: String,
    /// Your Project/Program Version
    pub project_version: String,
    /// Used to authenticate with Modrinth API
    /// Optional if you are using something that doesn't require authentication.
    pub access_token: Option<String>,
}

#[cfg(feature = "modrinth")]
impl Modrinth {
    /// Create a new instance of using Modrinth API.
    pub fn init(github: GithubModrinth, contact_email: String) -> Self {
        let user_agent = format!(
            "{}/{}/{} ({})",
            github.owner, github.repo, github.project_version, contact_email
        );
        Self {
            access_token: github.access_token.unwrap_or("".to_string()),
            user_agent,
        }
    }

    /// Get a project from Modrinth.
    pub async fn get_project(
        &self,
        project: &str,
    ) -> Result<modrinth::projects::ModrinthProject, errors::ModrinthErrors> {
        modrinth::projects::get_project(project, &self.user_agent).await
    }
}
