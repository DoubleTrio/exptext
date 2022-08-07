use crate::{ExpLanguage, ExpSpeakerType};

#[derive(Debug)]
pub struct ExpText {
    pub speaker: ExpSpeakerType,
    text: String,
    switch_id: u32,
    line_num: u32,
    pub language: ExpLanguage,
}

impl ExpText {
    pub fn new_default(
        speaker: ExpSpeakerType,
        text: String,
        language: ExpLanguage,
        line_num: u32,
    ) -> Self {
        Self {
            speaker,
            text,
            language,
            line_num,
            switch_id: 0,
        }
    }

    pub fn new(
        speaker: ExpSpeakerType,
        text: String,
        language: ExpLanguage,
        line_num: u32,
        switch_id: u32,
    ) -> Self {
        Self {
            speaker,
            text,
            language,
            line_num,
            switch_id,
        }
    }
}

impl ExpText {
    pub fn display(&self, show_line_num: bool) {
        let Self {
            speaker,
            switch_id,
            text,
            line_num,
            language: _,
        } = self;

        let switch_id = if switch_id == &0 {
            "".to_string()
        } else {
            format!("({})", self.switch_id.to_string())
        };

        println!(
            "{}[{}]{}: {}",
            format_field(&line_num, show_line_num),
            speaker,
            switch_id,
            text
        )
    }
}

fn format_field(value: &u32, show: bool) -> String {
    if show {
        format!("{:<8}", value)
    } else {
        "".to_string()
    }
}
