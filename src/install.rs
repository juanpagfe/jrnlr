use std::io;
use std::fs::{create_dir_all, File};
use crate::config::{get_config_dir, CONFIG_FILENAME};

pub fn is_installed() -> bool {
    if let Some(config_dir) = get_config_dir() {
        return config_dir.exists() && config_dir.join(CONFIG_FILENAME).exists();
    }
    return false;
}

pub fn install() -> Result<bool, io::Error> {
    if let Some(config_dir) = get_config_dir() {
        if !config_dir.exists() {
            create_dir_all(config_dir.clone())?;
        }
        let file_path = config_dir.join(CONFIG_FILENAME);
        let _ = File::create(&file_path);
    }
    Ok(true)
}
