use crate::{db::Database, Config, ExpError, ExpTextRetriever};

use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::PathBuf,
};

pub fn run(config: Config) -> Result<(), ExpError> {
    let db = Database::new(&config.db_path)?;
    let mut ssb_text_retrieve = ExpTextRetriever::new(
        db,
        config.no_format,
        config.sort_by_type,
        config.lang_filter,
        config.show_line_number,
    );
    let num_files = config.files.len();

    for (file_num, filename) in config.files.iter().enumerate() {
        let file_display = filename.display();
        match open(filename) {
            Ok(file) => {
                if num_files > 1 {
                    println!(
                        "{}==> {} <==",
                        if file_num > 0 { "\n" } else { "" },
                        &file_display
                    );
                }
                ssb_text_retrieve.output(file)?;
            }
            Err(err) => {
                let display = file_display;
                eprintln!("{}: {}", display, err)
            }
        }
    }

    Ok(())
}

fn open(filename: &PathBuf) -> Result<Box<dyn BufRead>, Box<dyn std::error::Error>> {
    let default = &PathBuf::from("-");
    match filename {
        path if path == default => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
