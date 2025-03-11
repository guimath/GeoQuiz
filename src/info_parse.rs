use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::str::FromStr;

#[derive(Deserialize, Debug, Clone)]
pub struct CountryName {
    pub common: String,
    // official: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Currency {
    pub name: String,
    pub symbol: String,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum CurrencyType {
    PRESENT(HashMap<String, Currency>),
    #[allow(dead_code)]
    EMPTY(Vec<String>),
}
#[derive(Deserialize, Debug, Clone)]
pub struct CountryStat {
    pub name: CountryName,
    pub cca3: String,
    pub independent: Option<bool>,
    pub capital: Vec<String>,
    pub currencies: Option<CurrencyType>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Score {
    pub time_played: u32,
    pub score: u32,
}

const JSON_DATA: &str = include_str!("../data/countries.json"); // Embed the JSON file
pub fn get_data() -> Vec<CountryStat> {
    serde_json::from_str(JSON_DATA).unwrap()
}

pub fn read(all_countries: &Vec<CountryStat>) -> HashMap<String, Score> {
    let score_path = PathBuf::from_str("score.json").unwrap();
    if Path::exists(&score_path) {
        let mut file = File::open(score_path).unwrap();
        let mut file_content = String::new();
        file.read_to_string(&mut file_content)
            .expect("File read failed");
        serde_json::from_str(&file_content).unwrap()
    } else {
        all_countries
            .iter()
            .map(|country| {
                (
                    country.cca3.clone(),
                    Score {
                        score: 0,
                        time_played: 0,
                    },
                )
            })
            .collect()
    }
}
pub fn save(scores: &HashMap<String, Score>) {
    let json_data = serde_json::to_string(scores).unwrap();
    let mut file = File::create("score.json").unwrap();
    file.write_all(json_data.as_bytes()).unwrap();
}
