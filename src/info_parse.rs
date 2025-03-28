use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Category {
    pub full: String,
    pub hint: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CountryInfos {
    pub cca3: String,
    pub independent: bool,
    pub infos: Vec<Category>,
    pub images: Vec<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Score {
    #[serde(rename = "tp")]
    pub time_played: u32,
    #[serde(rename = "st")]
    pub total_score: u32,
    #[serde(rename = "sl")]
    pub last_score: u32,
}

const JSON_DATA: &str = include_str!("../data/infos.json"); // Embed the JSON file
pub fn get_data() -> Vec<CountryInfos> {
    serde_json::from_str(JSON_DATA).unwrap()
}

pub fn read(all_countries: &Vec<CountryInfos>, score_path: PathBuf) -> HashMap<String, Score> {
    if score_path.exists() {
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
                        total_score: 0,
                        last_score: 0,
                        time_played: 0,
                    },
                )
            })
            .collect()
    }
}
pub fn save(scores: &HashMap<String, Score>, score_path: PathBuf) {
    let json_data = serde_json::to_string(scores).unwrap();
    let mut file = File::create(score_path).unwrap();
    file.write_all(json_data.as_bytes()).unwrap();
}

pub fn reset_score(score_path: PathBuf) {
    if score_path.exists() {
        fs::remove_file(score_path).unwrap();
    }
}
