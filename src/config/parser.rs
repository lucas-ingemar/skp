//! An example showing off the usage of `Deserialize` to automatically decode
//! TOML into a Rust `struct`

#![deny(warnings)]
#![allow(dead_code)]

use anyhow::Error;
use serde::Deserialize;
use std::{fs, io::ErrorKind};

/// This is what we're going to decode into. Each field is optional, meaning
/// that it doesn't have to be present in TOML.
#[derive(Debug, Deserialize)]
pub struct Config {
    pub global_string: Option<String>,
    pub global_integer: Option<u64>,
    pub project: Option<Vec<Project>>,
    #[serde(rename = "cmd.suffix")]
    pub cmd_suffix: Option<Vec<Cmd>>,
    #[serde(rename = "cmd.prefix")]
    pub cmd_prefix: Option<Vec<Cmd>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Project {
    pub name: String,
    pub path: String,
}

#[derive(Debug, Deserialize)]
pub struct Cmd {
    pub cmd: String,
}

pub fn parse(env_cfg: super::env::Config) -> Result<Config, Error> {
    let config_path = &env_cfg.config_file;

    match fs::read_to_string(config_path) {
        Ok(s) => {
            // println!("{}", s);
            let cfg: Config = toml::from_str(&s)?;
            Ok(cfg)
        }
        Err(e) if e.kind() == ErrorKind::NotFound => {
            let cfg = new();
            // TODO: Write to file aswell
            Ok(cfg)
        }
        Err(e) => Err(e.into()),
    }
}

pub fn new() -> Config {
    Config {
        global_string: None,
        global_integer: None,
        project: None,
        cmd_suffix: None,
        cmd_prefix: None,
    }
}
