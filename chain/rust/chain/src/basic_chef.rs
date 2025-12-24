use crate::chef::Chef;

pub struct BasicChef {
    pub next: Option<Box<dyn Chef>>,
}

impl BasicChef {
    pub fn new() -> Self {
        Self { next: None }
    }
}
