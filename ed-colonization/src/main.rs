use std::io::{stdin, stdout, Write};
use config::Config;

mod config;

const APP_NAME: &str = "ed-colonization";

fn main() {
    let mut config = Config::new();
    let journal_location = &config.data.journal_location;
    println!("[debug] Config file: {}", &config.file.display());
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
