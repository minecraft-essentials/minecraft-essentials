use crate::structs::VersionManifest;
use reqwest::{header::USER_AGENT, Client};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ManifestVersion {
    pub latest: Latest,
}

#[derive(Deserialize)]
pub struct Latest {}

pub async fn get_version_manifest(client: &Client, url: &str, user_agent: &str) {
    let result = client
        .get(url)
        .header(USER_AGENT, user_agent)
        .send()
        .await?;

    let version_manifest: VersionManifest = result.json().await?;
}
