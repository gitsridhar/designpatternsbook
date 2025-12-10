
pub trait Processor {
    fn process_food(&self) -> String;
}

pub struct FoodProcessor;

impl Processor for FoodProcessor {
    fn process_food(&self) -> String {
        return "Processor : processFood".into()
    }
}