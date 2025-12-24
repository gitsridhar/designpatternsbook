#[derive(Clone, Debug)]
pub struct Dish {
    pub name: String,
    pub stage: String, // e.g., "Prepped", "Cooking", "Garnished"
}

// The Memento for Dish
#[derive(Clone)]
pub struct DishMemento {
    state: String,
}

impl Dish {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            stage: "Raw".to_string(),
        }
    }

    pub fn save(&self) -> DishMemento {
        DishMemento { state: self.stage.clone() }
    }

    pub fn restore(&mut self, memento: &DishMemento) {
        self.stage = memento.state.clone();
    }
}
