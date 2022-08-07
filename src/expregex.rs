pub mod re {
    use lazy_static::lazy_static;
    use regex::Regex;

    const EXP_TEXT_PATTERN: &str = r"[a-zA-Z][0-9a-zA-Z_:]";
    const QUOTE_TEXT_PATTERN: &str = r#"["'](?P<text>.*)?["']"#;
    lazy_static! {
        pub static ref LANGUAGE_TEXT_REGEX: Regex = Regex::new(
            format!(
                r#"(?P<language>english|french|german|italian|spanish)[\s]*?=[\s]*?{}"#,
                QUOTE_TEXT_PATTERN.to_string(),
            )
            .as_str()
        )
        .unwrap();
        pub static ref NO_LANGUAGE_TEXT_REGEX: Regex = Regex::new(QUOTE_TEXT_PATTERN).unwrap();
        pub static ref FUNCTION_TEXT_REGEX: Regex = Regex::new(
            format!(
                r"(?P<function>{}*|case menu)[\s]*?\(",
                EXP_TEXT_PATTERN.to_string()
            )
            .as_str()
        )
        .unwrap();
        pub static ref ACTOR_TEXT_REGEX: Regex =
            Regex::new(format!(r"ACTOR_(?P<actor>{}*)", EXP_TEXT_PATTERN.to_string()).as_str())
                .unwrap();
        pub static ref BRACKET_TEXT_REGEX: Regex =
            Regex::new(format!(r"\[{}*\]", EXP_TEXT_PATTERN.to_string()).as_str()).unwrap();
        pub static ref CTYPE_TEXT_REGEX: Regex =
            Regex::new(format!(r"\[c_({0}*):({0}*)\]", EXP_TEXT_PATTERN.to_string()).as_str())
                .unwrap();
        pub static ref SWITCH_CASE_REGEX: Regex =
            Regex::new(r"(case|default)[\s*]?\d?[\s*]?:").unwrap();
        pub static ref END_FUNC_REGEX: Regex = Regex::new(r"[\s*]?\);").unwrap();
        pub static ref END_CASE_MENU_REGEX: Regex = Regex::new(r"[\s*]?\):").unwrap();
        pub static ref SPACE_REGEX: Regex = Regex::new(r"\s+").unwrap();
    }
}
