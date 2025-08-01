use rand::seq::SliceRandom;
use rand::thread_rng;
use slint::{Image, Model, ModelRc, SharedString, VecModel};

use crate::info_parse::{self, AllInfos, CountryInfos, ImageLink, Score};
use std::collections::HashMap;
use std::{cmp::Ordering, path::PathBuf};

slint::include_modules!();

const MAIN_SCORE_NAME: &str = "score_main.json";
const CHOICE_SCORE_NAME: &str = "score_choice.json";
pub const SUB_CAT_NAMES: [&str; 6] = ["World", "Africa", "Americas", "Asia", "Europe", "Oceania"];
pub struct AppLogic<'a> {
    order_type: u32,
    current: usize,
    results: Vec<u32>,
    scores: HashMap<String, Score>,
    last_scores: HashMap<String, usize>,
    all_countries: Vec<&'a CountryInfos>,
    all_countries_order: &'a Vec<CountryInfos>,
    cat_img_len: usize,
    all_cat_names: Vec<String>,
    search_names: Vec<String>,
    main_info_type: usize,
    main_guess_types: [usize; 3],
    choice_prev_guesses: HashMap<usize, ModelRc<bool>>,
    choice_index_guesses: HashMap<usize, [usize; 4]>,
    choice_info_type: usize,
    choice_guess_type: usize,
    score_path: PathBuf,
    data_path: PathBuf,
    score_folder: PathBuf,
}

#[derive(Default)]
pub struct ScoreStats {
    pub main_avg: [i32; 6],
    pub main_last: [i32; 6],
    pub choice_avg: [i32; 5],
    pub choice_last: [i32; 5],
    pub main_max: i32,
    pub choice_max: i32,
}

fn get_score_key(country: &CountryInfos) -> &String {
    &country.infos[0].full
}
impl<'a> AppLogic<'a> {
    pub fn new(score_path: &PathBuf, all_data: &'a AllInfos) -> Self {
        let all_cat_names: Vec<String> = all_data
            .info_names
            .iter()
            .cloned()
            .chain(all_data.image_names.iter().cloned())
            .collect();
        let search_names = all_data
            .all_countries
            .iter()
            .map(|x| x.infos[0].full.to_lowercase())
            .collect();
        let mut score_folder = score_path.join("scores/User 1");
        let v = info_parse::list_folders(score_folder.parent().unwrap());
        if v.is_empty() {
            info_parse::init_score_folder(&score_folder);
        } else {
            score_folder.set_file_name(&v[0]);
        }
        Self {
            order_type: Default::default(),
            current: Default::default(),
            results: Default::default(),
            scores: Default::default(),
            last_scores: Default::default(),
            all_countries: Default::default(),
            all_countries_order: &all_data.all_countries,
            cat_img_len: all_data.image_names.len(),
            all_cat_names,
            search_names,
            main_info_type: Default::default(),
            main_guess_types: Default::default(),
            choice_prev_guesses: Default::default(),
            choice_index_guesses: Default::default(),
            choice_info_type: Default::default(),
            choice_guess_type: Default::default(),
            score_path: Default::default(),
            data_path: score_path.join("data"),
            score_folder,
        }
    }
    pub fn get_all_categories_name(&self) -> &Vec<String> {
        &self.all_cat_names
    }
    pub fn set_config(&mut self, conf: PlaySelectParams) {
        self.score_path = if conf.play_type {
            self.score_folder.join(MAIN_SCORE_NAME)
        } else {
            self.score_folder.join(CHOICE_SCORE_NAME)
        };

        self.all_countries = if !conf.include_hard {
            self.all_countries_order
                .iter()
                .filter(|country| country.un_member)
                .collect()
        } else {
            self.all_countries_order.iter().collect()
        };
        if conf.region_idx > 0 {
            let idx = conf.region_idx as usize;
            self.all_countries
                .retain(|country| country.region == SUB_CAT_NAMES[idx]);
        }
        self.order_type = conf.order as u32;
    }

