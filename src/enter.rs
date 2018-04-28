use config::Config;
use EnterKind;
use scrape::{Giveaway, Scraper};

use std::collections::HashMap;
use reqwest::{Client, StatusCode, header::{Cookie, Headers}};
use open;

use Result;

pub fn enter_giveaways(kind: EnterKind, config: &Config) -> Result<()> {
    let scraper = Scraper::new();

    match kind {
        EnterKind::Auto { force } => {
            if !force {
                bail!("This breaks steamgifts guidelines. Add --force if you are sure.");
            }
            if config.session_id.is_empty() {
                bail!("No cookie set");
            }
            if config.xsrf_token.is_empty() {
                bail!("No token set");
            }
            let client = Client::new();

            let mut headers = Headers::new();
            let mut cookie = Cookie::new();
            cookie.append("PHPSESSID", config.session_id.clone());
            headers.set(cookie);

            for Giveaway { id, url_name, name } in scraper {
                if !config.whitelist.contains(&url_name) {
                    continue;
                }

                let mut params = HashMap::new();
                params.insert("xsrf_token", config.xsrf_token.clone());
                params.insert("do", String::from("entry_insert"));
                params.insert("code", id);

                let response = client
                    .post("https://www.steamgifts.com/ajax.php")
                    .form(&params)
                    .headers(headers.clone())
                    .send()?;

                if response.status() == StatusCode::Ok {
                    println!("Entered giveaway for {}", name);
                }
            }
        }
        EnterKind::Open => for Giveaway { id, url_name, .. } in scraper {
            if !config.whitelist.contains(&url_name) {
                continue;
            }
            open::that(format!(
                "https://www.steamgifts.com/giveaway/{}/{}",
                id, url_name
            ))?;
        },
    }
    
    Ok(())
}
