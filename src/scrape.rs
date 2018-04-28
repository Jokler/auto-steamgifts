use std::io::Read;

use reqwest;
use select::document::Document;
use select::predicate::Class;

use Result;

#[derive(Debug)]
pub struct Giveaway {
    pub id: String,
    pub name: String,
    pub url_name: String,
}

#[derive(Debug)]
pub struct Scraper {
    giveaways: Vec<Giveaway>,
    current_page: u32,
    is_last_page: bool,
    is_done: bool,
}

// TODO Not the most idiomatic iterator...
impl Scraper {
    pub fn new() -> Self {
        Scraper {
            giveaways: Vec::new(),
            current_page: 0,
            is_last_page: false,
            is_done: false,
        }
    }

    fn next_page(&mut self) -> Result<()> {
        if self.is_last_page {
            self.giveaways.clear();
            self.is_done = true;
            return Ok(());
        }

        self.current_page += 1;
        println!("Downloading page {}", self.current_page);

        let mut res = reqwest::get(&format!(
            "https://www.steamgifts.com/giveaways/search?page={}",
            self.current_page
        ))?;
        let mut content = String::new();
        res.read_to_string(&mut content)?;

        let document = Document::from(content.as_ref());
        if document.find(Class("fa-angle-right")).next() == None {
            self.is_last_page = true;
        }

        self.giveaways.clear();
        for tag in document.find(Class("giveaway__heading__name")) {
            let mut split = tag.attr("href").ok_or(format_err!("URL not found"))?.split("/");

            self.giveaways.push(Giveaway {
                id: split.nth(2).ok_or(format_err!("Unexpected URL found"))?.to_string(),
                name: tag.text(),
                url_name: split.next().ok_or(format_err!("Unexpected URL found"))?.to_string(),
            });
        }

        Ok(())
    }
}

impl Iterator for Scraper {
    type Item = Giveaway;

    fn next(&mut self) -> Option<Self::Item> {
        if self.giveaways.is_empty() {
            self.next_page().expect("Failed to download next page");
        }

        if self.is_done {
            None
        } else {
            self.giveaways.pop()
        }
    }
}