    fn randomize_order(&mut self) {
        let scores = info_parse::read(&self.all_countries_order, &self.score_path);

        let mut rng = thread_rng();
        self.all_countries.shuffle(&mut rng);
        let compare = |a: &CountryInfos, b: &CountryInfos| -> Ordering {
            scores
                .get(get_score_key(b))
                .unwrap()
                .total_score
                .cmp(&scores.get(get_score_key(a)).unwrap().total_score)
        };
        if self.order_type == 0 {
            self.all_countries.sort_by(|a, b| compare(a, b));
        } else if self.order_type == 2 {
            self.all_countries.sort_by(|b, a| compare(a, b));
        }

        self.current = 0;
        self.results = vec![0; self.all_countries.len()];
        self.scores = scores;
        self.choice_prev_guesses = Default::default();
        self.choice_index_guesses = Default::default();
        self.last_scores = Default::default();
    }

    pub fn prepare_main_play(&mut self, info_type: usize, guess_types: [usize; 3]) {
        self.main_guess_types = [
            self.txt_only_to_global_type(guess_types[0]),
            self.txt_only_to_global_type(guess_types[1]),
            self.txt_only_to_global_type(guess_types[2]),
        ];
        self.main_info_type = info_type;
        self.randomize_order()
    }

    pub fn next(&mut self, result: u32) -> Option<(MainPlayUpdate, [CatInfo; 3])> {
        let score_key = get_score_key(&self.all_countries[self.current]).to_owned();
        if result != 0 {
            let score = self.scores.get_mut(&score_key).unwrap();
            if self.results[self.current] == 0 {
                score.time_played += 1;
            }
            score.total_score = (score.total_score + result) - self.results[self.current];
            score.last_score = result;
            self.results[self.current] = result;
            self.save_scores();
            self.last_scores.insert(score_key, result as usize);
        } else if self.results[self.current] == 0 {
            self.last_scores.insert(score_key, 0);
        }
        if !self.is_at_end() {
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
        let country = &self.all_countries[self.current];
        let score = self.results[self.current] as i32;
        let last_score = self
            .scores
            .get(get_score_key(&self.all_countries[self.current]))
            .unwrap()
            .last_score as i32;

        let mut info = TxtOrImg {
            is_txt: self.is_info_txt(self.main_info_type),
            txt: SharedString::default(),
            img: Image::default(),
        };

        if info.is_txt {
            info.txt = country.infos[self.to_txt_idx(self.main_info_type)]
                .full
                .as_str()
                .into();
        } else {
            info.img = self.load_img(&country.images[self.main_info_type]);
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
                let cat = &country.infos[self.to_txt_idx(self.main_guess_types[i])];
                CatInfo {
                    full: cat.full.as_str().into(),
                    category: self.all_cat_names[self.main_guess_types[i]].as_str().into(),
                    first: cat
                        .hint
                        .as_ref()
                        .map(|s| s.as_str().into())
                        .unwrap_or(SharedString::new()),
                    with_hint: cat.hint.is_some(),
                }
            })
            .collect::<Vec<CatInfo>>()
            .try_into()
            .unwrap();
        (update, infos)
    }

    pub fn get_play_scores(&self, choice_play: bool) -> (Vec<i32>, i32) {
        let mut v = [0; 6];
        for s in self.last_scores.values() {
            v[*s] += 1;
        }
        let choice_max = *v.iter().max().unwrap();
        if choice_play {
            (v[0..5].to_vec(), choice_max)
        } else {
            (v.to_vec(), choice_max)
        }
    }
    pub fn prepare_choice_play(&mut self, info_type: usize, guess_type: usize) {
        self.choice_guess_type = guess_type;
        self.choice_info_type = info_type;
        self.randomize_order()
    }

