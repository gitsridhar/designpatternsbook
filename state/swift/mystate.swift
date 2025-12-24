import Foundation

// State Protocol
protocol OrderPhase {
    // Embedding the context to allow states to trigger transitions
    var order: OrderFood? { get set }
    
    func start()
    func end()
    func deliver()
}

// Context
class OrderFood {
    private var state: OrderPhase
    
    init(initialState: OrderPhase) {
        self.state = initialState
        self.state.order = self
    }
    
    func transitionTo(state: OrderPhase) {
        print("System: Transitioning to \(type(of: state))")
        self.state = state
        self.state.order = self
    }
    
    // Context methods delegate behavior to the current state
    func start() { state.start() }
    func end() { state.end() }
    func deliver() { state.deliver() }
}

// Concrete States

class StartOrder: OrderPhase {
    weak var order: OrderFood?
    
    func start() {
        print("StartOrder: Order is already being prepared.")
    }
    
    func end() {
        print("StartOrder: Finishing preparation...")
        order?.transitionTo(state: EndOrder())
    }
    
    func deliver() {
        print("StartOrder: Cannot deliver. Food is still being cooked.")
    }
}

class EndOrder: OrderPhase {
    weak var order: OrderFood?
    
    func start() {
        print("EndOrder: Order already finished. Cannot restart.")
    }
    
    func end() {
        print("EndOrder: Order is already in the final packing phase.")
    }
    
    func deliver() {
        print("EndOrder: Food is packed and handed to the courier.")
        order?.transitionTo(state: ReadyOrder())
    }
}

class ReadyOrder: OrderPhase {
    weak var order: OrderFood?
    
    func start() {
        print("ReadyOrder: Order was already delivered. Start a new one.")
    }
    
    func end() {
        print("ReadyOrder: Order is already complete.")
    }
    
    func deliver() {
        print("ReadyOrder: Order successfully delivered to the customer!")
    }
}

// Main Execution
let myOrder = OrderFood(initialState: StartOrder())

print("--- Step 1: Prepare ---")
myOrder.start()
myOrder.end()

print("\n--- Step 2: Delivery ---")
myOrder.deliver()

print("\n--- Step 3: Finalize ---")
myOrder.deliver()
