import Foundation

// 1. Strategy Interface
protocol StrategyInterface {
    func performOperation(a: Int, b: Int)
}

// 2. Concrete StrategyInterface Implementations
class OpenPan: StrategyInterface {
    func performOperation(a: Int, b: Int) {
        print("Cooking in an open pan at \(a)°C for \(b) minutes.")
    }
}

class ClosedPan: StrategyInterface {
    func performOperation(a: Int, b: Int) {
        print("Pressure cooking in a closed pan at \(a)°C for \(b) minutes.")
    }
}

// 3. Strategy Class (Context/Executor)
class Strategy {
    func executeStrategy(a: Int, b: Int, strategy: StrategyInterface) {
        strategy.performOperation(a: a, b: b)
    }
}

// 4. Extensions of Strategy
class OpenStrategy: Strategy {
    override func executeStrategy(a: Int, b: Int, strategy: StrategyInterface) {
        print("Initiating Open Air Method...")
        super.executeStrategy(a: a, b: b, strategy: strategy)
    }
}

class ClosedStrategy: Strategy {
    override func executeStrategy(a: Int, b: Int, strategy: StrategyInterface) {
        print("Initiating Sealed Method...")
        super.executeStrategy(a: a, b: b, strategy: strategy)
    }
}

// 5. Food Preparation (High-level Context)
class FoodPreparation {
    private var strategy: StrategyInterface?

    func setStrategy(strategy: StrategyInterface) {
        self.strategy = strategy
    }

    func prepareFood(a: Int, b: Int, executor: Strategy) {
        guard let strategy = strategy else {
            print("No cooking strategy set!")
            return
        }
        executor.executeStrategy(a: a, b: b, strategy: strategy)
    }
}

// 6. Main Execution
func main() {
    let kitchen = FoodPreparation()
    
    // Scenario 1: Open Pan Cooking
    let openPan = OpenPan()
    let openExec = OpenStrategy()
    
    kitchen.setStrategy(strategy: openPan)
    kitchen.prepareFood(a: 180, b: 20, executor: openExec)
    
    print("---------------------------")
    
    // Scenario 2: Closed Pan Cooking
    let closedPan = ClosedPan()
    let closedExec = ClosedStrategy()
    
    kitchen.setStrategy(strategy: closedPan)
    kitchen.prepareFood(a: 120, b: 45, executor: closedExec)
}

main()
