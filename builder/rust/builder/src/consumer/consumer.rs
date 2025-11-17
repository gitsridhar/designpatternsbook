pub trait Consumer {
    fn peeler(&self) -> String;
    fn knife(&self) -> String;
    fn juicer(&self) -> String;
}

pub struct FruitConsumer{}

pub struct VegetableConsumer{}

impl Consumer for FruitConsumer {
    fn peeler(&self) -> String {
        return "Fruit Consumer does not peel".to_string();
    }

    fn knife(&self) -> String {
        return "Use Knife for cutting fruit".to_string();
    }

    fn juicer(&self) -> String {
        return "Use Juicer to make fruit juice".to_string();
    }
}

impl Consumer for VegetableConsumer {
    fn peeler(&self) -> String {
        return "Use Peeler for peeling vegetable".to_string();
    }
    fn knife(&self) -> String {
        return "Use Knife for cutting vegetable".to_string();
    }
    fn juicer(&self) -> String {
        return "Juicer is not used by Vegetable consumer".to_string();
    }
}