use crate::config::JournalConfig;
use chrono::{DateTime, Local, Utc};
use std::fmt;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug)]
pub struct Tag {
    pub name: String,
    pub count: u64,
}

#[derive(Debug)]
pub struct Entry {
    pub date: chrono::DateTime<Utc>,
    pub title: String,
    pub body: String,
    pub tags: Vec<Tag>,
}

#[derive(Debug)]
pub struct Journal {
    pub config: JournalConfig,
    pub name: String,
    pub entries: Vec<Entry>,
}

impl fmt::Display for Entry {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let fdate: String = "2024-05-01 11:53:00".into();
        let titles: String = "This is a title".into();
        let bodys: String = "Body".into();
        let title = format!("\n\n[{fdate}] {titles}");
        let body = format!("\n{bodys}");
        let _ = fmt.write_str(title.as_str());
        let _ = fmt.write_str(body.as_str());
        Ok(())
    }
}

pub fn write_journal(config: &JournalConfig, entry: &Entry) {
    let mut file = OpenOptions::new()
        .create_new(!Path::new(config.path.as_str()).exists())
        .write(true)
        .append(true)
        .open(config.path.as_str())
        .unwrap();
    file.write(entry.to_string().as_bytes())
        .expect("Failed writing the file");
}
