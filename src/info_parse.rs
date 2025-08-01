use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Category {
    pub full: String,
    pub hint: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AllInfos {
    pub all_countries: Vec<CountryInfos>,
    pub info_names: Vec<String>,
    pub image_names: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CountryInfos {
    pub region: String,
    pub un_member: bool,
    pub infos: Vec<Category>,
    pub images: Vec<ImageLink>,
    pub wiki_link: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct Score {
    #[serde(rename = "tp")]
    pub time_played: u32,
    #[serde(rename = "st")]
    pub total_score: u32,
    #[serde(rename = "sl")]
    pub last_score: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ImageLink {
    EmbeddedSVG(String),
    FilePath(String),
}

const JSON_DATA: &str = include_str!("../data/infos.json"); // Embed the JSON file
pub fn get_data() -> AllInfos {
    let mut all_infos: AllInfos = serde_json::from_str(JSON_DATA).unwrap();
    all_infos.all_countries
            .sort_by(|a, b| a.infos[0].full.cmp(&b.infos[0].full));

    all_infos
}

pub fn read(all_countries: &Vec<CountryInfos>, score_path: &PathBuf) -> HashMap<String, Score> {
    if score_path.exists() {
        let mut file = File::open(score_path).unwrap();
        let mut file_content = String::new();
        file.read_to_string(&mut file_content).unwrap();

        if let Ok(score) = serde_json::from_str(&file_content) {
            return score;
        }
    }
    all_countries
        .iter()
        .map(|country| (country.infos[0].full.clone(), Score::default()))
        .collect()
}

pub fn save(scores: &HashMap<String, Score>, score_path: &PathBuf) {
    let json_data = serde_json::to_string(scores).unwrap();
    let mut file = File::create(score_path).unwrap();
    file.write_all(json_data.as_bytes()).unwrap();
}

pub fn delete_score(score_folder: &PathBuf) {
    if score_folder.exists() {
        fs::remove_dir_all(score_folder.clone()).unwrap();
    }
    // init_score_folder(score_folder);
}

pub fn init_score_folder(score_folder: &PathBuf) {
    if !score_folder.exists() {
        fs::create_dir_all(score_folder).unwrap();
    }
}

pub fn rename_score_folder(path1: &PathBuf, path2: &PathBuf) {
    fs::rename(path1, path2).unwrap();
}

pub fn list_folders(path: &Path) -> Vec<String> {
    let a = fs::read_dir(path).unwrap();
    let mut v: Vec<String> = a
        .into_iter()
        .map(|x| x.unwrap().file_name().into_string().unwrap())
        .collect();
    v.sort();
    v
}
