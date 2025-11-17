pub trait Fruit {
    fn consume(&self) -> String;
}

pub struct Apple {
}

pub struct Banana {
}

impl Fruit for Apple {
    fn consume(&self) -> String {
        let msg = "Consuming Apple".to_string();
        println!("Fruit: Apple : consume : {}",msg);
        return msg;
    }
}

impl Fruit for Banana {
    fn consume(&self) -> String {
        let msg = "Consuming Banana".to_string();
        println!("Fruit : Banana : consume : {}",msg);
        return msg;
    }
}