    pub fn choice_changed(
        &mut self,
        was_guessed: ModelRc<bool>,
        next: bool,
        found: bool,
    ) -> Option<ChoicePlayUpdate> {
        self.choice_prev_guesses
            .insert(self.current, was_guessed.clone());
        let score_key = get_score_key(&self.all_countries[self.current]).to_owned();
        if found {
            let down_ref: &VecModel<bool> = was_guessed.as_any().downcast_ref().unwrap();
            let guess_num = down_ref.iter().filter(|&x| x).count();
            let score = self.scores.get_mut(&score_key).unwrap();
            score.time_played += 1;
            score.last_score = (5 - guess_num) as u32;
            score.total_score += (5 - guess_num) as u32;
            self.save_scores();
            self.last_scores.insert(score_key, 5 - guess_num);
        } else if !self.last_scores.contains_key(&score_key) && next {
            self.last_scores.insert(score_key, 0);
        }

        if next {
            if !self.is_at_end() {
                self.current += 1;
            } else {
                return None;
            }
        } else if self.current > 0 {
            self.current -= 1;
        } else {
            return None;
        }
        Some(self.get_choices())
    }

    fn choice_same_info(&self, idx: usize) -> bool {
        if self.is_info_txt(self.choice_info_type) {
            let info = &self.all_countries[idx].infos[self.choice_info_type].full;
            let compare = &self.all_countries[self.current].infos[self.choice_info_type].full;
            info == compare
        } else {
            let info = &self.all_countries[idx].images[self.choice_info_type];
            let compare = &self.all_countries[self.current].images[self.choice_info_type];
            info == compare
        }
    }
    fn generate_guesses(&self) -> [usize; 4] {
        let guess_type = self.choice_guess_type;
        let unique_indices: Vec<usize> = if self.is_info_txt(guess_type) {
            let guess_idx = self.to_txt_idx(guess_type);
            let mut hash_map: HashMap<String, usize> = HashMap::new();
            for (idx, item) in self.all_countries.iter().enumerate() {
                if self.choice_same_info(idx) {
                    continue;
                }
                hash_map
                    .entry(item.infos[guess_idx].full.clone())
                    .or_insert(idx);
            }
            hash_map.remove(&self.all_countries[self.current].infos[guess_idx].full);
            hash_map.into_values().collect()
        } else {
            let mut hash_map = HashMap::new();
            for (idx, item) in self.all_countries.iter().enumerate() {
                if self.choice_same_info(idx) {
                    continue;
                }
                hash_map
                    .entry(item.images[guess_type].clone())
                    .or_insert(idx);
            }
            hash_map.remove(&self.all_countries[self.current].images[guess_type]);
            hash_map.into_values().collect()
        };
        let mut rng = rand::thread_rng();
        let mut random_elements: Vec<&usize> =
            unique_indices.choose_multiple(&mut rng, 3).collect();
        random_elements.push(&&self.current);
        if random_elements.len() != 4 {
            // TODO treat cases less than 4 possible choices (rare but you never know)
            panic!("Not enough possibilities to chose from")
        }
        random_elements.shuffle(&mut rng);
        [
            *random_elements[0],
            *random_elements[1],
            *random_elements[2],
            *random_elements[3],
        ]
    }

