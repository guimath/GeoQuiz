use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::prelude::IteratorRandom;
use slint::{Image, ModelRc, SharedString, VecModel};

use std::collections::HashMap;
use std::{cmp::Ordering, path::PathBuf};

use crate::info_parse::{self, CountryInfos, Score};

slint::include_modules!();
const CATEGORIES_TXT: [&str; 6] = [
    "Country",
    "Capital",
    "Languages",
    "Currencies",
    "Borders",
    "Region",
];
const CATEGORIES_IMG: [&str; 2] = ["Flag", "Outline"];
const CATEGORIES: [&str; 8] = [
    "Flag",
    "Outline",
    "Country",
    "Capital",
    "Languages",
    "Currencies",
    "Borders",
    "Region",
];
const NUM_IMG_TYPE: usize = CATEGORIES_IMG.len();
fn is_info_txt(i: usize) -> bool {
    i >= NUM_IMG_TYPE
}
fn txt_only_to_global_type(i: usize) -> usize {
    i + NUM_IMG_TYPE
}
fn to_txt_idx(i: usize) -> usize {
    i - NUM_IMG_TYPE
}

fn load_img(raw_data: &String) -> Image {
    Image::load_from_svg_data(raw_data.as_bytes()).unwrap()
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
    main_info_type: usize,
    main_guess_types: [usize; 3],
    choice_prev_guesses: HashMap<usize,ModelRc<bool>>,
    choice_index_guesses: HashMap<usize, [usize;4]>,
    choice_info_type: usize,
    choice_guess_type: usize,
    score_path: PathBuf,
    
}

impl AppLogic {
    pub fn new(score_path: PathBuf) -> Self {
        let mut s = Self::default();
        s.all_countries_order = info_parse::get_data();
        s.all_countries_order
            .sort_by(|a, b| a.infos[0].full.cmp(&b.infos[0].full));
        s.all_names = s
            .all_countries_order
            .iter()
            .map(|x| x.infos[0].full.clone().into())
            .collect();
        s.search_names = s.all_names.iter().map(|x| x.to_lowercase()).collect();
        s.score_path = score_path.join("score.json");
        s
    }

    pub fn set_config(&mut self, easy_first: bool, hard_mode: bool) {
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
    }

    pub fn prepare_main_play(&mut self, info_type: usize, guess_types: [usize; 3]) {
        self.main_guess_types = [
            txt_only_to_global_type(guess_types[0]),
            txt_only_to_global_type(guess_types[1]),
            txt_only_to_global_type(guess_types[2]),
        ];
        self.main_info_type = info_type;
    }

