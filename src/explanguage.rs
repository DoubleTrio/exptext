use std::convert::TryFrom;
use std::fmt::Display;

use crate::ExpError;
#[derive(Debug, Clone, PartialEq)]
pub enum ExpLanguage {
    English,
    French,
    German,
    Italian,
    Spanish,
}

impl Display for ExpLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::English => write!(f, "e"),
            Self::French => write!(f, "f"),
            Self::German => write!(f, "g"),
            Self::Italian => write!(f, "i"),
            Self::Spanish => write!(f, "s"),
        }
    }
}

impl TryFrom<&str> for ExpLanguage {
    type Error = crate::ExpError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "english" | "eng" | "en" | "e" => Ok(Self::English),
            "french" | "fr" | "f" => Ok(Self::French),
            "german" | "ger" | "der" | "g" | "de" | "d" => Ok(Self::German),
            "italian" | "ita" | "i" => Ok(Self::Italian),
            "spanish" | "spa" | "s" | "es" => Ok(Self::Spanish),
            _ => Err(ExpError::LanguageError),
        }
    }
}
