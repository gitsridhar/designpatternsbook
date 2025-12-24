import Foundation

// Observer Pattern Protocols

protocol Observer: AnyObject {
    var id: UUID { get }
    func update(message: String)
}

protocol Subject {
    var observers: Set<AnyObserverWrapper> { get set }
    func register(observer: Observer)
    func remove(observer: Observer)
    func notifyObservers(message: String)
}

// Wrapper to allow Set to store class-based Observers via Hashable
struct AnyObserverWrapper: Hashable {
    weak var observer: Observer?
    let id: UUID

    init(_ observer: Observer) {
        self.observer = observer
        self.id = observer.id
    }

    func hash(into hasher: inout Hasher) { hasher.combine(id) }
    static func == (lhs: AnyObserverWrapper, rhs: AnyObserverWrapper) -> Bool { lhs.id == rhs.id }
}

// Concrete Subject

class Chef: Subject {
    var observers = Set<AnyObserverWrapper>()
    private var lastOrder: String = ""

    func register(observer: Observer) {
        observers.insert(AnyObserverWrapper(observer))
    }

    func remove(observer: Observer) {
        observers.remove(AnyObserverWrapper(observer))
    }

    func notifyObservers(message: String) {
        observers.forEach { $0.observer?.update(message: message) }
    }

    func prepareOrder(_ dish: String) {
        print("Chef: Preparing \(dish)...")
        self.lastOrder = dish
        notifyObservers(message: "Order for \(dish) is ready!")
    }
}

// Concrete Observer

class Waiter: Observer {
    let id = UUID()
    let name: String
    private weak var chef: Chef?

    init(name: String, chef: Chef) {
        self.name = name
        self.chef = chef
        // Automatically register with the chef upon initialization
        chef.register(observer: self)
    }

    func update(message: String) {
        print("Waiter \(name) received notification: \(message)")
        serveOrder()
    }

    private func serveOrder() {
        print("Waiter \(name) is serving the dish to the table.")
    }
}

// Main Execution

let headChef = Chef()

let waiter1 = Waiter(name: "Alice", chef: headChef)
let waiter2 = Waiter(name: "Bob", chef: headChef)

print("--- Restaurant Service Started ---")
headChef.prepareOrder("Pasta Carbonara")

print("\n--- Bob leaves for break ---")
headChef.remove(observer: waiter2)

headChef.prepareOrder("Margherita Pizza")
