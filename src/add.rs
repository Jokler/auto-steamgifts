use std::fs::File;
use std::io::{stdin, stdout};
use std::io::Write;
use scrape::{Giveaway, Scraper};
use config::Config;
use open;

use Result;

pub fn add_games(config: &mut Config, file: &mut File) -> Result<()> {
    let scraper = Scraper::new();

    println!("Answer with 'n' to add to the blacklist, 'g' to google. (empty skips)");

    for Giveaway { name, url_name, .. } in scraper {
        if config.whitelist.contains(&url_name) || config.blacklist.contains(&url_name) {
            continue;
        }

        loop {
            print!("Whitelist '{}'?<y/n/g>", name);
            stdout().flush()?;

            let mut input = String::new();
            stdin().read_line(&mut input)?;

            if input.starts_with('y') || input.starts_with('Y') {
                config.whitelist.push(url_name);
                config.write_to_file(file)?;
            } else if input.starts_with('n') || input.starts_with('N') {
                config.blacklist.push(url_name);
                config.write_to_file(file)?;
            } else if input.starts_with('g') || input.starts_with('G') {
                if let Err(e) = open::that(format!(
                    "https://www.google.com/search?q={} {}",
                    name, "game"
                )) {
                    eprintln!("{}", e);
                }
                continue;
            }
            break;
        }
    }

    Ok(())
}
