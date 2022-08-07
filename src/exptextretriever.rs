use crate::{
    db::{CType, Database, DatabaseQuery},
    re, ExpCommand, ExpError, ExpLanguage, ExpMessageSpecial, ExpSpeakerType, ExpText,
};

use lazysort::SortedBy;
use std::io::BufRead;

pub struct ExpTextRetriever {
    database: Database,
    texts: Vec<ExpText>,
    no_format: bool,
    lang_filter: Option<ExpLanguage>,
    sort_by_type: bool,
    #[allow(dead_code)]
    show_line_num: bool,
    actor: Option<String>,
}

struct CommandStructure {
    command: ExpCommand,
    statements: Vec<Statement>,
}

#[derive(Clone)]
struct Statement {
    statement: String,
    line_num: u32,
}

impl Statement {
    pub fn new(statement: String, line_num: u32) -> Self {
        Self {
            statement,
            line_num,
        }
    }
}

impl CommandStructure {
    pub fn new(command: ExpCommand, statement: Statement) -> Self {
        Self {
            command,
            statements: vec![statement],
        }
    }

    pub fn add(&mut self, statement: Statement) {
        self.statements.push(statement)
    }
}

impl ExpTextRetriever {
    pub fn new(
        db: Database,
        no_format: bool,
        sort_by_type: bool,
        lang_filter: Option<ExpLanguage>,
        show_line_num: bool,
    ) -> Self {
        Self {
            database: db,
            no_format,
            sort_by_type,
            texts: vec![],
            show_line_num,
            lang_filter,
            actor: None,
        }
    }

    pub fn output(&mut self, file: impl BufRead) -> Result<(), ExpError> {
        let statements = self.get_statements(file)?;
        let commands = self.get_commands(statements);
        for c in commands {
            self.eval_command_struct(c);
        }

        self.display();
        self.clear();

        Ok(())
    }

    fn get_commands(&self, statements: Vec<Statement>) -> Vec<CommandStructure> {
        let mut commands: Vec<CommandStructure> = vec![];

        let mut index = 0;
        let n = statements.len();

        while index < n {
            let mut s = &statements[index];
            index += 1;
            let command = self.get_command(s.statement.as_str());

            match command {
                Some(c) => {
                    if c == ExpCommand::Unknown {
                        continue;
                    } else if c == ExpCommand::SwitchTalk {
                        let mut command_struct = CommandStructure::new(c, s.to_owned());
                        s = &statements[index];
                        while self.get_command(s.statement.as_str()).is_none() && index + 1 < n {
                            command_struct.add(s.to_owned());
                            index += 1;
                            s = &statements[index];
                        }
                        commands.push(command_struct);
                    } else if c == ExpCommand::CaseMenu {
                        let mut command_struct = CommandStructure::new(c, s.to_owned());
                        while !re::END_CASE_MENU_REGEX.is_match(s.statement.as_str()) {
                            s = &statements[index];
                            command_struct.add(s.to_owned());
                            index += 1;
                        }
                        commands.push(command_struct);
                    } else {
                        let mut command_struct = CommandStructure::new(c, s.to_owned());
                        while !re::END_FUNC_REGEX.is_match(s.statement.as_str()) {
                            s = &statements[index];
                            command_struct.add(s.to_owned());
                            index += 1;
                        }
                        commands.push(command_struct);
                    }
                }
                None => continue,
            }
        }

        commands
    }

    fn get_statements(&self, mut file: impl BufRead) -> Result<Vec<Statement>, ExpError> {
        let mut line = String::new();
        let mut statements: Vec<Statement> = vec![];
        let mut line_num = 1;

        loop {
            let line_bytes = file.read_line(&mut line)?;
            if line_bytes == 0 {
                break Ok(statements);
            }

            let lines = line
                .split_inclusive(&[';'])
                .map(|l| l.trim())
                .filter(|l| l.ne(&""))
                .collect::<Vec<&str>>();

            for l in lines {
                statements.push(Statement::new(l.to_string(), line_num));
            }

            line_num += 1;
            line.clear();
        }
    }

    fn get_command(&self, statement: &str) -> Option<ExpCommand> {
        let result = re::FUNCTION_TEXT_REGEX.captures(statement);
        match result {
            Some(captures) => Some(captures["function"].to_string().as_str().into()),
            _ => None,
        }
    }

    fn set_actor(&mut self, actor: Option<String>) {
        self.actor = actor
    }

    fn get_actor(&mut self) -> Option<String> {
        self.actor.to_owned()
    }

    fn format(&self, text: String) -> String {
        if self.no_format {
            text
        } else {
            self.database
                .replace_bracket_text(&text)
                .trim()
                .replace("\\n", " ")
                .replace("\\'", "'")
                .replace("\\\"", "\"")
        }
    }

