use crate::{
    db::{BracketType, CType, SpecialText},
    re, ExpError,
};

use serde_derive::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

#[derive(Debug)]
pub struct Database {
    actors: HashMap<String, Actor>,
    special: PMDTextSpecial,
}

impl Database {
    pub fn new(db_path: &dyn AsRef<Path>) -> Result<Self, ExpError> {
        let mut hashmap: HashMap<String, Actor> = HashMap::new();
        let text = std::fs::read_to_string(&db_path)?;
        let json_db = serde_json::from_str::<JSONDB>(&text)?;

        let JSONDB { actors, special } = json_db;

        for actor in actors {
            hashmap.insert(actor.id.to_owned(), actor);
        }

        Ok(Self {
            actors: hashmap,
            special,
        })
    }

    pub fn get_actor(&self, query: DatabaseQuery) -> String {
        let DatabaseQuery { id, ctype } = query;
        let actor_name = self.actors.get(&id);
        match actor_name {
            Some(a) => match ctype {
                CType::Name => a.name.to_owned(),
                CType::Species => a.species.to_owned(),
                CType::Unknown => format!("{}:", id),
            },
            _ => {
                format!("{}?", id)
            }
        }
    }

    pub fn replace_bracket_text<'a>(&self, text: &'a str) -> String {
        let texts = Self::extract_bracketed_text(text);
        let mut replace_text = String::from(text);

        for t in texts {
            let bracket_type = BracketType::new(t);
            let r = self.get_substitute_text(bracket_type);
            replace_text = replace_text.replace(t, r.as_str());
        }
        replace_text
    }

    pub fn get_substitute_text(&self, bracket_type: BracketType) -> String {
        match bracket_type {
            BracketType::Query(q) => self.get_actor(q),
            BracketType::Special(s) => self.get_special(s),
        }
    }

    fn get_special(&self, special: SpecialText) -> String {
        match special {
            SpecialText::Hero => self.special.hero.to_owned(),
            SpecialText::Partner => self.special.partner.to_owned(),
            SpecialText::Team => self.special.team.to_owned(),
            SpecialText::R => String::from(" "),
            SpecialText::Icon(text) => format!("[{}]", text),
            _ => String::from(""),
        }
    }

    fn extract_bracketed_text(text: &str) -> HashSet<&str> {
        re::BRACKET_TEXT_REGEX
            .find_iter(text)
            .map(|mat| mat.as_str())
            .collect::<HashSet<&str>>()
    }
}

#[derive(Debug)]
pub struct DatabaseQuery {
    id: String,
    ctype: CType,
}

impl DatabaseQuery {
    pub fn new(id: String, ctype: CType) -> Self {
        Self { id, ctype }
    }
}

#[derive(Deserialize, Serialize, Debug)]
struct PMDTextSpecial {
    hero: String,
    partner: String,
    team: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
struct Actor {
    id: String,
    #[serde(rename = "kind")]
    species: String,
    name: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct JSONDB {
    actors: Vec<Actor>,
    special: PMDTextSpecial,
}
