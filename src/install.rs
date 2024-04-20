use crate::config::{get_config, Config};
use std::io;

pub fn get_config_or_install() -> Result<Config, io::Error> {
    let cfg_res = get_config();
    match cfg_res {
        Ok(config) => return Ok(config),
        Err(e) => return Err(e),
    }
}
