use std::iter::Cycle;
use std::str::Chars;

const ASCII_LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
pub struct LabelGenerator {
    char_it: Cycle<Chars<'static>>,

    current_count: usize,
    sub_endless: Option<Box<LabelGenerator>>,
    prefix: String,
}

impl LabelGenerator {
    pub fn new() -> LabelGenerator {
        LabelGenerator {
            char_it: ASCII_LOWERCASE.chars().cycle(),
            current_count: 0,
            sub_endless: None,
            prefix: String::new(),
        }
    }
}

impl Iterator for LabelGenerator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_count != 0 && self.current_count % ASCII_LOWERCASE.len() == 0 {
            let prefix = if let Some(x) = self.sub_endless.as_mut() {
                x.next().unwrap()
            } else {
                let mut gen = Box::new(LabelGenerator::new());
                let prefix = gen.next().unwrap();
                self.sub_endless = Some(gen);
                prefix
            };

            self.prefix = prefix;
        }

        self.current_count += 1;
        Some(format!("{}{}", self.prefix, self.char_it.next().unwrap()))
    }
}

#[cfg(test)]
mod tests {
    use super::LabelGenerator;

    #[test]
    fn test_label_generator() {
        let mut lbl = LabelGenerator::new();
        assert_eq!(lbl.next().unwrap(), String::from("a"));
    }

    #[test]
    fn test_roll_around() {
        let mut lbl = LabelGenerator::new();
        let mut last = String::new();
        for _i in 0..55 {
            last = lbl.next().unwrap();
        }
        assert_eq!(last, String::from("bc"));
    }
}
