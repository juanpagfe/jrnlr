use serde::ser::{Serialize, SerializeStruct, Serializer};
use serde::{Deserialize, Deserializer};
use std::collections::{BTreeMap, HashMap};
use std::env;
use std::fs::{create_dir_all, File};
use std::io::prelude::*;
use std::io::Read;
use std::path::{Path, PathBuf};

pub const CONFIG_FILENAME: &str = "config.toml";
const DEFAULT_TAG_SYMBOLS: &str = "@#";

#[derive(Debug, Clone)]
pub struct JournalConfig {
    path: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    linewrap: i32,
    tagsymbols: String,
    date_format: String,
    indent_char: char,
    editor: String,
    journals: BTreeMap<String, JournalConfig>,
}

impl<'de> Deserialize<'de> for JournalConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let map: HashMap<String, serde_json::Value> = Deserialize::deserialize(deserializer)?;

        let path = map
            .get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| serde::de::Error::custom("Missing or invalid 'name' field"))?;

        Ok(JournalConfig {
            path: path.to_owned(),
        })
    }
}

impl Serialize for Config {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Config", 6)?;
        state.serialize_field("linewrap", &self.linewrap)?;
        state.serialize_field("tagsymbols", &self.tagsymbols)?;
        state.serialize_field("date_format", &self.date_format)?;
        state.serialize_field("indent_char", &self.indent_char)?;
        state.serialize_field("editor", &self.editor)?;
        state.serialize_field("journals", &self.journals)?;
        state.end()
    }
}

impl Serialize for JournalConfig {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("JournalConfig", 1)?;
        state.serialize_field("path", &self.path)?;
        state.end()
    }
}

pub fn get_cur_journal_config(args: &Vec<String>, config: &mut Config) -> JournalConfig {
    // For now, it returns the default
    return config.journals.get("default").unwrap().clone();
}

fn get_app_name() -> String {
    let app_path = env::args().next().unwrap();
    let app_name = Path::new(&app_path).file_name();
    return app_name.unwrap().to_str().unwrap().to_string();
}

pub fn get_config_dir() -> PathBuf {
    let config_dir = dirs::config_dir().unwrap();
    return config_dir.join(get_app_name());
}

pub fn get_config() -> std::io::Result<Config> {
    let config_dir = get_config_dir();
    if !config_dir.exists() || !config_dir.join(CONFIG_FILENAME).exists() {
        let config = create_default_config(config_dir);
        match config {
            Ok(cfg) => return Ok(cfg),
            Err(e) => return Err(e),
        }
    } else {
        let mut file =
            File::open(config_dir.join(CONFIG_FILENAME)).expect("Failed to open config file");
        let mut toml_str = String::new();
        file.read_to_string(&mut toml_str)
            .expect("Failed to read config file");
        let config: Config = toml::de::from_str(&toml_str).expect("Failed to deserialize TOML");

        return Ok(config);
    }
}

pub fn create_default_config(config_dir: PathBuf) -> std::io::Result<Config> {
    let mut journals: BTreeMap<String, JournalConfig> = BTreeMap::new();
    if let Some(journal_path) = config_dir.join("default").to_str() {
        journals.insert(
            "default".to_string(),
            JournalConfig {
                path: journal_path.to_string(),
            },
        );
    }
    let default_config = Config {
        linewrap: 80,
        tagsymbols: DEFAULT_TAG_SYMBOLS.to_string(),
        date_format: "".to_string(),
        indent_char: '|',
        editor: "".to_string(),
        journals,
    };

    create_dir_all(config_dir.clone())?;
    let file_path = config_dir.join(CONFIG_FILENAME);
    let mut file = File::create(&file_path)?;
    let tom_str = toml::to_string(&default_config).unwrap();

    file.write_all(tom_str.as_bytes())?;
    Ok(default_config)
}
