use std::{fs, path::PathBuf};

use crate::{async_trait_alias::AsyncSendSync, errors::LaunchErrors};
use reqwest::Client;

use super::download_files;

// use super::download_files;

pub enum JRE {
    Adoptium, // We only support this one for now more will come soon.
              //TODO: More Java Runtime Enviroments (JRE) Supported to make it not limited
}

pub fn get_java(dir: &PathBuf, version: &str, jre: JRE, user_agent: &str) {
    if super::is_dir_empty(dir).expect("Expected Dir Checker") {
        download_java(dir, version, jre, user_agent)
    }
}

pub fn download_java(
    dir: &PathBuf,
    version: &str,
    jre: JRE,
    user_agent: &str,
) -> AsyncSendSync<Result<()>> {
    let url: String;
    let client = Client::new();

    match jre {
        JRE::Adoptium => {
            // arch_support(["x86_64", "x86", "aarch64", "arm"]);

            if std::env::consts::ARCH == "x86_64" {
                url = format!(
                    "https://api.adoptium.net/v3/binary/latest/{}/ga/{}/{}/jre/hotspot/normal/eclipse",
                    version,
                    std::env::consts::OS,
                    "x64"
                );
            } else {
                url = format!(
                    "https://api.adoptium.net/v3/binary/latest/{}/ga/{}/{}/jre/hotspot/normal/eclipse",
                    version,
                    std::env::consts::OS,
                    std::env::consts::ARCH
                );
            }
        }
    }

    let dir_clone = dir.clone();

    if cfg!(target_os = "Windows") {
        dir_clone.push("jre.zip")
    } else {
        dir_clone.push("jre.tar.gz")
    }

    async move {
        'out: {
            download_files(client, user_agent, dir, url).await?;
            fs::remove_file(dir_clone)?;
            Ok(())
        }
    }
    //TODO: Donwload and extract JARs
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
