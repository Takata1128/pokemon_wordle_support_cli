use clap::Parser;
use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

#[derive(Parser, Debug)]
#[clap(
    name = "pokemon_wordle support program",
    version = "1.0.0",
    author = "rokah",
    about = "Support to solve pokemon wordle."
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

    let filename: &str = "./pokemon_names.txt";

    if opts.scrape {
        let result = Scraper::scrape("https://wiki.xn--rckteqa2e.com/wiki/%E3%83%9D%E3%82%B1%E3%83%A2%E3%83%B3%E4%B8%80%E8%A6%A7","table.bluetable tbody tr td:nth-child(2) > a");
        println!("{:?}", result);
        let result = write(filename, result.unwrap());
        println!("{:?}", result);
    }

    {
        let mut f = File::open(filename).expect("file not found.");

        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("something went wrong reading the file.");

        let v: Vec<&str> = (&contents).split('\n').collect();

        println!("{:?}", v);

        let mut chars: HashSet<char> = HashSet::new();

        for pokemon in v.into_iter() {
            for c in pokemon.chars() {
                chars.insert(c);
            }
        }
        let mut coodinates = vec![chars; 5];

        loop {
            // ポケモン名入力

            let mut pokemon_name = String::new();
            io::stdin()
                .read_line(&mut pokemon_name)
                .expect("Failed to read line.");

            // チェック文字列入力
            // 正解なら終了

            // . miss
            // # near
            // * hit

            let mut check = String::new();
            io::stdin()
                .read_line(&mut check)
                .expect("Failed to read line.");

            for (a, (b, c)) in check.chars().zip(pokemon_name.chars()).enumerate() {
                if b == '.' {
                    // すべての箇所の候補から該当文字を除外
                } else if b == '#' {
                    // 該当箇所の候補から該当文字を除外
                } else if b == '*' {
                    // 該当箇所の候補を決定
                }
                println!("{} {}", a, b);
            }

            // 候補の条件を満たすポケモンを表示
        }
    }
}
