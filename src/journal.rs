use crate::config::JournalConfig;
use chrono::{DateTime, Local, Utc};

#[derive(Debug)]
struct Tag {
    name: String,
    count: u64,
}

#[derive(Debug)]
struct Entry {
    date: chrono::DateTime<Utc>,
    title: String,
    body: String,
    tags: Vec<Tag>
}

#[derive(Debug)]
struct Journal {
    config: JournalConfig,
    name: String,
    entries: Vec<Entry>
}
