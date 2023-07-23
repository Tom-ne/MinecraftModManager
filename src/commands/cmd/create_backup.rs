use std::{
    fs::{self, File},
    io::{Read, Write},
    os::unix::prelude::PermissionsExt,
    path::Path,
};

use async_trait::async_trait;
use chrono::{DateTime, Datelike, Local, Timelike};
use zip::{write::FileOptions, CompressionMethod, ZipWriter};

use crate::lib::modify::{command::Command, config_helper::read_config};

pub struct CreateBackupCommand;

fn zip_folder(input_folder: &Path, output_zip: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let zip_file = File::create(output_zip)?;
    let mut zip = ZipWriter::new(zip_file);

    let options = FileOptions::default()
        .compression_method(CompressionMethod::Stored)
        .unix_permissions(fs::metadata(input_folder)?.permissions().mode());

    zip_folder_recursive(&input_folder, &mut zip, &input_folder, options)?;

    Ok(())
}

fn zip_folder_recursive(
    input_folder: &Path,
    zip: &mut ZipWriter<File>,
    base_folder: &Path,
    options: FileOptions,
) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(input_folder)? {
        let entry = entry?;
        let path = entry.path();
        let name = path
            .strip_prefix(base_folder)?
            .to_string_lossy()
            .into_owned();

        if path.is_dir() {
            zip.add_directory(name, options)?;
            zip_folder_recursive(&path, zip, base_folder, options)?;
        } else {
            zip.start_file(name, options)?;
            let mut file = File::open(&path)?;
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;
            zip.write_all(&buffer)?;
        }
    }

    Ok(())
}

#[async_trait]
impl Command for CreateBackupCommand {
    async fn run(&self) {
        let settings = read_config().unwrap();
        let mc_folder: &Path;

        let path = Path::new(&settings.mc_mod_dir);
        if let Some(parent) = path.parent() {
            mc_folder = parent;
        } else {
            println!("Error: unable to get minecraft folder!");
            return;
        }

        let local_time: DateTime<Local> = Local::now();
        let formatted_time = format!(
            "{:02}-{:02}-{:02}-{:02}-{}",
            local_time.hour(),
            local_time.minute(),
            local_time.day(),
            local_time.month(),
            local_time.year()
        );

        let backup_folder_str = format!("{}/{}", mc_folder.display(), "mod-backups");
        let backup_folder = Path::new(&backup_folder_str);

        if let Err(e) = fs::create_dir_all(backup_folder) {
            println!("Error creating backup folder: {:?}", e);
            return;
        }

        let zip_file_path = format!("{}.zip", formatted_time);
        let output_zip = backup_folder.join(zip_file_path);

        println!("Creating backup...");
        if let Err(e) = zip_folder(Path::new(&settings.mc_mod_dir), &output_zip) {
            println!("Failed to create backup: {:?}", e);
        } else {
            println!("Backup created successfully");
        }
    }

    fn description(&self) -> &str {
        "create a backup of the mods folder"
    }
}
