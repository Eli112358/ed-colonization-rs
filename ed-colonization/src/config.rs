use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use cross_xdg::BaseDirs;
use serde_json::{from_str, to_string_pretty};
use crate::APP_NAME;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigData {
    pub(crate) journal_location: String,
    pub(crate) minimum_stock: i64,
    pub(crate) list_all: bool,
    pub(crate) show_totals: bool,
    pub(crate) totals_only: bool,
    pub(crate) tracked_constructions: Vec<i64>,
}

#[derive(Debug)]
pub struct Config {
    file: PathBuf,
    pub(crate) data: ConfigData,
}

impl Config {
    pub(crate) fn new() -> Self {
        let base_dirs = BaseDirs::new().unwrap();
        let config_home = base_dirs.config_home();
        let app_config_dir = config_home.join(APP_NAME);
        let file = app_config_dir.join("config.json");
        let content = fs::read_to_string(&file).unwrap();
        let data = from_str(&content).unwrap();
        Self { file, data, }
    }

    pub(crate) fn reload(&mut self) {
        let content = fs::read_to_string(&self.file).unwrap();
        self.data = from_str(&content).unwrap();
    }

    pub(crate) fn save(&self) {
        let json_data = to_string_pretty(&self.data).unwrap();
        fs::write(&self.file, json_data).unwrap();
    }
}
