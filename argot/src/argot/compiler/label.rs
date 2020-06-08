pub struct LabelGenerator {}

impl LabelGenerator {
    pub fn new() -> Self {
        LabelGenerator {}
    }

    pub fn next(&mut self) -> String {
        String::from("condition")
    }
}

impl Default for LabelGenerator {
    fn default() -> Self {
        Self::new()
    }
}
