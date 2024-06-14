mod dry;
use dry::Args;
use serde::Deserialize;

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

// Version Manifest

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VersionManifest {
    // #[serde(rename = "type")]
    pub(crate) arguments: Arguments,
    pub(crate) assetindex: AssetIndex,
}

#[derive(Deserialize, Clone)]
pub struct Arguments {
    pub(crate) game: Args,
    pub(crate) jvm: Args,
}

#[derive(Deserialize, Clone)]
pub struct AssetIndex {}
