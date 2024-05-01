mod config;
mod install;
mod journal;
mod mode;
use crate::install::get_config_or_install;
use crate::mode::get_mode;
use crate::journal::{Entry, Journal, write_journal};
use chrono::{Local};
use clap::Parser;
use std::env;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    #[arg(short, long, required = false)]
    name: Option<String>,

    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    //let args = Args::parse();
    let arg_list: Vec<String> = env::args().collect();
    let mut config = get_config_or_install().unwrap();
    let mode = get_mode(&arg_list, &mut config);
    let entry = Entry {
        date: Local::now().to_utc(),
        title: "Title".into(),
        body: "Body".into(),
        tags: Vec::new(),
    };
    write_journal(&mode.journal_config, &entry);
    println!("{:?}", mode);
}
