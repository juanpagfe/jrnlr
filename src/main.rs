mod install;
mod config;
use clap::Parser;
use std::env;
use crate::install::{is_installed, install};

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
    let argl: Vec<String> = env::args().collect();
    let mut iter = argl.iter().skip(1);
    while let Some(item) = iter.next() {
        println!("{}", item);
    }
    if !is_installed() {
        let result = install();
        match result {
            Ok(val) => println!("Hello World {}", val.to_string()),
            Err(e) => println!("Error: {}", e.to_string())
        }
    }
    //println!("Hello {}!", args.name.unwrap_or("Option is none".to_string()))
}
