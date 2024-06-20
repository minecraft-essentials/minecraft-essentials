use serde::Serialize;

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
