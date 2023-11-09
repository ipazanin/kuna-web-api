pub trait ConfigSection {
    fn set_from_environment(&mut self, name: &str, value: &str);
}
