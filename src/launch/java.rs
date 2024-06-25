use std::{fs, path::PathBuf};

use crate::{async_trait_alias::AsyncSendSync, errors::LaunchErrors};
use reqwest::Client;

use super::download_files;

// use super::download_files;
#[derive(Debug)]
pub enum JRE {
    Adoptium, // We only support this one for now more will come soon.
    Zulu,
    //TODO: More Java Runtime Enviroments (JRE) Supported to make it not limited
}

pub fn get_java(
    dir: &PathBuf,
    version: &str,
    jre: JRE,
    user_agent: &str,
) -> impl AsyncSendSync<Result<(), LaunchErrors>> {
    dbg!(&jre, &user_agent, &version, &dir);
    let url = match jre {
        JRE::Adoptium => {
            let _ = arch_support(vec!["x86_64", "x86", "aarch64", "arm"]);

            let java_url = arch_url(vec![
                ArchUrl {
                    arch: Some(String::from("x86_64")),
                    url: format!(
                        "https://api.adoptium.net/v3/binary/latest/{}/ga/{}/{}/jre/hotspot/normal/eclipse",
                        version,
                        std::env::consts::OS,
                        "x64"
                    ),
                },
                ArchUrl {
                    arch: None,
                    url: format!(
                        "https://api.adoptium.net/v3/binary/latest/{}/ga/{}/{}/jre/hotspot/normal/eclipse",
                        version,
                        std::env::consts::OS,
                        std::env::consts::ARCH
                    ),
                },
            ]).unwrap_or(String::from(""));
            java_url
        }
        JRE::Zulu => todo!(),
    };

    let mut dir_clone = dir.clone();
    let user_agent = user_agent.to_owned();
    // println!("{}", url);

    async move {
        if cfg!(target_os = "Windows") {
            dir_clone.push("jre.zip")
        } else {
            dir_clone.push("jre.tar.gz");
        }

        let _ = download_jre(url, dir_clone, &user_agent).await;

        Ok(())
    }

    //TODO: Donwload and extract JARs
}

async fn download_jre(url: String, dir: PathBuf, user_agent: &str) -> Result<(), LaunchErrors> {
    let client = Client::new();

    match download_files(client.clone(), user_agent, &dir, url).await {
        Ok(_) => {}
        Err(e) => {
            return Err(LaunchErrors::Requirements(format!(
                "Failed to download JRE Due to: {}",
                e
            )))
        }
    }

    if dir.exists() && dir.is_file() {
        let _ = fs::remove_file(dir);
    }
    Ok(())
}

struct ArchUrl {
    arch: Option<String>,
    url: String,
}

fn arch_url(arch: Vec<ArchUrl>) -> Option<String> {
    for arch in arch {
        match arch.arch {
            Some(archsep) => {
                if archsep == std::env::consts::ARCH {
                    return Some(arch.url);
                }
            }
            None => {
                return Some(arch.url);
            }
        }
    }
    None
}

// Archtechure support function that will error if wrong archtechure for all jre/jdk's
fn arch_support(supports: Vec<&str>) -> Result<(), LaunchErrors> {
    match std::env::consts::ARCH {
        supports => Ok(()),
        _ => Err(LaunchErrors::UnsupportedArchitecture(
            std::env::consts::ARCH.to_owned(),
        )),
    }
}
