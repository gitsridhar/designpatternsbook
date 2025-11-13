pub mod consumer {

    pub trait Consumer {
        fn consume(&self) -> String;
    }

    pub struct AppleConsumer {
    }

    pub struct BananaConsumer {
    }

    impl Consumer for AppleConsumer {
        fn consume(&self) -> String {
           return "Consuming Apple".to_string();
        }
    }

    impl Consumer for BananaConsumer {
        fn consume(&self) -> String {
            return "Consuming Banana".to_string();
        }
    }
}
