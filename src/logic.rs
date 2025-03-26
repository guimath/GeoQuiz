use rand::seq::SliceRandom;
use rand::thread_rng;
use slint::{Image, SharedString};

use std::collections::HashMap;
use std::str::FromStr;
use std::{cmp::Ordering, path::PathBuf};

use crate::info_parse::{self, CountryInfos, Score};

slint::include_modules!();

#[derive(Default)]
pub enum ImageType {
    #[default]
    FLAG,
    OUTLINE,
}
impl ImageType {
    pub fn from_int(i: i32) -> Self {
        match i {
            0 => ImageType::FLAG,
            1 => ImageType::OUTLINE,
            _ => panic!("Not in image type"),
        }
    }
}
impl FromStr for ImageType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "Flags" => Ok(ImageType::FLAG),
            "Outlines" => Ok(ImageType::OUTLINE),
            _ => Err(()),
        }
    }
}

#[derive(Default)]
pub enum InfoType {
    #[default]
    COUNTRY,
    CAPITAL,
    LANGUAGES,
    CURRENCIES,
    LATLON,
    BORDERS,
    REGION,
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
    pub fn from_int(i: i32) -> Self {
        match i {
            0 => InfoType::COUNTRY,
            1 => InfoType::CAPITAL,
            2 => InfoType::LANGUAGES,
            3 => InfoType::BORDERS,
            4 => InfoType::REGION,
            5 => InfoType::CURRENCIES,
            6 => InfoType::LATLON,
            _ => panic!("Not in info type"),
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

#[derive(Default)]
pub struct AppLogic {
    current: usize,
    results: Vec<u32>,
    scores: HashMap<String, Score>,
    all_countries: Vec<CountryInfos>,
    all_countries_order: Vec<CountryInfos>,
    pub all_names: Vec<SharedString>,
    search_names: Vec<String>,
    info_types: [InfoType; 3],
    image_type: ImageType,
    score_path: PathBuf,
}
impl AppLogic {
    pub fn new(score_path: PathBuf) -> Self {
        let mut s = Self::default();
        s.all_countries_order = info_parse::get_data();
        s.all_countries_order
            .sort_by(|a, b| a.name.full.cmp(&b.name.full));
        s.all_names = s
            .all_countries_order
            .iter()
            .map(|x| x.name.full.clone().into())
            .collect();
        s.search_names = s.all_names.iter().map(|x| x.to_lowercase()).collect();
        s.score_path = score_path.join("score.json");
        s
    }

    pub fn prepare_infos(
        &mut self,
        easy_first: bool,
        hard_mode: bool,
        info_types: [InfoType; 3],
        img: ImageType,
    ) {
        let scores = info_parse::read(&self.all_countries_order, self.score_path.clone());

        let mut all_countries = if !hard_mode {
            self.all_countries_order
                .clone()
                .into_iter()
                .filter(|country| country.independent)
                .collect()
        } else {
            self.all_countries_order.clone()
        };

        let mut rng = thread_rng();
        all_countries.shuffle(&mut rng);
        let compare = |a: &CountryInfos, b: &CountryInfos| -> Ordering {
            scores
                .get(&b.cca3.clone())
                .unwrap()
                .total_score
                .cmp(&scores.get(&a.cca3.clone()).unwrap().total_score)
        };
        if easy_first {
            all_countries.sort_by(|a, b| compare(a, b));
        } else {
            all_countries.sort_by(|b, a| compare(a, b));
        }
        let len = all_countries.len();

        self.all_countries = all_countries;
        self.current = 0;
        self.results = vec![0; len];
        self.scores = scores;
        self.image_type = img;
        self.info_types = info_types;
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
            score.total_score = (score.total_score + result) - self.results[self.current];
            score.last_score = result;
            self.results[self.current] = result;
            self.save_scores(); // TODO ONLY SAVE WHEN LEAVING APP OR BACK TO MENU
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
        let score = self.results[self.current] as i32;
        let last_score = self
            .scores
            .get(&self.all_countries[self.current].cca3.clone())
            .unwrap()
            .last_score as i32;
        let raw_svg = match self.image_type {
            ImageType::FLAG => country.svg_flag,
            ImageType::OUTLINE => country.svg_outline,
        };
        let update = FullUpdate {
            flag: Image::load_from_svg_data(raw_svg.as_bytes()).unwrap(),
            num: self.current as i32,
            out_of: self.all_countries.len() as i32,
            score,
            last_score,
            seen: score != 0,
        };
        let infos: [CatInfo; 3] = (0..3)
            .map(|i| {
                let cat = match self.info_types[i] {
                    InfoType::COUNTRY => country.name.clone(),
                    InfoType::CAPITAL => country.capitals.clone(),
                    InfoType::CURRENCIES => country.currencies.clone(),
                    InfoType::LANGUAGES => country.languages.clone(),
                    InfoType::REGION => country.region.clone(),
                    InfoType::BORDERS => country.borders.clone(),
                    InfoType::LATLON => panic!("NOT DONE"),
                };
                CatInfo {
                    full: cat.full.into(),
                    category: self.info_types[i].to_str().into(),
                    first: cat.hint.clone().unwrap_or(" ".to_string()).into(),
                    with_hint: cat.hint.is_some(),
                }
            })
            .collect::<Vec<CatInfo>>()
            .try_into()
            .unwrap();
        (update, infos)
    }

    pub fn search_changed(&self, s: String) -> Vec<bool> {
        let s = s.to_lowercase();
        let search = s.as_str();
        self.search_names
            .iter()
            .map(|x| x.contains(search))
            .collect()
    }

    pub fn look_up_selected(&self, num: usize) -> FullInfo {
        let country = self.all_countries_order[num].clone();
        let name = country.name.full;
        let mut text_infos: Vec<TextWithTitle> = Vec::new();
        let mut image_infos: Vec<ImageWithTitle> = Vec::new();

        macro_rules! add_text_info {
            ($title: expr, $data:expr ) => {
                text_infos.push(TextWithTitle {
                    title: $title.into(),
                    text: $data.into(),
                });
            };
        }
        macro_rules! add_image_info {
            ($title: expr, $data:expr ) => {
                image_infos.push(ImageWithTitle {
                    title: $title.into(),
                    image: Image::load_from_svg_data($data.as_bytes()).unwrap(),
                });
            };
        }
        add_text_info!("Capital", country.capitals.full);
        add_text_info!("Languages", country.languages.full);
        add_text_info!("Region", country.region.full);
        add_text_info!("Borders", country.borders.full);
        add_text_info!("Currencies", country.currencies.full);
        let is_independant = if country.independent { "Yes" } else { "No" };
        add_text_info!("Independant", is_independant);
        add_image_info!("Flag", country.svg_flag);
        add_image_info!("Outline", country.svg_outline);

        FullInfo {
            name: name.into(),
            text_infos: text_infos.as_slice().into(),
            image_infos: image_infos.as_slice().into(),
        }
    }

    pub fn save_scores(&self) {
        info_parse::save(&self.scores, self.score_path.clone());
    }
    pub fn reset_score(&self) {
        info_parse::reset_score(self.score_path.clone())
    }
}
