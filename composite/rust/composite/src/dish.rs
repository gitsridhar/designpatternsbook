use std::{cell::RefCell, rc::Rc};

pub trait Dish {
    fn name(&self) -> &str;
    fn operation(&self, indent: usize) -> String;

    // Optional composite operations with defaults (no-op or not supported)
    fn add(&mut self, _child: Rc<RefCell<dyn Dish>>) {
        // default: Dish - does nothing
    }
    fn remove(&mut self, _name: &str) -> bool {
        false
    }
    fn is_composite(&self) -> bool {
        false
    }
}