    pub fn get_choices(&mut self) -> ChoicePlayUpdate {
        let mut info = TxtOrImg::default();
        info.is_txt = self.is_info_txt(self.choice_info_type);
        let mut guesses = vec![TxtOrImg::default(); 4];
        for i in 0..4 {
            guesses[i].is_txt = self.is_info_txt(self.choice_guess_type);
        }

        // getting previous guesses or default (no guess) + counting guesses
        let prev_guess = match self.choice_prev_guesses.get(&self.current) {
            Some(v) => v.clone(),
            None => {
                let d: [bool; 4] = [false; 4];
                VecModel::from_slice(&d)
            }
        };
        let down_ref: &VecModel<bool> = prev_guess.as_any().downcast_ref().unwrap();
        let guess_num = down_ref.iter().filter(|&x| x).count();
        // getting randomly sorted array of guess idx
        let guess_idx = match self.choice_index_guesses.get(&self.current) {
            Some(v) => *v,
            None => {
                let v = self.generate_guesses();
                self.choice_index_guesses.insert(self.current, v);
                v
            }
        };
        let correct_guess = guess_idx.iter().position(|x| *x == self.current).unwrap();

        let country = &self.all_countries[self.current];
        // adding default info only if the idx 0 info is not either infos
        let default_type = self.txt_only_to_global_type(0);
        let default_info =
            if self.choice_guess_type != default_type && self.choice_info_type != default_type {
                &country.infos[0].full
            } else {
                &String::new()
            };

        if info.is_txt {
            info.txt = country.infos[self.to_txt_idx(self.choice_info_type)]
                .full
                .as_str()
                .into();
        } else {
            info.img = self.load_img(&country.images[self.choice_info_type]);
        }

        if guesses[0].is_txt {
            let idx = self.to_txt_idx(self.choice_guess_type);
            for i in 0..4 {
                guesses[i].txt = self.all_countries[guess_idx[i]].infos[idx]
                    .full
                    .as_str()
                    .into();
            }
        } else {
            let idx = self.choice_guess_type;
            for i in 0..4 {
                guesses[i].img = self.load_img(&self.all_countries[guess_idx[i]].images[idx]);
            }
        }
        ChoicePlayUpdate {
            correct_guess: correct_guess as i32,
            guess_num: guess_num as i32,
            guesses: VecModel::from_slice(&guesses),
            info,
            num: self.current as i32,
            out_of: self.all_countries.len() as i32,
            prev_guess,
            default_info: default_info.into(),
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
    pub fn look_up_current(&self) -> FullInfo {
        self.get_full_info_country(&self.all_countries[self.current])
    }
    pub fn look_up_selected(&self, num: usize) -> FullInfo {
        self.get_full_info_country(&self.all_countries_order[num])
    }
    fn get_full_info_country(&self, country: &CountryInfos) -> FullInfo {
        let name = &country.infos[0].full;
        let mut text_infos: Vec<TextWithTitle> = Vec::new();
        let mut image_infos: Vec<ImageWithTitle> = Vec::new();

        for i in 0..self.all_cat_names.len() {
            if self.is_info_txt(i) {
                text_infos.push(TextWithTitle {
                    title: self.all_cat_names[i].as_str().into(),
                    text: country.infos[self.to_txt_idx(i)].full.as_str().into(),
                });
            } else {
                image_infos.push(ImageWithTitle {
                    title: self.all_cat_names[i].as_str().into(),
                    image: self.load_img(&country.images[i]),
                });
            }
        }
        let status = if country.un_member {
            if country.infos[0].full == "Vatican City" {
                SharedString::from("Non-member but permanent observer state")
            } else {
                SharedString::from("Yes")
            }
        } else {
            SharedString::from("No")
        };
        text_infos.push(TextWithTitle {
            title: SharedString::from("UN Member"),
            text: status,
        });

        let mut val = [0; 2];
        for (i, score_type) in [MAIN_SCORE_NAME, CHOICE_SCORE_NAME].iter().enumerate() {
            let path = self.score_folder.join(score_type);
            let s = info_parse::read(&self.all_countries_order, &path);
            let score = s.get(name).unwrap();
            val[i] = score.last_score as i32;
        }
        FullInfo {
            name: name.into(),
            text_infos: text_infos.as_slice().into(),
            image_infos: image_infos.as_slice().into(),
            score_free_play: val[0],
            score_choice_play: val[1],
            wiki_link: country.wiki_link.as_str().into(),
        }
    }

    pub fn score_user_selected(&mut self, name: String) {
        self.score_folder.pop();
        self.score_folder.push(name);
    }
    pub fn score_user_change(&mut self, name: String, delete: bool) {
        self.score_folder.set_file_name(&name);
        if delete {
            info_parse::delete_score(&self.score_folder);
            let mut v = self.list_users();
            v.retain(|x| x != &name);
            if v.is_empty() {
                self.score_folder.set_file_name("User 1");
            } else {
                self.score_folder.set_file_name(v.remove(0));
            }
        }
        println!("{:?}", self.score_folder);
        info_parse::init_score_folder(&self.score_folder);
    }
    pub fn score_rename_user(&mut self, name1: String, name2: String) {
        let mut p1 = self.score_folder.clone();
        p1.set_file_name(name1);
        self.score_folder.set_file_name(name2);
        info_parse::rename_score_folder(&p1, &self.score_folder)
    }

    pub fn score_filter_changed(&mut self, all: bool) {
        self.all_countries = if !all {
            self.all_countries_order
                .iter()
                .filter(|country| country.un_member)
                .collect()
        } else {
            self.all_countries_order.iter().collect()
        };
    }
    pub fn score_sub_cat_changed(&self, sub_cat_idx: usize) -> ScoreStats {
        let filtered_countries: Vec<&String> = if sub_cat_idx == 0 {
            self.all_countries
                .iter()
                .map(|x| &x.infos[0].full)
                .collect()
        } else {
            self.all_countries
                .iter()
                .filter(|country| country.region == SUB_CAT_NAMES[sub_cat_idx])
                .map(|x| &x.infos[0].full)
                .collect()
        };

        let score_path_main = self.score_folder.join(MAIN_SCORE_NAME);
        let score_path_choice = self.score_folder.join(CHOICE_SCORE_NAME);
        let main_scores = info_parse::read(&self.all_countries_order, &score_path_main);
        let choice_scores = info_parse::read(&self.all_countries_order, &score_path_choice);

        let mut stat = ScoreStats::default();
        for country_name in filtered_countries {
            let s = main_scores.get(country_name).unwrap();
            stat.main_last[s.last_score as usize] += 1;
            if s.time_played > 0 {
                let avg = ((s.total_score as f32) / (s.time_played as f32)).round() as usize;
                stat.main_avg[avg] += 1;
            } else {
                stat.main_avg[0] += 1;
            }
            let s = choice_scores.get(country_name).unwrap();
            stat.choice_last[s.last_score as usize] += 1;
            if s.time_played > 0 {
                let avg = ((s.total_score as f32) / (s.time_played as f32)).round() as usize;
                stat.choice_avg[avg] += 1;
            } else {
                stat.choice_avg[0] += 1;
            }
        }
        stat.main_max = *stat
            .main_avg
            .iter()
            .max()
            .unwrap()
            .max(stat.main_last.iter().max().unwrap());
        stat.choice_max = *stat
            .choice_avg
            .iter()
            .max()
            .unwrap()
            .max(stat.choice_last.iter().max().unwrap());
        stat
    }

    fn load_img(&self, image_link: &ImageLink) -> Image {
        match image_link {
            ImageLink::EmbeddedSVG(raw_data) => {
                Image::load_from_svg_data(raw_data.as_bytes()).unwrap()
            }
            ImageLink::FilePath(path) => {
                let p = self.data_path.join(path);
                Image::load_from_path(&p).unwrap()
            }
        }
    }
    pub fn get_active_user(&self) -> String {
        self.score_folder
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    }
    pub fn list_users(&self) -> Vec<String> {
        info_parse::list_folders(self.score_folder.parent().unwrap())
    }
    pub fn save_scores(&self) {
        info_parse::save(&self.scores, &self.score_path);
    }
    fn is_info_txt(&self, i: usize) -> bool {
        i >= self.cat_img_len
    }
    fn txt_only_to_global_type(&self, i: usize) -> usize {
        i + self.cat_img_len
    }
    fn to_txt_idx(&self, i: usize) -> usize {
        i - self.cat_img_len
    }
    pub fn is_at_end(&self) -> bool {
        self.current >= self.all_countries.len() - 1
    }
}
