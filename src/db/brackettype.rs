use crate::{
    db::{CType, DatabaseQuery, SpecialText},
    re,
};

pub enum BracketType<'a> {
    Query(DatabaseQuery),
    Special(SpecialText<'a>),
}

impl<'a> BracketType<'a> {
    pub fn extract_text_from_bracketed_text(text: &str) -> SpecialText {
        let end = text.len() - 1;
        text[1..end].into()
    }

    pub fn extract_ctype_from_bracketed_text(text: &str) -> Option<DatabaseQuery> {
        let result = re::CTYPE_TEXT_REGEX.captures(text);

        match result {
            Some(captures) if captures.len() == 3 => {
                let ctype: CType = captures.get(1).unwrap().as_str().into();
                let actor = captures.get(2).unwrap().as_str().to_string();
                Some(DatabaseQuery::new(actor, ctype))
            }

            _ => None,
        }
    }

    pub fn new(bracket_text: &'a str) -> Self {
        match BracketType::extract_ctype_from_bracketed_text(bracket_text) {
            Some(q) => Self::Query(q),
            _ => {
                let special = BracketType::extract_text_from_bracketed_text(bracket_text);
                Self::Special(special)
            }
        }
    }
}
