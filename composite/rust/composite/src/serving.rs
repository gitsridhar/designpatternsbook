use crate::dish::Eatable;

pub struct Serving {
    pub dishes: Vec<Box<dyn Eatable>>,
}

impl Serving {
    pub fn new() -> Self {
        Serving {
            dishes: Vec::new(),
        }
    }
}

impl Eatable for Serving {
    fn addDish(&self, eatable: Eatable) {
        self.dishes.push(Box::new(eatable))
    }

    fn removeDish(&self, eatable: Eatable) {
        //self.dishes.retain(|dish| dish != dish);
    }
    fn prepare(&self) -> String {
        let retval = "Eatable : Serving : prepare".into();
        for eatable in self.dishes {
            retval += eatable.prepare();
        }
        retval
    }
    fn isComposite(&self) -> bool {
        true
    }
}