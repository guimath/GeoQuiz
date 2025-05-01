use num_format::{Locale, ToFormattedString};
use serde::Deserialize;
use serde_json;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use geo_quiz::info_parse::{AllInfos, Category, CountryInfos, ImageLink};

#[derive(Deserialize, Debug, Clone)]
struct CountryName {
    common: String,
    // official: String,
}

#[derive(Deserialize, Debug, Clone)]
struct Currency {
    name: String,
    symbol: String,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
enum CurrencyType {
    PRESENT(HashMap<String, Currency>),
    #[allow(dead_code)]
    EMPTY(Vec<String>),
}
#[derive(Deserialize, Debug, Clone)]
struct CountryStat {
    name: CountryName,
    cca3: String,
    #[serde(rename = "unMember")]
    un_member: bool,
    capital: Vec<String>,
    currencies: Option<CurrencyType>,
    languages: HashMap<String, String>,
    region: String,
    subregion: String,
    borders: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct OtherStat {
    cca3: String,
    #[serde(rename = "pop2025")]
    population: u64,
    #[serde(rename = "landAreaKm")]
    area: f64,
}

fn hint_from_name(s: String) -> String {
    let mut hint = s.chars().nth(0).unwrap_or(' ').to_string();
    hint.push_str("...");
    for _ in 1..s.split_whitespace().count() {
        hint.push_str(" ...");
    }
    hint
}

struct CategoryCreator {
    pub cca3_to_name: HashMap<String, String>,
    pub cca3_to_area: HashMap<String, f64>,
    pub cca3_to_pop: HashMap<String, u64>,
}

impl CategoryCreator {
    fn get_country(&self, x: &CountryStat) -> Category {
        let full = x.name.common.clone();
        Category {
            full: full.clone(),
            hint: Some(hint_from_name(full)),
        }
    }
    
    fn get_capital(&self, x: &CountryStat) -> Category {
        let hint: Vec<String> = x
            .capital
            .iter()
            .map(|s| hint_from_name(s.clone()))
            .collect();
        if x.capital.len() == 0 {
            Category {
                full: "No capital".to_string(),
                hint: Some("No capital".to_string()),
            }
        } else {
            Category {
                full: x.capital.join(", "),
                hint: Some(hint.join(", ")),
            }
        }
    }
    
    fn get_border(&self, x: &CountryStat) -> Category {
        let full: Vec<String> = x
            .borders
            .iter()
            .map(|s| self.cca3_to_name.get_key_value(s).unwrap().1.to_string())
            .collect();
        if full.len() == 0 {
            Category {
                full: "No borders".to_string(),
                hint: Some("No borders".to_string()),
            }
        } else {
            let hint: Vec<String> = full.iter().map(|s| hint_from_name(s.clone())).collect();
            Category {
                full: full.join(", "),
                hint: Some(hint.join(", ")),
            }
        }
    }
    fn get_area(&self, x: &CountryStat) -> Category {
        let value_string = if let Some(val) = self.cca3_to_area.get(&x.cca3.to_string()) {
            if *val > 100.0 {
                let v = val.round() as u64;
                format!("{} km²", v.to_formatted_string(&Locale::fr))
            } else {
                format!("{} km²", val)
            }
        } else {
            // println!("{} - empty area",x.name.common.clone());
            "No data".to_string()
        };
        Category {
            full: value_string,
            hint: None,
        }
    }

    fn get_population(&self, x: &CountryStat) -> Category {
        let value_string = if let Some(val) = self.cca3_to_pop.get(&x.cca3.to_string()) {
            val.to_formatted_string(&Locale::fr)
        } else {
            // println!("{} ({}) - empty population",x.name.common.clone(), x.cca3);
            "No data".to_string()
        };
        Category {
            full: value_string,
            hint: None,
        }
    }
    fn get_language(&self, x: &CountryStat) -> Category {
        let full: Vec<String> = x.languages.values().map(|v| v.to_string()).collect();
        let hint: Vec<String> = full.iter().map(|s| hint_from_name(s.clone())).collect();
        if full.len() == 0 {
            Category {
                full: "No data".to_string(),
                hint: Some("No data".to_string()),
            }
        } else {
            Category {
                full: full.join(", "),
                hint: Some(hint.join(", ")),
            }
        }
    }
    fn get_currency(&self, x: &CountryStat) -> Category {
        let mut full = Vec::new();
        if let Some(c) = x.currencies.clone() {
            if let CurrencyType::PRESENT(currencies) = c {
                for (code, currency) in currencies {
                    full.push(format!("{} ({}, {})", currency.name, code, currency.symbol));
                    //// Euro (EUR, €)
                }
            }
        }
        if full.len() == 0 {
            Category {
                full: "No data".to_string(),
                hint: None,
            }
        } else {
            Category {
                full: full.join(", "),
                hint: None,
            }
        }
    }

