use rand::seq::SliceRandom;
use rand::thread_rng;
use slint::Image;

use std::cmp::Ordering;
use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;

use crate::info_parse::{self, CountryStat, Score};

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
    categories: [CatInfo; 3],
    learn_mode: bool,
}
impl AppLogic {
    pub fn new(easy_first: bool, learn_mode: bool, initials_on: bool) -> Self {
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

        let mut categories = [CatInfo::default(), CatInfo::default(), CatInfo::default()];
        categories[0].category = "Country".into();
        categories[1].category = "Capital".into();
        categories[2].category = "Languages".into();
        categories[0].show_first = initials_on;
        categories[1].show_first = initials_on;
        categories[2].show_first = initials_on;

        Self {
            all_countries,
            current: 0,
            results: vec![0; len],
            scores,
            categories,
            learn_mode,
        }
    }

    pub fn next(&mut self, result: u32) -> Option<(FullUpdate, [CatInfo; 3])> {
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

    pub fn prev(&mut self) -> Option<(FullUpdate, [CatInfo; 3])> {
        if self.current > 0 {
            self.current -= 1;
            return Some(self.get_stat());
        }
        None
    }

    pub fn get_stat(&mut self) -> (FullUpdate, [CatInfo; 3]) {
        let country = self.all_countries[self.current].clone();

        let svg_path =
            PathBuf::from_str(format!("data/flags/{}.svg", country.cca3.to_lowercase()).as_str())
                .unwrap();
        let score = self.results[self.current] as i32;
        let update = FullUpdate {
            flag: Image::load_from_path(&svg_path).unwrap(),
            num: self.current as i32,
            out_of: self.all_countries.len() as i32,
            score,
            info_level: if score != 0 || self.learn_mode { 3 } else { 0 },
        };
        let country_name = country.name.common;
        self.categories[0].full = country_name.clone().into();
        self.categories[0].first = country_name.chars().nth(0).unwrap_or(' ').into();
        let capital = country.capital.join(", ");
        self.categories[1].full = capital.clone().into();
        self.categories[1].first = capital.chars().nth(0).unwrap_or(' ').into();
        // let languages_vec: Vec<String> = country.languages.values().map(|v| v.to_string()).collect();
        // let languages = languages_vec.join(", ");
        // self.categories[2].full = languages.into();
        self.categories[2].full = format!("{} ({})", country.subregion, country.region).into();

        (update, self.categories.clone())
    }

    pub fn save_scores(&self) {
        info_parse::save(&self.scores);
    }
}
