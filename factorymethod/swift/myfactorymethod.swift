protocol Fruit {
    func consume() -> String
}

struct Apple: Fruit {
    func consume() -> String {
        return "Cut and Eat."
    }
}

struct Banana: Fruit {
    func consume() -> String {
        return "Peel and Eat."
    }
}

protocol Consumer {
    func consume() -> String
    func getFruit() -> Fruit
}

struct AppleConsumer: Consumer {
    func getFruit() -> Fruit {
        let apple = Apple()
        return apple
    }

    func consume() -> String {
        let f = getFruit()
        return f.consume()
    }
}

struct BananaConsumer: Consumer {
    func getFruit() -> Fruit {
        let apple = Banana()
        return apple
    }

    func consume() -> String {
        let f = getFruit()
        return f.consume()
    }
}

let appleconsumer = AppleConsumer()
print(appleconsumer.consume())

let bananaconsumer = BananaConsumer()
print(bananaconsumer.consume())