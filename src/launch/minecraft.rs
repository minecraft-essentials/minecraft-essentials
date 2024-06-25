use crate::{
    structs::{ManifestVersion, VersionManifest},
    MANIFEST_URL,
};
use reqwest::{header::USER_AGENT, Client};

pub async fn get_version_manifest(
    url: &str,
    user_agent: &str,
) -> Result<VersionManifest, Box<dyn std::error::Error>> {
    let client = Client::new();
    let result = client
        .get(url)
        .header(USER_AGENT, user_agent)
        .send()
        .await?;

    let version_manifest: VersionManifest = result.json().await?;

    Ok(version_manifest)
}

pub async fn get_manifest(user_agent: &str) -> Result<ManifestVersion, Box<dyn std::error::Error>> {
    let client = Client::new();

    let result = client
        .get(MANIFEST_URL)
        .header(USER_AGENT, user_agent)
        .send()
        .await?;

    let manifest: ManifestVersion = result.json().await?;

    Ok(manifest)
}
