use std::fs::{read_dir, File};
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use config::Config;
use regex::Regex;
use serde_json::from_str;
use events::JournalEntry;

mod config;
mod events;

const APP_NAME: &str = "ed-colonization";

fn main() {
    let mut config = Config::new();
    let journal_location = String::from(&config.data.journal_location);
    println!("[debug] Config file: {}", &config.file.display());
    println!("Journal Location: {}", journal_location);
    println!("Minimum stock: {}", &config.data.minimum_stock);
    println!("Tracked:");
    let tracked_constructions = &config.data.tracked_constructions;
    for tracked_construction in tracked_constructions {
        println!("  {}", tracked_construction);
    }
    println!("List all available: {}", &config.data.list_all);
    println!("Show totals: {}", &config.data.show_totals);
    println!("Totals Only: {}", &config.data.totals_only);

    // for testing `config.save` and `config.reload` and avoid `never used` warnings
    config.data.totals_only = !config.data.totals_only;
    config.save();
    config.reload();
    println!("[debug] Totals only: {}", config.data.totals_only);

    let paths = read_dir(&journal_location).unwrap();

    let journal_re = Regex::new(r"Journal\.\d{4}-\d{2}-\d{2}T\d{6}\.\d{2}\.log").unwrap();
    let mut journal_files: Vec<String> = vec![];
    for path in paths {
        let copy = path.as_ref().unwrap();
        let is_file = &copy.path().is_file();
        if !is_file {
            continue;
        }
        let file_name = &copy.file_name().to_str().unwrap().to_string();
        let journal_match = journal_re.is_match(&file_name);
        if !journal_match {
            continue;
        }
        journal_files.push(String::from(file_name));
    }
    let latest_journal_file = journal_files.pop().unwrap();
    println!("[debug] Latest Journal file: {}", latest_journal_file);
    let open_latest_journal = File::open(PathBuf::from(&journal_location).join(&latest_journal_file)).unwrap();
    let reader = BufReader::new(open_latest_journal);
    for line in reader.lines() {
        let entry: JournalEntry = from_str(line.as_ref().unwrap().as_str()).unwrap();
        println!("[debug] Timestamp: {}; event: {}", entry.timestamp, entry.event);
        println!("[debug] Line: {}", line.unwrap());
    }
}
