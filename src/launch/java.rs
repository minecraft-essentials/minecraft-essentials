use std::{env, fs, path::PathBuf};

use crate::{errors::LaunchErrors, trait_alias::AsyncSendSync};
use reqwest::Client;

use super::download_files;

// use super::download_files;
/// Java Runtime Environment (JRE) for Minecraft.
#[derive(Debug, Clone)]
pub enum JRE {
    /// Adoptium or known as Eclipse Temurin is the Java Runtime Environment (JRE) by Eclipse Foundation.
    /// It is open-source and free to use for all Java projects.
    Adoptium,
    /// Zulu is a free and closed-sourced Java Runtime Environment (JRE) by Azul Systems.
    /// It is not open-source but free to use for all Java projects.
    /// WIP: Not Supported Yet
    Zulu,
    /// GraalVM is a free and open-source Java Runtime Environment (JRE) by Oracle.
    /// It is open-source and free to use for all Java projects.
    GraalVM,
    //TODO: More Java Runtime Enviroments (JRE) Supported to make it not limited
}

fn java_url(jre: JRE, version: &str) -> String {
    match jre {
        JRE::Adoptium => {
            let _ = arch_support(vec!["x86_64", "x86", "aarch64", "arm"]);
            arch_url(vec![
                ArchUrl {
                    arch: Some(String::from("x86_64")),
                    os: None,
                    url: format!(
                        "https://api.adoptium.net/v3/binary/latest/{}/ga/{}/{}/jre/hotspot/normal/eclipse",
                        version,
                        std::env::consts::OS,
                        "x64"
                    ),
                },
                ArchUrl {
                    arch: None,
                    os: None,
                    url: format!(
                        "https://api.adoptium.net/v3/binary/latest/{}/ga/{}/{}/jre/hotspot/normal/eclipse",
                        version,
                        std::env::consts::OS,
                        std::env::consts::ARCH
                    ),
                },
            ]).unwrap_or(String::from(""))
        }
        JRE::Zulu => todo!(),
        JRE::GraalVM => {
            let _ = arch_support(vec!["x86_64", "x86", "aarch64"]);

            arch_url(vec![
                ArchUrl {
                    arch: Some(String::from("x86")),
                    os: Some(String::from("widnows")),
                    url: format!(
                        "https://download.oracle.com/graalvm/{}/latest/graalvm-jdk-{}_windows-x64_bin.zip",
                        version,
                        version,
                    ),
                },
                ArchUrl {
                    arch: None,
                    os: None,
                    url: format!(
                        "https://download.oracle.com/graalvm/{}/latest/graalvm-jdk-{}_{}-{}_bin.tar.gz",
                        version,
                        version,
                        env::consts::OS,
                        env::consts::ARCH,
                    ),
                }
            ]).unwrap_or(String::from(""))
        }
    }
}

pub fn get_java(
    dir: &PathBuf,
    version: &str,
    jre: JRE,
    user_agent: &str,
) -> impl AsyncSendSync<Result<(), LaunchErrors>> {
    dbg!(&jre, &user_agent, &version, &dir);
    let url = java_url(jre, version);

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
    os: Option<String>,
    url: String,
}

fn arch_url(arch: Vec<ArchUrl>) -> Option<String> {
    fn arch_url(arch: Vec<ArchUrl>, os: Option<String>) -> Option<String> {
        if let None = os {
            for arch in &arch {
                // Deref to get a reference to ArchUrl
                match &arch.arch {
                    Some(archsep) => {
                        if archsep == env::consts::ARCH {
                            return Some(arch.url.clone()); // Clone the url string to return
                        }
                    }
                    None => {
                        return Some(arch.url.clone());
                    }
                }
            }
        } else {
            while let Some(os) = os.as_deref() {
                if os == env::consts::OS {
                    for arch in &arch {
                        match &arch.arch {
                            Some(archsep) => {
                                if archsep == env::consts::ARCH {
                                    return Some(arch.url.clone()); // Clone the url string to return
                                }
                            }
                            None => {
                                return Some(arch.url.clone());
                            }
                        }
                    }
                }
            }
        }
        None // Return None if no matching url is found
    }
    None // Return None if no matching url is found
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
