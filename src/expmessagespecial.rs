use std::fmt::Display;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ExpMessageSpecial {
    Explanation,
    Narration,
    Mail,
    Notice,
    Debug,
    Banner,
    CaseMenu,
    Unknown,
}

impl Display for ExpMessageSpecial {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Explanation => write!(f, "message_Explanation"),
            Self::Narration => write!(f, "message_Narration"),
            Self::Mail => write!(f, "message_Mail"),
            Self::Notice => write!(f, "message_Notice"),
            Self::Debug => write!(f, "debug_Print"),
            Self::Banner => write!(f, "back_SetBanner2"),
            Self::CaseMenu => write!(f, "case_menu"),
            Self::Unknown => write!(f, "UNKNOWN"),
        }
    }
}

impl From<&str> for ExpMessageSpecial {
    fn from(item: &str) -> Self {
        match item {
            "message_Explanation" => Self::Explanation,
            "message_Narration" => Self::Narration,
            "message_Mail" => Self::Mail,
            "message_Notice" => Self::Notice,
            "debug_Print" => Self::Debug,
            "back_SetBanner2" => Self::Banner,
            "case menu" => Self::CaseMenu,
            _ => Self::Unknown,
        }
    }
}
