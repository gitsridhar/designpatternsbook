use crate::order_phase::OrderPhase;

pub struct StartOrderPhase;
pub struct ReadyOrderPhase;
pub struct EndOrderPhase;

impl OrderPhase for StartOrderPhase {
    fn start(self: Box<Self>) -> Box<dyn OrderPhase> {
        println!("Order started! Preparing food...");
        Box::new(ReadyOrderPhase)
    }
    fn deliver(self: Box<Self>) -> Box<dyn OrderPhase> { self }
    fn end(self: Box<Self>) -> Box<dyn OrderPhase> { self }
}

impl OrderPhase for ReadyOrderPhase {
    fn start(self: Box<Self>) -> Box<dyn OrderPhase> { self }
    fn deliver(self: Box<Self>) -> Box<dyn OrderPhase> {
        println!("Food is ready and delivered!");
        Box::new(EndOrderPhase)
    }
    fn end(self: Box<Self>) -> Box<dyn OrderPhase> { self }
}

impl OrderPhase for EndOrderPhase {
    fn start(self: Box<Self>) -> Box<dyn OrderPhase> { self }
    fn deliver(self: Box<Self>) -> Box<dyn OrderPhase> { self }
    fn end(self: Box<Self>) -> Box<dyn OrderPhase> {
        println!("Order finished. Thank you!");
        self // Final state
    }
}
