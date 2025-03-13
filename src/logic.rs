use rand::seq::SliceRandom;
use rand::thread_rng;
use slint::Image;

use std::cmp::Ordering;
use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;

use crate::info_parse::{self, CountryStat, CurrencyType, Score};

slint::include_modules!();

pub enum InfoType {
    COUNTRY,
    CAPITAL,
    LANGUAGES,
    CURRENCIES,
    LATLON,
    BORDERS,
    REGION,
}

macro_rules! map_all_countries {
    ($all_countries:ident, $country:ident, $code:block) => {
        $all_countries.iter().map(|$country| $code).collect()
    };
}
fn hint_from_name(s: String) -> String {
    let mut s = s.chars().nth(0).unwrap_or(' ').to_string();
    s.push_str("...");
    s
}
impl InfoType {
    pub fn to_str(&self) -> &str {
        match self {
            InfoType::COUNTRY => "Country:",
            InfoType::CAPITAL => "Capital:",
            InfoType::LANGUAGES => "Languages:",
            InfoType::CURRENCIES => "Currencies:",
            InfoType::LATLON => "LatLon:",
            InfoType::BORDERS => "Borders:",
            InfoType::REGION => "Region:",
        }
    }
}

impl FromStr for InfoType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "Country" => Ok(InfoType::COUNTRY),
            "Capital" => Ok(InfoType::CAPITAL),
            "Languages" => Ok(InfoType::LANGUAGES),
            "Currencies" => Ok(InfoType::CURRENCIES),
            "Latlon" => Ok(InfoType::LATLON),
            "Borders" => Ok(InfoType::BORDERS),
            "Region" => Ok(InfoType::REGION),
            _ => Err(()), // Return an error if the string doesn't match
        }
    }
}

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
    infos: [Vec<CatInfo>; 3],
    cca3_to_name: HashMap<String, String>,
}
impl AppLogic {
    pub fn new(easy_first: bool) -> Self {
        // let mut file = File::open("data/countries.json").expect("File data/countries.json not found");
        // let mut file_content = String::new();
        // file.read_to_string(&mut file_content).expect("File read failed");
        let all_countries_no_filter = info_parse::get_data();
        let mut cca3_to_name = HashMap::new();
        for country in all_countries_no_filter.clone() {
            cca3_to_name.insert(country.cca3, country.name.common);
        }
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
            infos: Default::default(),
            cca3_to_name,
        }
    }

    pub fn prep_categories(&mut self, info_types: [InfoType; 3]) {
        let all_countries = self.all_countries.clone();
        for i in 0..3 {
            let cat_str = info_types[i].to_str();
            self.infos[i] = match info_types[i] {
                InfoType::COUNTRY => map_all_countries!(all_countries, x, {
                    let data = x.name.common.clone();
                    CatInfo {
                        category: cat_str.into(),
                        full: data.clone().into(),
                        first: hint_from_name(data).into(),
                        with_hint: true,
                    }
                }),

                InfoType::CAPITAL => map_all_countries!(all_countries, x, {
                    let first: Vec<String> = x
                        .capital
                        .iter()
                        .map(|s| hint_from_name(s.clone()))
                        .collect();
                    CatInfo {
                        category: cat_str.into(),
                        full: x.capital.join(", ").into(),
                        first: first.join(", ").into(),
                        with_hint: true,
                    }
                }),

                InfoType::LANGUAGES => map_all_countries!(all_countries, x, {
                    let lang_vec: Vec<String> =
                        x.languages.values().map(|v| v.to_string()).collect();
                    let first: Vec<String> =
                        lang_vec.iter().map(|s| hint_from_name(s.clone())).collect();
                    CatInfo {
                        category: cat_str.into(),
                        full: lang_vec.join(", ").into(),
                        first: first.join(", ").into(),
                        with_hint: true,
                    }
                }),

                InfoType::CURRENCIES => map_all_countries!(all_countries, x, {
                    let mut v = Vec::new();
                    if let Some(c) = x.currencies.clone() {
                        if let CurrencyType::PRESENT(currencies) = c {
                            for (code, currency) in currencies {
                                v.push(format!(
                                    "{} ({}, {})",
                                    currency.name, code, currency.symbol
                                ));
                                // Euro (EUR, €)
                            }
                        }
                    }
                    CatInfo {
                        category: cat_str.into(),
                        full: v.join(", ").into(),
                        first: ' '.into(),
                        with_hint: false,
                    }
                }),

                InfoType::LATLON => {
                    panic!("Latlon info not done yet")
                }

                InfoType::REGION => map_all_countries!(all_countries, x, {
                    let data = format!("{} ({})", x.subregion, x.region);
                    CatInfo {
                        category: cat_str.into(),
                        full: data.clone().into(),
                        first: hint_from_name(data).into(),
                        with_hint: true,
                    }
                }),

                InfoType::BORDERS => map_all_countries!(all_countries, x, {
                    let border_vec: Vec<String> = x
                        .borders
                        .iter()
                        .map(|s| self.cca3_to_name.get_key_value(s).unwrap().1.to_string())
                        .collect();
                    let first: Vec<String> = border_vec
                        .iter()
                        .map(|s| hint_from_name(s.clone()))
                        .collect();
                    CatInfo {
                        category: cat_str.into(),
                        full: border_vec.join(", ").into(),
                        first: first.join(", ").into(),
                        with_hint: true,
                    }
                }),
            }
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
            seen: score != 0,
        };

        (
            update,
            [
                self.infos[0][self.current].clone(),
                self.infos[1][self.current].clone(),
                self.infos[2][self.current].clone(),
            ],
        )
    }

    pub fn save_scores(&self) {
        info_parse::save(&self.scores);
    }
}
