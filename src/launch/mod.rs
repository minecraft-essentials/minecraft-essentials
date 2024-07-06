mod java;
mod minecraft;

pub use java::JRE as JavaJRE;
use reqwest::{header::USER_AGENT, Client};
use std::{fs::File, io::Write, path::PathBuf};

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

pub(crate) async fn launch_minecraft(
    args: Vec<String>,
    dir: &PathBuf,
    user_agent: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let manifest = minecraft::get_manifest(user_agent).await?;
    let version_url = manifest.versions.url.clone();
    let version_manifest = minecraft::get_version_manifest(&version_url, user_agent).await?;

    println!("Version Manifest: {:#?}", version_manifest);

    Ok(())
}
