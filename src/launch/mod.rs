mod java;
mod minecraft;
mod structs;

use std::{fs::File, io::Write, path::PathBuf};

use reqwest::{header::USER_AGENT, Client};

pub(crate) async fn download_files(
    client: Client,
    user_agent: &str,
    dir: &PathBuf,
    url: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let response = client
        .get(url)
        .header(USER_AGENT, user_agent)
        .send()
        .await?;

    let content = response.bytes().await?;

    let mut dest = File::create(dir)?;
    dest.write_all(&content)?;
    return Ok(());
}

pub(crate) async fn extract_files() {}

pub(crate) fn is_dir_empty(dir: &PathBuf) -> Result<bool, Box<dyn std::error::Error>> {
    Ok(dir.read_dir()?.next().is_none())
}

pub async fn get_minecraft(user_agent: &str, version: &str, jre: &str) -> Result<_, ()> {
    let client = Client::new();
    let manifest_res = client
        .get("https://piston-meta.mojang.com/mc/game/version_manifest_v2.json")
        .header(USER_AGENT, user_agent)
        .send()
        .await?;

    let manifest: structs::ManifestVersion = manifest_res.json().await?;
    let version_type = manifest.versions.version_type;
    let version_entry = &manifest.versions;
    let version_url: String = match version_entry.url {
        url => url.to_string(),
    };

    let version_manifest = client
        .get(&version_url)
        .header(USER_AGENT, user_agent)
        .send()
        .await?;

    let version_manifest_res: structs::VersionManifest = version_manifest.json().await?.clone();
}
