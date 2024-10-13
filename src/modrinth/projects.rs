use std::fmt::format;

use reqwest::Client;

use crate::errors::ModPlatformErrors;

use super::MODRINTH_API;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct ModrinthProject {
    pub title: String,
    pub description: String,
    pub categories: Vec<String>,
    pub client_side: String,
    pub server_side: String,
    pub body: String,
    pub status: String,
    pub additional_categories: Vec<String>,
    pub issues_url: Option<String>,
    pub source_url: Option<String>,
    pub wiki_url: Option<String>,
    pub discord_url: Option<String>,
    pub donation_urls: Vec<DonationUrl>,
    pub project_type: String,
    pub downloads: i64,
    pub icon_url: String,
    pub published: String,
    pub updated: String,
    pub approved: String,
    pub followers: i64,
    pub license: License,
    pub versions: Vec<String>,
    pub game_versions: Vec<String>,
    pub loaders: Vec<String>,
    pub body_url: Option<String>,
    pub queued: Option<bool>,
    pub moderator_message: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct License {
    pub id: String,
    pub name: String,
    pub url: Option<String>,
}

// Assuming DonationUrl was previously defined incorrectly, updating based on your JSON structure
#[derive(Serialize, Deserialize, Debug)]
pub struct DonationUrl {
    pub id: String,
    pub platform: String,
    pub url: String,
}

pub async fn get_project(
    project: &str,
    user_agent: &str,
) -> Result<ModrinthProject, ModPlatformErrors> {
    let url = format!("{}/project/{}", MODRINTH_API, project);
    let client = Client::new();
    let res = client
        .get(url)
        .header("User-Agent", user_agent)
        .send()
        .await
        .map_err(|err| {
            ModplatformErrors::RequestError("Failed to request to modrinth: ", err.to_string())
        })?;

    let modrinth_project: ModrinthProject = res.json().await.map_err(|err| {
        ModPlatformErrors::DeserializationError(format!(
            "Failed to deserialize modrinth error: {}",
            err
        ))
    })?;

    Ok(modrinth_project)
}
