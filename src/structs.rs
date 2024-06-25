use serde::{Deserialize, Serialize};

// Dry (Do not Repeat Yourself)

#[derive(Deserialize, Clone, Debug)]
pub enum Args {
    Simple(String),
    ComplexArgs,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ComplexArgs {
    pub(crate) rules: Vec<Rules>,
    pub(crate) value: Vec<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Rules {
    pub(crate) action: String,
    pub(crate) features: Option<RulesFutures>,
    pub(crate) os: Option<RulesOS>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct RulesFutures {}

#[derive(Deserialize, Clone, Debug)]
pub struct RulesOS {
    pub(crate) name: String,
    pub(crate) arch: String,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FileIndex {
    pub(crate) id: Option<String>,
    pub(crate) sha1: Option<String>,
    pub(crate) path: Option<String>,
    pub(crate) size: Option<i32>,
    pub(crate) totalsize: Option<i32>,
    pub(crate) url: Option<String>,
}

/////////....../////

// Manifest Json

#[derive(Deserialize, Debug, Clone)]
pub struct ManifestVersion {
    pub latest: LatestVersion,
    pub versions: VersionEntry,
}

#[derive(Deserialize, Debug, Clone)]
pub struct LatestVersion {
    pub release: String,
    pub snapshot: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct VersionEntry {
    pub id: String,
    #[serde(rename = "type")]
    pub version_type: String,
    pub url: String,
    pub time: String,
    pub release_time: String,
    pub sha1: String,
    pub compliance_level: i32,
}

/////////....../////

// Version Manifest

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VersionManifest {
    // #[serde(rename = "type")]
    pub(crate) arguments: Arguments,
    pub(crate) assetindex: FileIndex,
    pub(crate) assets: String,
    pub(crate) compliancelevel: i32,
    pub(crate) downloads: Downloads,
    pub(crate) id: String,
    pub(crate) javaversion: JavaVersion,
    pub(crate) libraries: Vec<Libraries>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Arguments {
    pub(crate) game: Args,
    pub(crate) jvm: Args,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JavaVersion {
    pub(crate) component: String,
    pub(crate) majorversion: i32,
    pub(crate) minorversion: i32,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Libraries {
    pub(crate) donwloads: FileIndex,
    pub(crate) name: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Logging {
    pub(crate) argument: String,
    pub(crate) file: FileIndex,
    #[serde(rename = "type")]
    pub(crate) log_type: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Downloads {
    pub(crate) artiface: Option<FileIndex>,
    pub(crate) client: Option<FileIndex>,
    pub(crate) client_mappings: Option<FileIndex>,
    pub(crate) server: Option<FileIndex>,
    pub(crate) server_mappings: Option<FileIndex>,
}

/////////....../////

/// Game Arguments for Minecraft
#[derive(Serialize)]
pub struct GameArguments {
    /// ClientId
    pub client_id: Option<String>,
    /// Username (Not Recommended)
    pub username: Option<String>,
    /// Version of Minecraft
    pub version: Option<String>,
    /// UUID
    pub uuid: Option<String>,

    /// Game Directory
    pub game_directory: Option<String>,
    /// Window Size
    pub window_size: Option<(i32, i32)>,
    /// Quick Play
    pub quick_play: Option<QuickPlayArguments>,
}

/// Represents arguments for quick play options in Minecraft.
#[derive(Serialize)]
pub enum QuickPlayArguments {
    /// Singleplayer
    SinglePlayer(String),
    /// Multiplayer
    MultiPlayer(String),
    /// None
    None,
}

/// Java Arguments for Minecraft
#[derive(Serialize)]
pub struct JavaArguments {
    /// Minimal Memory
    pub min_memory: Option<i32>,
    /// Maximum Memory
    pub max_memory: i32,
    /// Launcher Name
    pub launcher_name: Option<String>,
    /// Launcher Version
    pub launcher_version: Option<String>,
    /// Class Path
    pub class_path: Option<String>,
}
