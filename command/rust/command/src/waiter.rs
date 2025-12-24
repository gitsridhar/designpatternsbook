use crate::customer::{Action};

pub struct Waiter {
    actions: Vec<Box<dyn Action>>,
}

impl Waiter {
    pub fn new() -> Self {
        Self { actions: Vec::new() }
    }

    pub fn add_action(&mut self, action: Box<dyn Action>) {
        self.actions.push(action);
    }

    pub fn serve_all(&self) {
        for action in &self.actions {
            action.execute();
        }
    }
}