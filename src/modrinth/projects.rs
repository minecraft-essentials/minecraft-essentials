use reqwest::Client;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ModrinthProject {
    pub title: String,
    pub description: String,
    pub categories: Vec<String>,
    pub client_side: Side,
    pub server_side: Side,
    pub body: String,
    status: String,
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

pub enum Side {
    Required,
    Optional,
    Unsupported,
}

impl Side {
    pub fn to_string(&self) -> String {
        match self {
            Side::Required => "required".to_owned(),
            Side::Optional => "optional".to_owned(),
            Side::Unsupported => "unsupported".to_owned(),
        }
    }
}

pub enum AppovalStatus {
    Approved,
    Pending,
    Rejected,
}

impl AppovalStatus {
    pub fn to_string(&self) -> String {
        match self {
            AppovalStatus::Approved => "approved".to_owned(),
            AppovalStatus::Pending => "pending".to_owned(),
            AppovalStatus::Rejected => "rejected".to_owned(),
        }
    }
}

pub async fn get_project(project: String) {
    let url = format!("{}/project/{}", MODRINTH_API, project);
    let res = Client::new().get(url).send().await.unwrap();
    let res_json = res.json::<ModrinthProject>().await.unwrap();
}
