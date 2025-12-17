use crate::dish::Eatable;

pub struct FruitSalad {
}

impl Eatable for FruitSalad {
    fn prepare(&self) -> String {
        "Eatable : FruitSalad : prepare".into()
    }
}