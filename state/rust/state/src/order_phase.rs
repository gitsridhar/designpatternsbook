pub trait OrderPhase {
    fn start(self: Box<Self>) -> Box<dyn OrderPhase>;
    fn deliver(self: Box<Self>) -> Box<dyn OrderPhase>;
    fn end(self: Box<Self>) -> Box<dyn OrderPhase>;
}

pub struct OrderFood {
    state: Option<Box<dyn OrderPhase>>,
}

impl OrderFood {
    pub fn new(initial_state: Box<dyn OrderPhase>) -> Self {
        OrderFood { state: Some(initial_state) }
    }

    pub fn start(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.start());
        }
    }

    pub fn deliver(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.deliver());
        }
    }

    pub fn end(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.end());
        }
    }
}
