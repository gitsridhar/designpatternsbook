pub trait Chef {
    fn set_next(&mut self, next: Box<dyn Chef>);
    fn cook(&self, dish: &str);
}
