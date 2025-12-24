#[derive(Debug)]
pub struct Waiter {
    pub status: String, // e.g., "Idle", "Serving", "Cleaning"
}

pub struct WaiterMemento {
    state: String,
}

impl Waiter {
    pub fn new() -> Self {
        Self { status: "Idle".to_string() }
    }

    pub fn save(&self) -> WaiterMemento {
        WaiterMemento { state: self.status.clone() }
    }

    pub fn restore(&mut self, memento: &WaiterMemento) {
        self.status = memento.state.clone();
    }
}
