#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate structopt;

extern crate open;
extern crate reqwest;
extern crate select;
extern crate toml;

use std::path::PathBuf;
use std::fs::OpenOptions;
use std::io::Read;
use structopt::StructOpt;

mod config;
mod add;
mod enter;
mod scrape;

use config::{set_config, Config};
use add::add_games;
use enter::enter_giveaways;

#[derive(StructOpt, Debug)]
#[structopt(name = "Auto Steamgifts")]
/// Makes using Steamgifts more efficient
pub struct CliArgs {
    #[structopt(name = "PATH", short = "c", long = "config", default_value = "./config.toml",
                parse(from_os_str))]
    /// Use a custom config file path
    config: PathBuf,
    #[structopt(subcommand)]
    sub: SubCli,
}

#[derive(StructOpt, Debug)]
pub enum SubCli {
    #[structopt(name = "config",
                raw(setting = "structopt::clap::AppSettings::ArgRequiredElseHelp"))]
    /// Edits the config
    Config {
        #[structopt(flatten)]
        args: ConfigArgs,
    },
    #[structopt(name = "add")]
    /// Adds games to the config interactively
    Add,
    #[structopt(name = "enter")]
    /// Provides fast ways to enters giveaways
    Enter {
        #[structopt(subcommand)]
        args: Option<EnterKind>,
    },
}

#[derive(StructOpt, Debug)]
pub enum EnterKind {
    #[structopt(name = "auto")]
    /// Enter automatically - this is against the guidelines
    Auto {
        /// Ignore the guidelines
        #[structopt(long = "force")]
        force: bool,
    },
    #[structopt(name = "open")]
    /// This opens all whitelisted giveaways in your browser
    Open,
}

#[derive(StructOpt, Debug)]
pub struct ConfigArgs {
    #[structopt(short = "l", long = "list")]
    /// Lists all settings
    list: bool,

    #[structopt(name = "PHPSESSID", short = "s", long = "sessionid")]
    /// Set PHPSESSID from your cookies
    session_id: Option<String>,

    #[structopt(short = "t", long = "token")]
    /// Set xsrf_token from the webpage
    xsrf_token: Option<String>,

    #[structopt(short = "r", long = "request-sleep")]
    /// Set the sleep time between requests in milliseconds
    request_sleep: Option<u64>,
}

fn main() {
    if let Err(e) = run() {
        let text = e.causes()
            .skip(1)
            .fold(format!("{}", e), |acc, err| format!("{}: {}", acc, err));
        eprintln!("{}", text);
    }
}

type Result<T> = std::result::Result<T, failure::Error>;

fn run() -> Result<()> {
    let opt = CliArgs::from_args();

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(opt.config)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut config: Config = toml::from_str(&contents)?;

    match opt.sub {
        SubCli::Config { args } => set_config(args, &mut config, &mut file)?,
        SubCli::Add => add_games(&mut config, &mut file)?,
        SubCli::Enter { args } => enter_giveaways(args.unwrap_or(EnterKind::Open), &config)?,
    };

    Ok(())
}
