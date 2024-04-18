use std::env;
use std::path::{Path, PathBuf};

pub const CONFIG_FILENAME: &str = "config.toml";

fn get_app_name() -> Option<String> {
    if let Some(app_path) = env::args().next() {
        if let Some(app_name) = Path::new(&app_path).file_name() {
            if let Some(app_name_str) = app_name.to_str() {
                return Some(app_name_str.to_string());
            }
        }
    }
    None
}

pub fn get_config_dir() -> Option<PathBuf> {
    if let Some(config_dir) = dirs::config_dir() {
        if let Some(app_name) = get_app_name() {
            return Some(config_dir.join(app_name));
        }
    }
    None
}
