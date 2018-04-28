extern crate serde;

use toml;
use std::fs::File;
use std::io::{Seek, Write};
use std::io::SeekFrom::Start;
use ConfigArgs;

use Result;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Config {
    pub session_id: String,
    pub xsrf_token: String,
    pub whitelist: Vec<String>,
    pub blacklist: Vec<String>,
}

pub fn set_config(args: ConfigArgs, config: &mut Config, file: &mut File) -> Result<()> {
    if let Some(session_id) = args.session_id {
        config.session_id = session_id.to_string();
    }

    if let Some(xsrf_token) = args.xsrf_token {
        config.xsrf_token = xsrf_token;
    }

    if args.list {
        println!("session_id = \"{}\"", config.session_id);
        println!("xsrf_token = \"{}\"", config.xsrf_token);
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
