use crate::{re, ExpMessageSpecial};

#[derive(Debug, PartialEq, Clone)]
pub enum ExpCommand {
    SpecialMessage(ExpMessageSpecial),
    SetActor,
    ResetActor,
    SwitchTalk,
    CaseMenu,
    Talk,
    Unknown,
}

impl From<&str> for ExpCommand {
    fn from(item: &str) -> Self {
        let item_single_space = re::SPACE_REGEX.replace_all(item, " ").to_string();
        match item_single_space.as_str() {
            "message_Explanation"
            | "message_Narration"
            | "message_Mail"
            | "debug_Print"
            | "back_SetBanner2" => Self::SpecialMessage(item.into()),
            "message_SetFaceEmpty"
            | "message_SetFace"
            | "message_SetFaceOnly"
            | "message_SetActor" => Self::SetActor,
            "message_EmptyActor" | "message_ResetActor" => Self::ResetActor,
            "message_SwitchTalk" | "message_SwitchMonologue" => Self::SwitchTalk,
            "message_Talk" => Self::Talk,
            "case menu" => Self::CaseMenu,
            _ => Self::Unknown,
        }
    }
}
