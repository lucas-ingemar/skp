use std::env;
use std::fmt;
use xdg;

const APP_NAME: &str = "skp";

pub struct Config {
    config_file: String,
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Define how to format the output
        match &self.config_file {
            file => write!(f, "Config file: {}", file),
        }
    }
}

pub fn load_envs() -> Config {
    let xdg_dirs = xdg::BaseDirectories::with_prefix(APP_NAME);

    match xdg_dirs.config_home {
        Some(d) => println!("{}/{}/config.toml", d.to_str().unwrap(), APP_NAME),
        None => println!("fel fel"),
    }

    let mut c = Config {
        config_file: "df".to_string(),
    };

    match env::var(APP_NAME) {
        Ok(t) => c.config_file = t,
        Err(_) => (),
    }
    c
}
