use reqwest::Client;

use crate::errors::ModrinthErrors;

use super::MODRINTH_API;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ModrinthProject {
    pub title: String,
    pub description: String,
    pub categories: Vec<String>,
    pub client_side: String,
    pub server_side: String,
    pub body: String,
    pub status: String,
    pub additional_categories: Vec<String>,
    pub issues_url: String,
    pub source_url: String,
    pub wiki_url: String,
    pub discord_url: String,
    pub donation_urls: Vec<DonationUrl>,
    pub project_type: String,
    pub downloads: i64,
    pub icon_url: String,
    pub team: String,
    pub published: String,
    pub updated: String,
    pub approved: String,
    pub followers: i64,
    pub license: License,
    pub versions: Vec<String>,
    pub game_versions: Vec<String>,
    pub loaders: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct DonationUrl {
    pub id: String,
    pub platform: String,
    pub url: String,
}

#[derive(Serialize, Deserialize)]
pub struct License {
    pub id: String,
    pub name: String,
    pub url: String,
}

pub async fn get_project(
    project: &str,
    user_agent: &str,
) -> Result<ModrinthProject, ModrinthErrors> {
    let url = format!("{}/project/{}", MODRINTH_API, project);
    let res = Client::new()
        .get(url)
        .header("User-Agent", user_agent)
        .send()
        .await
        .map_err(|err| ModrinthErrors::RequestError(err.to_string()))?;
    let res_json: ModrinthProject = res
        .json()
        .await
        .map_err(|err| ModrinthErrors::DeserializationError(err.to_string()))?;

    Ok(res_json)
}
