#[derive(Copy, Debug, PartialEq)]
pub enum Section {
    Code,
    Unknown,
}

impl From<&str> for Section {
    fn from(s: &str) -> Section {
        match s {
            "code" => Section::Code,
            _ => Section::Unknown,
        }
    }
}