    fn eval_command_struct(&mut self, command_struct: CommandStructure) {
        match command_struct.command {
            ExpCommand::SpecialMessage(display) => {
                for s in command_struct.statements {
                    let text = s.statement;

                    let res = self.get_texts(text.as_str());
                    if res.is_ok() {
                        for (t, l) in res.unwrap() {
                            let speaker_type = ExpSpeakerType::Other(display);
                            let text = self.format(t);
                            let text_structure =
                                ExpText::new_default(speaker_type, text, l, s.line_num);
                            self.texts.push(text_structure)
                        }
                    }
                }
            }
            ExpCommand::SetActor => {
                let mut text = String::new();
                for s in command_struct.statements {
                    text += "\n";
                    text += &s.statement
                }

                let actor = self.get_actor_id(&text);
                self.set_actor(Some(actor));
            }
            ExpCommand::ResetActor => self.set_actor(None),

            ExpCommand::SwitchTalk => {
                let mut counter = 0;
                for s in command_struct.statements {
                    let text = s.statement;

                    if re::SWITCH_CASE_REGEX.is_match(text.as_str()) {
                        counter += 1;
                    }

                    let res = self.get_texts(text.as_str());
                    if res.is_ok() {
                        for (t, l) in res.unwrap() {
                            let speaker_type = {
                                let actor = self.get_actor();
                                match actor {
                                    Some(a) => ExpSpeakerType::Actor(
                                        self.database.get_actor(DatabaseQuery::new(a, CType::Name)),
                                    ),
                                    None => ExpSpeakerType::Unknown,
                                }
                            };
                            let text = self.format(t);
                            let text_structure =
                                ExpText::new(speaker_type, text, l, s.line_num, counter);
                            self.texts.push(text_structure)
                        }
                    }
                }
            }
            ExpCommand::Talk => {
                for s in command_struct.statements {
                    let text = s.statement;
                    let res = self.get_texts(text.as_str());
                    if res.is_ok() {
                        for (t, l) in res.unwrap() {
                            let speaker_type = {
                                let actor = self.get_actor();
                                match actor {
                                    Some(a) => ExpSpeakerType::Actor(
                                        self.database.get_actor(DatabaseQuery::new(a, CType::Name)),
                                    ),
                                    None => ExpSpeakerType::Unknown,
                                }
                            };
                            let text = self.format(t);
                            let text_structure =
                                ExpText::new_default(speaker_type, text, l, s.line_num);
                            self.texts.push(text_structure)
                        }
                    }
                }
            }
            ExpCommand::CaseMenu => {
                for s in command_struct.statements {
                    let text = s.statement;
                    let res = self.get_texts(text.as_str());
                    if res.is_ok() {
                        for (t, l) in res.unwrap() {
                            let speaker_type = ExpSpeakerType::Other(ExpMessageSpecial::CaseMenu);
                            let text = self.format(t);
                            let text_structure =
                                ExpText::new_default(speaker_type, text, l, s.line_num);
                            self.texts.push(text_structure)
                        }
                    }
                }
            }
            ExpCommand::Unknown => (),
        }
    }

    pub fn display(&self) {
        let filter = &self.lang_filter;
        let sort = &self.sort_by_type;
        let iter = self.texts.iter();
        let iter = iter.filter(|t| filter.as_ref().map_or(true, |l| t.language == *l));

        if *sort {
            for text_struct in
                iter.sorted_by(|a, b| a.speaker.to_string().cmp(&b.speaker.to_string()))
            {
                text_struct.display(self.show_line_num)
            }
        } else {
            for text_struct in iter {
                text_struct.display(self.show_line_num)
            }
        }
    }

    fn clear(&mut self) {
        self.texts.clear();
    }

    fn get_texts(&self, text: &str) -> Result<Vec<(String, ExpLanguage)>, ExpError> {
        let mut texts: Vec<(String, ExpLanguage)> = vec![];

        for caps in re::LANGUAGE_TEXT_REGEX.captures_iter(text) {
            let t = caps["text"].to_string();
            let language: ExpLanguage = caps["language"].try_into()?;
            texts.push((t, language));
        }

        if texts.len() == 0 {
            for caps in re::NO_LANGUAGE_TEXT_REGEX.captures_iter(text) {
                let t = caps["text"].to_string();
                texts.push((t, ExpLanguage::English));
            }
        }

        Ok(texts)
    }

    fn get_actor_id(&self, text: &str) -> String {
        let caps = re::ACTOR_TEXT_REGEX.captures(text).unwrap();
        caps["actor"].to_string()
    }
}