    pub fn next(&mut self, result: u32) -> Option<(MainPlayUpdate, [CatInfo; 3])> {
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

    pub fn prev(&mut self) -> Option<(MainPlayUpdate, [CatInfo; 3])> {
        if self.current > 0 {
            self.current -= 1;
            return Some(self.get_stat());
        }
        None
    }

    pub fn get_stat(&mut self) -> (MainPlayUpdate, [CatInfo; 3]) {
        let country = self.all_countries[self.current].clone();
        let score = self.results[self.current] as i32;
        let last_score = self
            .scores
            .get(&self.all_countries[self.current].cca3.clone())
            .unwrap()
            .last_score as i32;

        let mut info = TxtOrImg {
            is_txt: is_info_txt(self.main_info_type),
            txt: SharedString::default(),
            img: Image::default(),
        };

        if info.is_txt {
            info.txt = country.infos[to_txt_idx(self.main_info_type)]
                .full
                .clone()
                .into();
        } else {
            info.img = load_img(&country.images[self.main_info_type]);
        }

        let update = MainPlayUpdate {
            info,
            num: self.current as i32,
            out_of: self.all_countries.len() as i32,
            score,
            last_score,
            seen: score != 0,
        };
        let infos: [CatInfo; 3] = (0..3)
            .map(|i| {
                let cat = country.infos[to_txt_idx(self.main_guess_types[i])].clone();
                CatInfo {
                    full: cat.full.into(),
                    category: CATEGORIES[self.main_guess_types[i]].into(),
                    first: cat.hint.clone().unwrap_or(" ".to_string()).into(),
                    with_hint: cat.hint.is_some(),
                }
            })
            .collect::<Vec<CatInfo>>()
            .try_into()
            .unwrap();
        (update, infos)
    }


    pub fn prepare_choice_play(&mut self, info_type: usize, guess_type: usize) {
        self.choice_guess_type = guess_type;
        self.choice_info_type = info_type;

    }

    pub fn choice_changed(&mut self, was_guessed: ModelRc<bool>, next:bool) -> Option<ChoicePlayUpdate> {
        self.choice_prev_guesses.insert(self.current, was_guessed);
        if next {
            if self.current < self.all_countries.len() {
                self.current += 1;
            } else {
                return None;
            }
        } else {
            if self.current > 0 {
                self.current -= 1;
            } else {
                return None;
            }
        }
        Some(self.get_choices())
    }
    
    pub fn get_choices(&mut self) -> ChoicePlayUpdate {
        let mut info = TxtOrImg { img: Image::default(), is_txt: is_info_txt(self.choice_info_type), txt: SharedString::default()};
        let mut guesses = vec![
            TxtOrImg { img: Image::default(), is_txt: is_info_txt(self.choice_guess_type), txt: SharedString::default()},
            TxtOrImg { img: Image::default(), is_txt: is_info_txt(self.choice_guess_type), txt: SharedString::default()},
            TxtOrImg { img: Image::default(), is_txt: is_info_txt(self.choice_guess_type), txt: SharedString::default()},
            TxtOrImg { img: Image::default(), is_txt: is_info_txt(self.choice_guess_type), txt: SharedString::default()},
        ];
        let prev_guess = match self.choice_prev_guesses.get(&self.current) {
            Some(v) => v.clone(),
            None => {
                let d: [bool; 4]=  [false; 4];
                VecModel::from_slice(&d)
            }
        };
        let guess_idx = match self.choice_index_guesses.get(&self.current) {
            Some(v) => v.clone(),
            None => {
                let mut rng = rand::thread_rng();
                let mut random_elements: Vec<usize> = (0usize..self.all_countries.len())
                    .choose_multiple(&mut rng, 6);
                let target = self.current as i32; 
                random_elements = random_elements
                    .into_iter()
                    .filter(|x| {
                    (*x as i32) < (target - 1) || (*x as i32) > (target + 1)
                }).collect();
                random_elements = vec![random_elements[0], random_elements[1], random_elements[2]];
                random_elements.push(self.current);
                random_elements.shuffle(&mut rng);
                let v = [random_elements[0], random_elements[1], random_elements[2], random_elements[3]];
                self.choice_index_guesses.insert(self.current, v);
                v
            }
        };
        let correct_guess = guess_idx.iter().position(|x| *x == self.current).unwrap();
        let country = self.all_countries[self.current].clone();
        if info.is_txt {
            info.txt = country.infos[to_txt_idx(self.choice_info_type)].full.clone().into();
        } else {
            info.img = load_img(&country.images[self.choice_info_type]);
        }
        if guesses[0].is_txt {
            let idx = to_txt_idx(self.choice_guess_type); 
            for i in 0..4 {
                guesses[i].txt = self.all_countries[guess_idx[i]].infos[idx].full.clone().into();
            }
        } else {
            let idx = self.choice_guess_type; 
            for i in 0..4 {
                guesses[i].img = load_img(&self.all_countries[guess_idx[i]].images[idx]);
            }
        }
        ChoicePlayUpdate { 
            correct_guess: correct_guess as i32, 
            guess_num: 0, 
            guesses: VecModel::from_slice(&guesses), 
            info: info, 
            num: self.current as i32, 
            out_of: self.all_countries.len() as i32, 
            prev_guess: prev_guess.clone()
        }
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
        let name = country.infos[0].full.clone();
        let mut text_infos: Vec<TextWithTitle> = Vec::new();
        let mut image_infos: Vec<ImageWithTitle> = Vec::new();

        for i in 0..CATEGORIES.len() {
            if is_info_txt(i) {
                text_infos.push(TextWithTitle {
                    title: CATEGORIES[i].into(),
                    text: country.infos[to_txt_idx(i)].full.clone().into(),
                });
            } else {
                image_infos.push(ImageWithTitle {
                    title: CATEGORIES[i].into(),
                    image: load_img(&country.images[i]),
                });
            }
        }

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
    pub fn get_all_categories_names(&self) -> ModelRc<SharedString> {
        let v: VecModel<SharedString> = CATEGORIES.iter().map(|&x| SharedString::from(x)).collect();
        ModelRc::new(v)
    }
    pub fn get_txt_categories_names(&self) -> ModelRc<SharedString> {
        let v: VecModel<SharedString> = CATEGORIES_TXT
            .iter()
            .map(|&x| SharedString::from(x))
            .collect();
        ModelRc::new(v)
    }
}
