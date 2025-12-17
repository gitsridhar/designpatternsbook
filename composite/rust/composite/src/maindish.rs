use crate::dish::Eatable;

pub struct MainDish {
}

impl Eatable for MainDish {
    fn prepare(&self) -> String {
        "Eatable : MainDish : prepare".into()
    }
}