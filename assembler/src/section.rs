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

#[cfg(test)]
mod tests {
    use super::Section;

    #[test]
    fn parse_section_text() {
        assert_eq!(Section::from("text"), Section::Text);
    }

    #[test]
    fn parse_section_data() {
        assert_eq!(Section::from("data"), Section::Data);
    }

    #[test]
    fn parse_section_unknown() {
        assert_eq!(Section::from("hello"), Section::Unknown);
    }
}
