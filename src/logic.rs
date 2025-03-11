use rand::seq::SliceRandom;
use rand::thread_rng;

use std::cmp::Ordering;
use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;

use crate::info_parse::{self, CountryStat, CurrencyType, Score};

slint::include_modules!();


pub struct StatDisplay {
    pub name: String,
    pub capital: String,
    pub other_info: String,
    pub svg_path: PathBuf,
    pub num: usize,
    pub out_of: usize,
    pub score: u32,
}
pub struct AppLogic {
    current: usize,
    results: Vec<u32>,
    scores: HashMap<String, Score>,
    all_countries: Vec<CountryStat>,
}
impl AppLogic {
    pub fn new(easy_first: bool) -> Self {
        // let mut file = File::open("data/countries.json").expect("File data/countries.json not found");
        // let mut file_content = String::new();
        // file.read_to_string(&mut file_content).expect("File read failed");
        let all_countries_no_filter = info_parse::get_data();
        let mut all_countries: Vec<CountryStat> = all_countries_no_filter
            .into_iter()
            .filter(|country| country.independent.unwrap_or(false))
            .collect();

        let mut rng = thread_rng();
        all_countries.shuffle(&mut rng);
        let scores = info_parse::read(&all_countries);
        let compare = |a: &CountryStat, b: &CountryStat| -> Ordering {
            scores
                .get(&b.cca3.clone())
                .unwrap()
                .score
                .cmp(&scores.get(&a.cca3.clone()).unwrap().score)
        };
        if easy_first {
            all_countries.sort_by(|a, b| compare(a, b));
        } else {
            all_countries.sort_by(|a, b| compare(b, a));
        }
        let len = all_countries.len();
        Self {
            all_countries,
            current: 0,
            results: vec![0; len],
            scores,
        }
    }

    pub fn next(&mut self, result: u32) -> Option<StatDisplay> {
        if result != 0 {
            let score = self
                .scores
                .get_mut(&self.all_countries[self.current].cca3.clone())
                .unwrap();
            if self.results[self.current] == 0 {
                score.time_played += 1;
            }
            score.score = (score.score + result) - self.results[self.current];
            self.results[self.current] = result;
        }
        if self.current < self.all_countries.len() {
            self.current += 1;
            return Some(self.get_stat());
        }
        None
    }

    pub fn prev(&mut self) -> Option<StatDisplay> {
        if self.current > 0 {
            self.current -= 1;
            return Some(self.get_stat());
        }
        None
    }

    pub fn get_stat(&self) -> StatDisplay {
        let country = self.all_countries[self.current].clone();
        let svg_path =
            PathBuf::from_str(format!("data/flags/{}.svg", country.cca3.to_lowercase()).as_str())
                .unwrap();
        let mut v = Vec::new();
        if let Some(c) = country.currencies {
            if let CurrencyType::PRESENT(currencies) = c {
                for (code, currency) in currencies {
                    v.push(format!("{} ({}, {})", currency.name, code, currency.symbol));
                    // Euro (EUR, €)
                }
            }
        }

        StatDisplay {
            name: country.name.common,
            capital: country.capital.join(", "),
            other_info: v.join("\n"),
            svg_path,
            num: self.current,
            out_of: self.all_countries.len(),
            score: self.results[self.current],
        }
    }

    pub fn save_scores(&self) {
        info_parse::save(&self.scores);
    }
}
