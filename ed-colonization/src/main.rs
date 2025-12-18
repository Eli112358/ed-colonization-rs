use cross_xdg::BaseDirs;
use serde::{Deserialize, Serialize};
use serde_json::de::from_str;
use serde_json::to_string_pretty;
use std::fs;
use std::io::{stdin, stdout, Write};
use std::path::PathBuf;

const APP_NAME: &str = "ed-colonization";

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigData {
    journal_location: String,
    minimum_stock: i64,
    list_all: bool,
    show_totals: bool,
    totals_only: bool,
    tracked_constructions: Vec<i64>,
}

#[derive(Debug)]
pub struct Config {
    file: PathBuf,
    data: ConfigData,
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

fn main() {
    let mut config = Config::new();
    let journal_location = &config.data.journal_location;
    println!("Journal Location: {}", journal_location);
    println!("Minimum stock: {}", config.data.minimum_stock);
    println!("Tracked:");
    let tracked_constructions = &config.data.tracked_constructions;
    for tracked_construction in tracked_constructions {
        println!("  {}", tracked_construction);
    }
    println!("List all available: {}", config.data.list_all);
    println!("Show totals: {}", config.data.show_totals);
    println!("Totals Only: {}", config.data.totals_only);
    loop {
        // for testing `config.save` and `config.reload`
        let mut buffer = String::new();
        let stdin = stdin();
        print!("Please hit enter to toggle totals only:");
        let _=stdout().flush();
        stdin.read_line(&mut buffer).unwrap_or(0);
        config.data.totals_only = !config.data.totals_only;
        config.save();
        config.reload();
        println!("[debug] Totals only: {}", config.data.totals_only);
    }
}
