use std::{cell::RefCell, rc::Rc};
use crate::dish::Dish;

pub struct Serving {
    name: String,
    children: Vec<Rc<RefCell<dyn Dish>>>,
}

impl Serving {
    pub fn new(name: impl Into<String>) -> Self {
        Serving {
            name: name.into(),
            children: Vec::new(),
        }
    }
}

impl Dish for Serving {
    fn name(&self) -> &str {
        &self.name
    }

    fn operation(&self, indent: usize) -> String {
        let mut out = format!("{}+ Serving: {}\n", " ".repeat(indent), self.name);
        for c in &self.children {
            out.push_str(&c.borrow().operation(indent + 2));
        }
        out
    }

    fn add(&mut self, child: Rc<RefCell<dyn Dish>>) {
        self.children.push(child);
    }

    fn remove(&mut self, name: &str) -> bool {
        if let Some(pos) = self
            .children
            .iter()
            .position(|c| c.borrow().name() == name)
        {
            self.children.remove(pos);
            return true;
        }
        // Try to remove recursively from children that are composites
        for c in &self.children {
            if c.borrow().is_composite() {
                if c.borrow_mut().remove(name) {
                    return true;
                }
            }
        }
        false
    }

    fn is_composite(&self) -> bool {
        true
    }
}
