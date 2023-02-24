use serde::{Deserialize, Serialize};
use std::{env, fs, path::Path};

use crate::util::current_dir;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub server: ConfigServer,
    pub sites: ConfigSites,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigServer {
    pub host: String,
    pub port: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigSites {
    pub gitee: Vec<ConfigSite>,
    pub github: Vec<ConfigSite>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigSite {
    pub name: String,
    pub password: String,
    #[serde(rename = "ref")]
    pub _ref: String,
    pub hook_name: String,
    pub cmds: Vec<String>,
}

pub fn get_config() -> anyhow::Result<Config> {
    let file_path = Path::new(&current_dir()?).join("config.json");
    let config: Config = json5::from_str(&fs::read_to_string(file_path)?)?;
    Ok(config)
}
