pub trait Vegetable {
    fn cookandconsume(&self) -> String;
    fn serve(&self);
}

pub struct Potato {
}

pub struct Beans {
}

impl Vegetable for Potato {
    fn cookandconsume(&self) -> String {
        let msg = "Potato: Cook and Consume".to_string();
        println!("Vegetable: Cookandconsume : {}", msg);
        return msg;
    }

    fn serve(&self) {
        let msg = "Potato : Serving".to_string();
        println!("Vegetable : Serve : {}", msg);
    }
}

impl Vegetable for Beans {
    fn cookandconsume(&self) -> String {
        let msg = "Beans: Cook and Consume".to_string();
        println!("Vegetable: Cookandconsume : {}", msg);
        return msg;
    }

    fn serve(&self) {
        let msg = "Beans: Serving".to_string();
        println!("Vegetable: Serve : {}", msg);
    }
}