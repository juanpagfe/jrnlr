mod config;
mod install;
use crate::install::get_config_or_install;
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
    let argl: Vec<String> = env::args().collect();
    let mut iter = argl.iter().skip(1);
    while let Some(item) = iter.next() {
        println!("{}", item);
    }
    let cfg_res = get_config_or_install();
    match cfg_res {
        Ok(config) => {
            println!("{:?}", config);
        }
        Err(e) => println!("Error: {}", e.to_string()),
    }
}