    fn get_region(&self, x: &CountryStat) -> Category {
        let full = format!("{} ({})", x.subregion, x.region.clone());
        Category { full, hint: None }
    }

    fn get_flag(&self, x: &CountryStat) -> ImageLink {
        let svg_path_o = format!("sources/flags/{}.svg", x.cca3.to_lowercase());
        let svg_path = Path::new(&svg_path_o);
        ImageLink::EmbeddedSVG(
            fs::read_to_string(svg_path).unwrap(),
        )
    }
    fn get_map(&self, x: &CountryStat) -> ImageLink {
        let svg_path_o = format!("maps/{}.svg", x.cca3.to_lowercase());
        ImageLink::FilePath(svg_path_o)
    }
    fn get_outline(&self, x: &CountryStat) -> ImageLink {
        let svg_path_o = format!("outlines/{}.svg", x.cca3.to_lowercase());
        ImageLink::FilePath(svg_path_o)
    }
}

const JSON_DATA: &str = include_str!("../sources/countries.json");
const OTHER_DATA: &str = include_str!("../sources/world_pop.csv");

fn main() {
    let mut cca3_to_name = HashMap::new();
    let mut cca3_to_pop: HashMap<String, u64> = HashMap::new();
    let mut cca3_to_area: HashMap<String, f64> = HashMap::new();
    let mut rdr = csv::Reader::from_reader(OTHER_DATA.as_bytes());
    for result in rdr.deserialize() {
        if result.is_err() {
            println!("CSV parse err");
            continue;
        }
        let r: OtherStat = result.unwrap();
        cca3_to_area.insert(r.cca3.clone(), r.area);
        cca3_to_pop.insert(r.cca3.clone(), r.population);
    }
    let raw: Vec<CountryStat> = serde_json::from_str(JSON_DATA).unwrap();
    for country in raw.clone() {
        cca3_to_name.insert(country.cca3, country.name.common);
    }

    let cat = CategoryCreator{
        cca3_to_name,
        cca3_to_pop,
        cca3_to_area,
    };

    let all_countries: Vec<CountryInfos> = raw
        .iter()
        .map(|x: &CountryStat| {
            let mut infos: Vec<Category> = Vec::new();
            infos.push(cat.get_country(x));
            infos.push(cat.get_capital(x));
            infos.push(cat.get_border(x));
            infos.push(cat.get_area(x));
            infos.push(cat.get_population(x));
            infos.push(cat.get_language(x));
            infos.push(cat.get_currency(x));
            infos.push(cat.get_region(x));

            let mut images: Vec<ImageLink> = Vec::new();
            images.push(cat.get_flag(x));
            images.push(cat.get_map(x));
            images.push(cat.get_outline(x));
            
            CountryInfos {
                region: x.region.clone(),
                un_member: x.un_member,
                infos,
                images,
            }
        })
        .collect();
    let un = all_countries.iter().filter(|x| x.un_member).count();
    println!("All done, total count: {} ({} UN)", all_countries.len(), un);
    let converted = AllInfos {
        all_countries,
        info_names: vec![
            "Country".to_string(),
            "Capital".to_string(),
            "Borders".to_string(),
            "Area".to_string(),
            "Population".to_string(),
            "Language".to_string(),
            "Currency".to_string(),
            "Region".to_string(),
        ],
        image_names: vec![
            "Flag".to_string(),
            "Map".to_string(),
            "Outline".to_string(),
        ],
    };
    let out_json = serde_json::to_string(&converted).unwrap();
    let mut file = File::create("data/infos.json").unwrap();
    file.write_all(out_json.as_bytes()).unwrap();
}

// Area & population data missing 
// Åland Islands
// Antarctica
// French Southern and Antarctic Lands
// Saint Helena, Ascension and Tristan da Cunha
// Caribbean Netherlands
// Bouvet Island
// Cocos (Keeling) Islands
// Christmas Island
// Heard Island and McDonald Islands
// British Indian Ocean Territory
// Kosovo
// Norfolk Island
// Pitcairn Islands
// South Georgia
// Svalbard and Jan Mayen
// United States Minor Outlying Islands
 
// No capital (normal)
// Antarctica
// Bouvet Island
// Heard Island and McDonald Islands
// Macau
// United States Minor Outlying Islands