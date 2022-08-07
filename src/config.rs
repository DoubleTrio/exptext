use crate::{ExpError, ExpLanguage};
use std::path::PathBuf;

#[derive(clap::Parser, Debug)]
#[clap(author, version, about, long_about = None)] // Read from `Cargo.toml`
pub struct Config {
    #[clap(
        short = 'f',
        long = "no-format",
        action,
        help = "Do not format the dialogue of original script"
    )]
    pub no_format: bool,

    #[clap(
        short = 's',
        long = "sort",
        action,
        help = "Sort the dialogue by message type"
    )]
    pub sort_by_type: bool,

    #[clap(
        short = 'n',
        long = "number",
        action,
        help = "Number the text at the line number it appears"
    )]
    pub show_line_number: bool,

    #[clap(short='l', long="language", value_parser = lang_filter, value_name = "LANGUAGE", help="Filter by language (ex: english, italian)")]
    pub lang_filter: Option<ExpLanguage>,

    #[clap(
        short = 'd',
        long = "db",
        parse(from_os_str),
        default_value = "actors.json",
        help = "The path to actor json database"
    )]
    pub db_path: PathBuf,

    #[clap(name = "FILES", parse(from_os_str))]
    pub files: Vec<PathBuf>,
}

fn lang_filter(s: &str) -> Result<ExpLanguage, String> {
    let lang: Result<ExpLanguage, ExpError> = s.try_into();
    match lang {
        Ok(l) => Ok(l),
        Err(e) => Err(format!("{}", e)),
    }
}
