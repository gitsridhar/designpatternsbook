use crate::dish::{Dish, DishMemento};
use crate::waiter::{Waiter, WaiterMemento};

pub struct Chef {
    dish_history: Vec<DishMemento>,
    waiter_history: Vec<WaiterMemento>,
}

impl Chef {
    pub fn new() -> Self {
        Self {
            dish_history: Vec::new(),
            waiter_history: Vec::new(),
        }
    }

    // Backup current state
    pub fn backup(&mut self, dish: &Dish, waiter: &Waiter) {
        self.dish_history.push(dish.save());
        self.waiter_history.push(waiter.save());
    }

    // Undo to previous state
    pub fn undo(&mut self, dish: &mut Dish, waiter: &mut Waiter) {
        if let Some(memento) = self.dish_history.pop() {
            dish.restore(&memento);
        }
        if let Some(memento) = self.waiter_history.pop() {
            waiter.restore(&memento);
        }
    }
}
