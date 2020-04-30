pub trait LabelConverter {
    fn offset_of(&self, label_name: &str) -> Option<u32>;
}
