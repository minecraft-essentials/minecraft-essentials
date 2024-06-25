mod java;
mod minecraft;

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
