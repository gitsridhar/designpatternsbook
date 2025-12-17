use crate::dish::Eatable;

pub struct Soup {
}

impl Eatable for Soup {
    fn prepare(&self) -> String {
        "Eatable : Soup : prepare".into()
    }
}