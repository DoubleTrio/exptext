use clap::Parser;
use exptext::ExpError;

fn main() {
    if let Err(e) = exptext::Config::try_parse()
        .map_err(|e| ExpError::Clap(e))
        .and_then(exptext::run)
    {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
