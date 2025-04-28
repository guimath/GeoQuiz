use num_format::{Locale, ToFormattedString};
use serde::Deserialize;
use serde_json;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

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
    independent: Option<bool>,
    capital: Vec<String>,
    currencies: Option<CurrencyType>,
    languages: HashMap<String, String>,
    region: String,
    subregion: String,
    borders: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct PopulationStat {
    #[serde(rename = "Country Code")]
    cca3: String,
    #[serde(rename = "2023")]
    population: u64,
}

#[derive(Debug, Deserialize)]
struct AreaStat {
    #[serde(rename = "Country Code")]
    cca3: String,
    #[serde(rename = "2022")]
    area: f64,
}
use geo_quiz::info_parse::{AllInfos, Category, CountryInfos, ImageLink};

fn hint_from_name(s: String) -> String {
    let mut hint = s.chars().nth(0).unwrap_or(' ').to_string();
    hint.push_str("...");
    for _ in 1..s.split_whitespace().count() {
        hint.push_str(" ...");
    }
    hint
}

const JSON_DATA: &str = include_str!("../sources/countries.json");
const POP_DATA: &str = include_str!("../sources/world_bank_pop.csv");
const AREA_DATA: &str = include_str!("../sources/world_bank_area.csv");
fn main() {
    let mut rdr = csv::Reader::from_reader(POP_DATA.as_bytes());
    let mut cca_to_pop: HashMap<String, u64> = HashMap::new();
    for result in rdr.deserialize() {
        if result.is_err() {
            continue;
        }
        let r: PopulationStat = result.unwrap();
        cca_to_pop.insert(r.cca3, r.population);
    }
    let mut rdr = csv::Reader::from_reader(AREA_DATA.as_bytes());
    let mut cca_to_area: HashMap<String, f64> = HashMap::new();
    for result in rdr.deserialize() {
        if result.is_err() {
            continue;
        }
        let r: AreaStat = result.unwrap();
        cca_to_area.insert(r.cca3, r.area);
    }
    let raw: Vec<CountryStat> = serde_json::from_str(JSON_DATA).unwrap();
    let mut cca3_to_name = HashMap::new();
    for country in raw.clone() {
        cca3_to_name.insert(country.cca3, country.name.common);
    }

    let all_countries: Vec<CountryInfos> = raw
        .iter()
        .map(|x: &CountryStat| {
            let mut infos: Vec<Category> = Vec::new();
            // ---------- COUNTRY ----------
            let full = x.name.common.clone();
            infos.push(Category {
                full: full.clone(),
                hint: Some(hint_from_name(full)),
            });
            // ---------- CAPITAL ----------
            let hint: Vec<String> = x
                .capital
                .iter()
                .map(|s| hint_from_name(s.clone()))
                .collect();
            let cat = if x.capital.len() == 0 {
                Category {
                    full: "No data".to_string(),
                    hint: Some("No data".to_string()),
                }
            } else {
                Category {
                    full: x.capital.join(", "),
                    hint: Some(hint.join(", ")),
                }
            };
            infos.push(cat);
            // ---------- BORDERS ----------
            let full: Vec<String> = x
                .borders
                .iter()
                .map(|s| cca3_to_name.get_key_value(s).unwrap().1.to_string())
                .collect();
            let cat = if full.len() == 0 {
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
            };
            infos.push(cat);
            // ---------- AREA ----------
            let value_string = if let Some(val) = cca_to_area.get(&x.cca3.to_string()) {
                if *val > 100.0 {
                    let v = val.round() as u64;
                    format!("{} km²", v.to_formatted_string(&Locale::fr))
                } else {
                    format!("{} km²", val)
                }
            } else {
                //// println!("{} - empty area",x.name.common.clone());
                "No data".to_string()
            };
            infos.push(Category {
                full: value_string,
                hint: None,
            });
            // ---------- POPULATION ----------
            let value_string = if let Some(val) = cca_to_pop.get(&x.cca3.to_string()) {
                val.to_formatted_string(&Locale::fr)
            } else {
                //// println!("{} - empty population",x.name.common.clone());
                "No data".to_string()
            };
            infos.push(Category {
                full: value_string,
                hint: None,
            });
            // ---------- LANGUAGES ----------
            let full: Vec<String> = x.languages.values().map(|v| v.to_string()).collect();
            let hint: Vec<String> = full.iter().map(|s| hint_from_name(s.clone())).collect();
            let cat = if full.len() == 0 {
                Category {
                    full: "No data".to_string(),
                    hint: Some("No data".to_string()),
                }
            } else {
                Category {
                    full: full.join(", "),
                    hint: Some(hint.join(", ")),
                }
            };
            infos.push(cat);
            // ---------- CURRENCIES ----------
            let mut full = Vec::new();
            if let Some(c) = x.currencies.clone() {
                if let CurrencyType::PRESENT(currencies) = c {
                    for (code, currency) in currencies {
                        full.push(format!("{} ({}, {})", currency.name, code, currency.symbol));
                        //// Euro (EUR, €)
                    }
                }
            }
            let cat = if full.len() == 0 {
                Category {
                    full: "No data".to_string(),
                    hint: None,
                }
            } else {
                Category {
                    full: full.join(", "),
                    hint: None,
                }
            };
            infos.push(cat);
            // ---------- REGION ----------
            let full = format!("{} ({})", x.subregion, x.region.clone());
            infos.push(Category { full, hint: None });




            //// println!("{}",x.cca3.clone());
            let mut images: Vec<ImageLink> = Vec::new();
            // ---------- FLAG ----------
            let svg_path_o = format!("sources/flags/{}.svg", x.cca3.to_lowercase());
            let svg_path = Path::new(&svg_path_o);
            images.push(ImageLink::EmbeddedSVG(
                fs::read_to_string(svg_path).unwrap(),
            ));
            // ---------- MAP ----------
            let svg_path_o = format!("maps/{}.svg", x.cca3.to_lowercase());
            images.push(ImageLink::FilePath(svg_path_o));
            // ---------- OUTLINE ----------
            let svg_path_o = format!("outlines/{}.svg", x.cca3.to_lowercase());
            images.push(ImageLink::FilePath(svg_path_o));
            CountryInfos {
                region: x.region.clone(),
                independent: x.independent.unwrap_or(false),
                infos,
                images,
            }
        })
        .collect();
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
