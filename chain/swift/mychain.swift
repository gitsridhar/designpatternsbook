import Foundation

// The Chef protocol defining the Chain of Responsibility interface
protocol Chef: AnyObject {
    var nextChef: Chef? { get set }
    func setNext(chef: Chef) -> Chef
    func cook(dish: String)
}

// Extension to provide default implementation for linking
extension Chef {
    func setNext(chef: Chef) -> Chef {
        self.nextChef = chef
        return chef
    }
}

// 1. Collecting Ingredients Chef
class CollectingIngredientsChef: Chef {
    var nextChef: Chef?
    
    func cook(dish: String) {
        print("CollectingIngredientsChef: Gathering fresh ingredients for \(dish).")
        nextChef?.cook(dish: dish)
    }
}

// 2. Boiling Chef
class BoilingChef: Chef {
    var nextChef: Chef?
    
    func cook(dish: String) {
        if dish.lowercased().contains("soup") || dish.lowercased().contains("pasta") {
            print("BoilingChef: Boiling water and preparing the base for \(dish).")
        }
        nextChef?.cook(dish: dish)
    }
}

// 3. Frying Chef
class FryingChef: Chef {
    var nextChef: Chef?
    
    func cook(dish: String) {
        if dish.lowercased().contains("fries") || dish.lowercased().contains("steak") {
            print("FryingChef: Searing and frying components for \(dish).")
        }
        nextChef?.cook(dish: dish)
    }
}

// 4. Basic Chef
class BasicChef: Chef {
    var nextChef: Chef?
    
    func cook(dish: String) {
        print("BasicChef: Plating and garnishing \(dish).")
        nextChef?.cook(dish: dish)
    }
}

// 5. Master Chef (The Final Authority)
class MasterChef: Chef {
    var nextChef: Chef?
    
    func cook(dish: String) {
        print("MasterChef: Final quality check. \(dish) is ready for the guest!\n")
    }
}

// Implementation in "Main"
let ingredientsChef = CollectingIngredientsChef()
let boilingChef = BoilingChef()
let fryingChef = FryingChef()
let basicChef = BasicChef()
let masterChef = MasterChef()

// Setting up the chain: Ingredients -> Boiling -> Frying -> Basic -> Master
ingredientsChef
    .setNext(chef: boilingChef)
    .setNext(chef: fryingChef)
    .setNext(chef: basicChef)
    .setNext(chef: masterChef)

// Execute the chain
print("--- Ordering Tomato Soup ---")
ingredientsChef.cook(dish: "Tomato Soup")

print("--- Ordering Fried Steak ---")
ingredientsChef.cook(dish: "Fried Steak")
