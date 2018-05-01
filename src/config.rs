extern crate serde;

use toml;
use std::fs::File;
use std::io::{Seek, Write};
use std::io::SeekFrom::Start;
use ConfigArgs;

use Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub session_id: String,
    #[serde(default)]
    pub xsrf_token: String,
    #[serde(default = "default_sleep")]
    pub request_sleep: u64,
    #[serde(default)]
    pub whitelist: Vec<String>,
    #[serde(default)]
    pub blacklist: Vec<String>,
}

fn default_sleep() -> u64 {
    500
}

pub fn set_config(args: ConfigArgs, config: &mut Config, file: &mut File) -> Result<()> {
    if let Some(session_id) = args.session_id {
        config.session_id = session_id.to_string();
    }

    if let Some(xsrf_token) = args.xsrf_token {
        config.xsrf_token = xsrf_token;
    }

    if let Some(request_sleep) = args.request_sleep {
        config.request_sleep = request_sleep;
    }

    if args.list {
        println!("session_id = \"{}\"", config.session_id);
        println!("xsrf_token = \"{}\"", config.xsrf_token);
        println!("request_sleep = {}", config.request_sleep);
    }

    config.write_to_file(file)
}

impl Config {
    pub fn write_to_file(&self, file: &mut File) -> Result<()> {
        let toml = toml::to_string_pretty(self)?;

        file.seek(Start(0))?;
        file.set_len(0)?;
        file.write(toml.as_bytes())?;

        Ok(())
    }
}
