struct Fruit {
    var utensils: [String] = []
    func showUtensils() {
        for utensil in utensils {
            print("     ", utensil)
        }
    }
}

protocol Consumer {
    mutating func Peeler()
    mutating func Knife()
}

struct FruitConsumer: Consumer {
    var fruit = Fruit()
    mutating func Peeler() {
        self.fruit.utensils.append("Peeler Not Required")
    }

    mutating func Knife() {
        self.fruit.utensils.append("Knife")
    }

    func showUtensils() {
        print("Fruit Utensils:")
        self.fruit.showUtensils()
    }
}

struct VegetableConsumer: Consumer {
    var fruit = Fruit()
    mutating func Peeler() {
        self.fruit.utensils.append("Peeler")
    }

    mutating func Knife() {
        self.fruit.utensils.append("Knife")
    }

    func showUtensils() {
        print("Vegetable Utensils:")
        self.fruit.showUtensils()
    }
}

var fc = FruitConsumer()
fc.Peeler()
fc.Knife()

fc.showUtensils()

var vc = VegetableConsumer()
vc.Peeler()
vc.Knife()

vc.showUtensils()