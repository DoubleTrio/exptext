#[derive(Debug)]
pub enum CType {
    Name,
    Species,
    Unknown,
}

impl From<&str> for CType {
    fn from(item: &str) -> Self {
        match item {
            "name" => CType::Name,
            "kind" => CType::Species,
            _ => CType::Unknown,
        }
    }
}
