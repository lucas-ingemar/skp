use anyhow::Error;
use std::env;
use std::fmt;
use std::path::PathBuf;
use std::str::FromStr;
use xdg;

const APP_NAME: &str = "skp";

pub struct Config {
    pub config_file: PathBuf,
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Define how to format the output
        match &self.config_file {
            file => write!(f, "Config file: {}", file.display()),
        }
    }
}

fn env_name(name: &str) -> String {
    format!("{APP_NAME}_{name}").to_ascii_uppercase()
}

pub fn load_env_cfg() -> Result<Config, Error> {
    let xdg_dirs = xdg::BaseDirectories::with_prefix(APP_NAME);

    let mut c = Config {
        config_file: xdg_dirs.place_config_file("config.toml")?,
    };

    match env::var(env_name("config_file")) {
        Ok(t) => c.config_file = PathBuf::from_str(t.as_str())?,
        Err(_) => (),
    }

    Ok(c)
}
