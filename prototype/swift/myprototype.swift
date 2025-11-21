protocol Prototype {
    func clone() -> Prototype
}

struct FruitPrototype: Prototype {

    func clone() -> Prototype {
        return FruitPrototype()
    }
}

struct VegetablePrototype: Prototype {

    func clone() -> Prototype {
        return VegetablePrototype()
    }
}

struct PrototypeFactory {
    static func getPrototype(type: String) -> Prototype? {
        if type == "Fruit" {
            return FruitPrototype()
        } else if type == "Vegetable" {
            return VegetablePrototype()
        }
        return nil
    }
}   
var fruitPrototype = PrototypeFactory.getPrototype(type: "Fruit")
var clonedFruitPrototype = fruitPrototype?.clone()
print("Cloned Fruit Prototype: \(clonedFruitPrototype!)")

var vegetablePrototype = PrototypeFactory.getPrototype(type: "Vegetable")
var clonedVegetablePrototype = vegetablePrototype?.clone()
print("Cloned Vegetable Prototype: \(clonedVegetablePrototype!)")   


