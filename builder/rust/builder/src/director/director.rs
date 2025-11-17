use crate::consumer::consumer::{Consumer};

pub struct Director {
}

impl Director {
    pub fn start_consuming(&self, consumer: &dyn Consumer ) {
        println!("{}", consumer.peeler().to_string());
        println!("{}", consumer.knife().to_string());
        println!("{}", consumer.juicer().to_string());

    }
}