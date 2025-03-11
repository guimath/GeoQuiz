use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
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
