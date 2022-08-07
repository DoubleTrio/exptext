use std::fmt::Display;

use crate::ExpMessageSpecial;

#[derive(Debug)]
pub enum ExpSpeakerType {
    Actor(String),
    Other(ExpMessageSpecial),
    Unknown,
}

impl Display for ExpSpeakerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ExpSpeakerType::Actor(actor_name) => write!(f, "{}", actor_name),
            ExpSpeakerType::Other(display) => write!(f, "{}", display),
            ExpSpeakerType::Unknown => write!(f, "???"),
        }
    }
}
