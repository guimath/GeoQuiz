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

    let converted: Vec<CountryInfos> = raw.iter().map(|x: &CountryStat| {
        // NAME
        let full = x.name.common.clone();
        let name = Category {
            full: full.clone(),
            hint: Some(hint_from_name(full)),
        };
        // CAPITAL
        let hint: Vec<String> = x
            .capital
            .iter()
            .map(|s| hint_from_name(s.clone()))
            .collect();
        let capitals = Category{
            full: x.capital.join(", "),
            hint: Some(hint.join(", "))
        };
        // LANGUAGES
        let full: Vec<String> = x.languages.values().map(|v| v.to_string()).collect();
        let hint: Vec<String> = full.iter().map(|s| hint_from_name(s.clone())).collect();
        let languages = Category {
            full: full.join(", "),
            hint: Some(hint.join(", ")),
        };
        // CURRENCIES
        let mut full = Vec::new();
        if let Some(c) = x.currencies.clone() {
            if let CurrencyType::PRESENT(currencies) = c {
                for (code, currency) in currencies {
                    full.push(format!(
                        "{} ({}, {})",
                        currency.name, code, currency.symbol
                    ));
                    // Euro (EUR, €)
                }
            }
        }
        let currencies = Category {
            full: full.join(", "),
            hint: None
        };
        // REGION
        let full = format!("{} ({})", x.subregion, x.region);
        let region = Category{
            full,
            hint:None
        };
        // BORDERS
        let full: Vec<String> = x
            .borders
            .iter()
            .map(|s| cca3_to_name.get_key_value(s).unwrap().1.to_string())
            .collect();
        let hint: Vec<String> = full
            .iter()
            .map(|s| hint_from_name(s.clone()))
            .collect();
        let borders = Category{
            full: full.join(", "),
            hint: Some(hint.join(", ")),
        };
        // SVG_FLAG
        let svg_path_o = format!("data/flags/{}.svg", x.cca3.to_lowercase());
        let svg_path = Path::new(&svg_path_o);
        let svg_flag = fs::read_to_string(svg_path).unwrap();
        // SVG_OUTLINE
        let svg_path_o = format!("data/outlines/{}.svg", x.cca3.to_lowercase());
        let svg_path = Path::new(&svg_path_o);
        let svg_outline = fs::read_to_string(svg_path).unwrap();
        CountryInfos { 
            cca3: x.cca3.clone(), 
            independent: x.independent.unwrap_or(false), 
            name, 
            capitals,
            currencies,
            languages,
            region,
            borders,
            svg_flag,
            svg_outline,
        }
    }).collect();
    let out_json = serde_json::to_string(&converted).unwrap();
    let mut file = File::create("data/infos.json").unwrap();
    file.write_all(out_json.as_bytes()).unwrap();
}

