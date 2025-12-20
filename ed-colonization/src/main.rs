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

    // for testing `config.save` and `config.reload` and avoid `never used` warnings
    config.data.totals_only = !config.data.totals_only;
    config.save();
    config.reload();
    println!("[debug] Totals only: {}", config.data.totals_only);
}
