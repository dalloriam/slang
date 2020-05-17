#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Section {
    Text,
    Data,
    Unknown,
}

impl From<&str> for Section {
    fn from(s: &str) -> Section {
        match s {
            "text" => Section::Text,
            "data" => Section::Data,
            _ => Section::Unknown,
        }
    }
}
