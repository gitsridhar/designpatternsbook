use crate::dish::Eatable;

pub struct SaltAndPepper {
}

impl Eatable for SaltAndPepper {
    fn prepare(&self) -> String {
        "Eatable : SaltAndPepper : prepare".into()
    }
}