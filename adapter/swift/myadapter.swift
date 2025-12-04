protocol Chopper {
    func chop() -> String
}

class FoodChopper: Chopper {
    func chop() -> String {
        return "Choppping Food."
    } 
}

protocol Processor {
    func processFood() -> String
}

class FoodProcessor: Processor {
    func processFood() -> String {
        return "Processing Food"
    }
}

struct NewFoodProcessor: Chopper {
    var processor = FoodProcessor()

    init(processor: FoodProcessor) {
        self.processor = processor
    }

    func chop() -> String {
        return "New Food Processor chopping food"
    }
}

@main
struct MyAdapter {
    static func main() {
        let foodProcessor = FoodProcessor()
        print(foodProcessor.processFood())

        let chopper = NewFoodProcessor(processor: foodProcessor)
        print(chopper.chop())
    }
}

// swiftc -parse-as-library myadapter.swift
// ./myadapter