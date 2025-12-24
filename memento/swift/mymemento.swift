import Foundation

// Memento
// The Memento stores the internal state of the Dish.
struct DishMemento {
    let state: String
}

// Originator
// The Dish has an internal state that needs to be saved/restored.
class Dish {
    private var state: String
    let name: String

    init(name: String, state: String) {
        self.name = name
        self.state = state
    }

    func updateState(newState: String) {
        print("Dish [\(name)]: Changing state from '\(state)' to '\(newState)'")
        state = newState
    }

    func save() -> DishMemento {
        return DishMemento(state: state)
    }

    func restore(memento: DishMemento) {
        self.state = memento.state
        print("Dish [\(name)]: Restored to state '\(state)'")
    }
}

// Caretaker Component
// The Waiter manages its own state and holds a list of dish mementos.
class Waiter {
    private var status: String
    private var history: [DishMemento] = []

    init(status: String) {
        self.status = status
    }

    func setStatus(_ status: String) {
        self.status = status
    }

    func addMemento(_ memento: DishMemento) {
        history.append(memento)
    }

    func getLastMemento() -> DishMemento? {
        return history.popLast()
    }
}

// Facade/Controller
// Chef manages the dishes and uses the Waiter to handle the history.
class Chef {
    private var dishes: [Dish]
    private var waiter: Waiter

    init(dishes: [Dish], waiter: Waiter) {
        self.dishes = dishes
        self.waiter = waiter
    }

    func backup() {
        print("\n--- Chef: Backing up all dish states ---")
        for dish in dishes {
            waiter.addMemento(dish.save())
        }
    }

    func undo() {
        print("\n--- Chef: Undoing last changes ---")
        // Restoring in reverse order of how they were saved
        for dish in dishes.reversed() {
            if let memento = waiter.getLastMemento() {
                dish.restore(memento: memento)
            }
        }
    }
}

// Main Execution
let steak = Dish(name: "Steak", state: "Raw")
let pasta = Dish(name: "Pasta", state: "Dry")
let waiter = Waiter(status: "Ready")

let chef = Chef(dishes: [steak, pasta], waiter: waiter)

// Initial Backup
chef.backup()

// Change States
steak.updateState(newState: "Medium Rare")
pasta.updateState(newState: "Al Dente")

// Undo Changes
chef.undo()
