pub trait Eatable {

    fn addDish(&self, dish: Dish) {}

    fn removeDish(&self, dish: Dish) {}

    fn isComposite(&self) -> bool {
        false
    }

    fn prepare(&self) -> String {
        "".to_string()
    }
}
