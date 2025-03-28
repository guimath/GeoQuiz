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
use geo_quiz::info_parse::{Category, CountryInfos};

fn hint_from_name(s: String) -> String {
    let mut s = s.chars().nth(0).unwrap_or(' ').to_string();
    s.push_str("...");
    s
}

const JSON_DATA: &str = include_str!("../data/countries.json"); // Embed the JSON file
fn main() {
    let raw: Vec<CountryStat> = serde_json::from_str(JSON_DATA).unwrap();
    let mut cca3_to_name = HashMap::new();
    for country in raw.clone() {
        cca3_to_name.insert(country.cca3, country.name.common);
    }

    let converted: Vec<CountryInfos> = raw
        .iter()
        .map(|x: &CountryStat| {
            let mut infos: Vec<Category> = Vec::new();
            let mut txt_cat_names: Vec<&str> = Vec::new();
            // NAME
            txt_cat_names.push("Country");
            let full = x.name.common.clone();
            infos.push(Category {
                full: full.clone(),
                hint: Some(hint_from_name(full)),
            });
            // CAPITAL
            txt_cat_names.push("Capital");
            let hint: Vec<String> = x
                .capital
                .iter()
                .map(|s| hint_from_name(s.clone()))
                .collect();
            infos.push(Category {
                full: x.capital.join(", "),
                hint: Some(hint.join(", ")),
            });
            // LANGUAGES
            txt_cat_names.push("Languages");
            let full: Vec<String> = x.languages.values().map(|v| v.to_string()).collect();
            let hint: Vec<String> = full.iter().map(|s| hint_from_name(s.clone())).collect();
            infos.push(Category {
                full: full.join(", "),
                hint: Some(hint.join(", ")),
            });
            // CURRENCIES
            txt_cat_names.push("Currencies");
            let mut full = Vec::new();
            if let Some(c) = x.currencies.clone() {
                if let CurrencyType::PRESENT(currencies) = c {
                    for (code, currency) in currencies {
                        full.push(format!("{} ({}, {})", currency.name, code, currency.symbol));
                        // Euro (EUR, €)
                    }
                }
            }
            infos.push(Category {
                full: full.join(", "),
                hint: None,
            });
            // REGION
            txt_cat_names.push("Region");
            let full = format!("{} ({})", x.subregion, x.region);
            infos.push(Category { full, hint: None });
            // BORDERS
            txt_cat_names.push("Borders");
            let full: Vec<String> = x
                .borders
                .iter()
                .map(|s| cca3_to_name.get_key_value(s).unwrap().1.to_string())
                .collect();
            let hint: Vec<String> = full.iter().map(|s| hint_from_name(s.clone())).collect();
            infos.push(Category {
                full: full.join(", "),
                hint: Some(hint.join(", ")),
            });

            let mut images: Vec<String> = Vec::new();
            let mut img_cat_names: Vec<&str> = Vec::new();
            // SVG_FLAG
            img_cat_names.push("Flag");
            let svg_path_o = format!("data/flags/{}.svg", x.cca3.to_lowercase());
            let svg_path = Path::new(&svg_path_o);
            images.push(fs::read_to_string(svg_path).unwrap());
            // SVG_OUTLINE
            img_cat_names.push("Outline");
            let svg_path_o = format!("data/outlines/{}.svg", x.cca3.to_lowercase());
            let svg_path = Path::new(&svg_path_o);
            images.push(fs::read_to_string(svg_path).unwrap());
            CountryInfos {
                cca3: x.cca3.clone(),
                independent: x.independent.unwrap_or(false),
                infos,
                images,
            }
        })
        .collect();
    let out_json = serde_json::to_string(&converted).unwrap();
    let mut file = File::create("data/infos.json").unwrap();
    file.write_all(out_json.as_bytes()).unwrap();
}
