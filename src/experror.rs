use std::{fmt::Display, io};

#[derive(Debug)]
pub enum ExpError {
    Io(io::Error),
    Serde(serde_json::Error),
    Clap(clap::Error),
    Regex(regex::Error),
    ObjectError,
    LanguageError,
}

impl From<io::Error> for ExpError {
    fn from(e: io::Error) -> Self {
        ExpError::Io(e)
    }
}

impl From<serde_json::Error> for ExpError {
    fn from(e: serde_json::Error) -> Self {
        ExpError::Serde(e)
    }
}

impl From<clap::Error> for ExpError {
    fn from(e: clap::Error) -> Self {
        ExpError::Clap(e)
    }
}

impl From<regex::Error> for ExpError {
    fn from(e: regex::Error) -> Self {
        ExpError::Regex(e)
    }
}

impl Display for ExpError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ExpError::Io(e) => write!(f, "{}", e),
            ExpError::Serde(e) => write!(f, "{}", e),
            ExpError::Clap(e) => write!(f, "{}", e),
            ExpError::Regex(e) => write!(f, "{}", e),
            ExpError::ObjectError => write!(f, "Failed to convert JSON into object"),
            ExpError::LanguageError => write!(f, ""),
        }
    }
}
