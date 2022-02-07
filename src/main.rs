use clap::Parser;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Parser, Debug)]
#[clap(
    name = "RPN program",
    version = "1.0.0",
    author = "rokah",
    about = "RPN calculator."
)]
struct Opts {
    /// Sets the level of verbosity
    #[clap(short, long)]
    verbose: bool,

    #[clap(short, long)]
    scrape: bool,

    /// Formulas written in RPN
    #[clap(name = "FILE")]
    formula_file: Option<String>,
}

struct Scraper;

impl Scraper {
    pub fn scrape(url: &str, selector: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let body = reqwest::blocking::get(url)?.text()?;
        let document = scraper::Html::parse_document(&body);

        // maybe panic
        let selector = scraper::Selector::parse(selector).unwrap();

        let elements = document.select(&selector);

        let mut ret: Vec<String> = Vec::new();
        elements.for_each(|e| {
            let name = e.text().next().unwrap().to_string();
            if name.chars().count() == 5 {
                ret.push(name)
            }
        });
        Ok(ret)
    }
}

pub fn write<P: AsRef<Path>>(path: P, v: Vec<String>) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    let content = v.join("\n");
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn main() {
    let opts = Opts::parse();

    if opts.scrape {
        let result = Scraper::scrape("https://wiki.xn--rckteqa2e.com/wiki/%E3%83%9D%E3%82%B1%E3%83%A2%E3%83%B3%E4%B8%80%E8%A6%A7","table.bluetable tbody tr td:nth-child(2) > a");
        println!("{:?}", result);
        let result = write("./pokemon_names.txt", result.unwrap());
        println!("{:?}", result);
    }
}
