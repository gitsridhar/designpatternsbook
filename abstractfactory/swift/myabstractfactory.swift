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

protocol Vegetable {
    func cookandconsume() -> String
    func serve() -> String
}

struct Potato: Vegetable {
    func cookandconsume() -> String {
        return "Fry and Eat"
    }

    func serve() -> String {
        return "In a bag"
    }
}

struct Beans: Vegetable {
    func cookandconsume() -> String {
        return "Boil and Eat"
    }

    func serve() -> String {
        return "In a bowl"
    }
}

protocol Factory {
    func consumeFruit() -> Fruit
    func consumeVegetable() -> Vegetable
}

struct FruitFactory {
    func consumeFruit() -> Fruit {
        let f = Apple()
        return f
    }

    func consumeVegetable() -> Vegetable {
        let v = Potato()
        return v
    }
}

struct VegetableFactory {
    func consumeFruit() -> Fruit {
        let f = Banana()
        return f
    }

    func consumeVegetable() -> Vegetable {
        let v = Beans()
        return v
    }
}

let f1 = FruitFactory()
let f2 = VegetableFactory()

let a = f1.consumeFruit()
let b = f2.consumeFruit()

print(a.consume())
print(b.consume())

let p = f1.consumeVegetable()
let be = f2.consumeVegetable()

print(p.cookandconsume())
print(p.serve())

print(be.cookandconsume())
print(be.serve())
