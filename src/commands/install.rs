use serde_json::Value;
use std::io::{self, Error, ErrorKind};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use crate::lib::config_helper::read_config;
use crate::lib::io_helper::{flush_output_stream, get_user_input};
use crate::lib::modrinth::get_project::get_project;
use crate::lib::modrinth::get_versions::get_mod_versions;

async fn download_mod(json_str: &str, mc_version: &str) -> Result<(), io::Error> {
    let json: Value = serde_json::from_str(json_str)?;

    let binding = json["slug"].as_str().unwrap_or("").trim_matches('"');
    let mod_versions = get_mod_versions(binding).await?;
    if let Some(mod_version) = mod_versions.iter().find(|v| v.minecraft_version == mc_version) {
        let response = reqwest::get(&mod_version.download_url)
            .await
            .map_err(|err| Error::new(ErrorKind::Other, format!("Request failed: {:?}", err)))?;

        let config = read_config("config.json").unwrap();

        let file_name = format!("{}/{}_{}.jar", config.mc_mod_dir, binding, mod_version.minecraft_version);
        let mut file = File::create(file_name).await?;
        let mut content = response
            .bytes()
            .await
            .map_err(|err| Error::new(ErrorKind::Other, format!("Failed to read response: {:?}", err)))?;
        file.write_all(&mut content)
            .await
            .map_err(|err| Error::new(ErrorKind::Other, format!("Failed to write to file: {:?}", err)))?;

        println!("Successfully installed {} for Minecraft version {}", binding, mc_version);
    } else {
        println!("Failed to install {} for Minecraft version {}", binding, mc_version);
    }

    Ok(())
}

pub(crate) async fn run() {
    print!("Enter mod to install: ");
    flush_output_stream();
    let input = get_user_input().to_lowercase();
    print!("Enter mc version: ");
    flush_output_stream();
    let mc_version = get_user_input().to_lowercase();

    match get_project(&input).await {
        Ok(json) => {
            if let Ok(pretty_json) = serde_json::to_string_pretty(&json) {
                if let Err(err) = download_mod(&pretty_json, &mc_version).await {
                    eprintln!("Error: {:?}", err);
                }
            } else {
                println!("Failed to format JSON");
            }
        }
        Err(err) => {
            eprintln!("Error: {:?}", err);
        }
    }
}
