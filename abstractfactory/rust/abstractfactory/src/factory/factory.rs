pub mod factory {
    use crate::fruit::fruit::Fruit; 
    use crate::fruit::fruit::{Apple, Banana};

    use crate::vegetable::vegetable::Vegetable; 
    use crate::vegetable::vegetable::{Potato, Beans};

    pub trait Factory {
        fn consume_fruit(&self) -> Box<dyn Fruit>;
        fn consume_vegetable(&self) -> Box<dyn Vegetable>;
    }

    pub struct FirstFactory {
    }

    pub struct SecondFactory {
    }

    impl Factory for FirstFactory {
        fn consume_fruit(&self) -> Box<dyn Fruit> {
            let apple = Apple{};
            let msg = apple.consume();
            println!("Factory : consume_fruit : Apple : {}", msg);
            return Box::new(apple);
        }

        fn consume_vegetable(&self) -> Box<dyn Vegetable> {
            let potato = Potato{};
            let msg = potato.cookandconsume();
            println!("Factory : consume_vegetable : Potato : {}", msg);
            return Box::new(potato);
        }
    }
    impl Factory for SecondFactory {
        fn consume_fruit(&self) -> Box<dyn Fruit> {
            let banana = Banana{};
            let msg = banana.consume();
            println!("Factory : consume_fruit : Banana : {}", msg);
            return Box::new(banana);
        }

        fn consume_vegetable(&self) -> Box<dyn Vegetable> {
            let beans = Beans{};
            let msg = beans.cookandconsume();
            println!("Factory : consume_vegetable : Beans : {}", msg);
            return Box::new(beans);
        }
    }
}