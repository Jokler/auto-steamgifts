use std::fs::File;
use std::io::{stdin, stdout};
use std::io::Write;
use scrape::{Giveaway, Scraper};
use config::Config;

use Result;

pub fn add_games(config: &mut Config, file: &mut File) -> Result<()> {
    let scraper = Scraper::new();

    println!("Answer with 'n' to add to the blacklist. (empty to skip)");

    for Giveaway { name, url_name, .. } in scraper {
        if config.whitelist.contains(&url_name) || config.blacklist.contains(&url_name) {
            continue;
        }

        print!("Whitelist '{}'?<y/n>", name);
        stdout().flush()?;

        let mut input = String::new();
        stdin().read_line(&mut input)?;

        if input.starts_with('y') || input.starts_with('Y') {
            config.whitelist.push(url_name);
            config.write_to_file(file)?;
        } else if input.starts_with('n') || input.starts_with('N') {
            config.blacklist.push(url_name);
            config.write_to_file(file)?;
        }
    }

    Ok(())
}
