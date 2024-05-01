use crate::config::{Config, get_cur_journal_config, JournalConfig};

#[derive(Debug)]
pub enum ModeKind {
    Append
}

#[derive(Debug)]
pub struct Mode {
    kind: ModeKind,
    journal: JournalConfig
}

pub fn get_mode(args: &Vec<String>, config: &mut Config) -> Mode {
    let mode = Mode {
        kind: ModeKind::Append,
        journal: get_cur_journal_config(&args, config)
    };
    return mode;
}
