// subject.rs
use crate::observer::Observer;

pub trait Subject {
    fn attach(&mut self, observer: Box<dyn Observer>);
    fn notify(&self, dish_name: &str);
}

pub struct Chef {
    observers: Vec<Box<dyn Observer>>,
}

impl Chef {
    pub fn new() -> Self {
        Self { observers: Vec::new() }
    }

    pub fn prepare_dish(&self, dish_name: &str) {
        println!("Chef: Preparing {}...", dish_name);
        self.notify(dish_name);
    }
}

impl Subject for Chef {
    fn attach(&mut self, observer: Box<dyn Observer>) {
        self.observers.push(observer);
    }

    fn notify(&self, dish_name: &str) {
        for observer in &self.observers {
            observer.update(dish_name);
        }
    }
}
