class Food {
    init(){}
    func dip(){}
}

class Strawberry : Food {
    override func dip() {
        print("Strawberry : dip")
    }
}

class Sauce : Food {

    var food = Food()

    init(food : Food) {
        self.food = food
    }

    override func dip() {
        print("Sauce : dip")
        food.dip()
    }
}

class HotSauce : Sauce {
    override init(food: Food) {
        super.init(food: food)
    }

    override func dip() {
        print("HotSauace : dip")
        super.dip()
    }
}

class ChocolateSauce: Sauce {
    override init(food: Food) {
        super.init(food: food)
    }

    override func dip() {
        print("ChocolateSauce : dip")
        super.dip()
    }
}

@main

class MyDecorator {
    static func main() {
        let strawberry = Strawberry()
        strawberry.dip()

        let strawberrywithhotsauce = HotSauce(food: strawberry)
        strawberrywithhotsauce.dip()

        let strawberrywithchocolatewithhotsauce = ChocolateSauce(food: strawberrywithhotsauce)
        strawberrywithchocolatewithhotsauce.dip()
    }
}