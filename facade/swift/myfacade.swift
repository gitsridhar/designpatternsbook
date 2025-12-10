class ColdFood {
    func WashAndRinse() -> String {
        return("ColdFood : Wash and Rinse")
    }

    func Wrap() -> String {
        return("ColdFood ; Wrap")
    }

    func Freeze() -> String {
        return("ColdFood : Freeze")
    }
}

class HotFood {
    func UnWrap() -> String {
        return("HotFood : Unwrap")
    }

    func Clean() -> String {
        return("HotFood : Clean")
    }

    func Cook() -> String {
        return("HotFood : Cook")
    }
}

class Restaurant {
    var coldFood = ColdFood()
    var hotFood = HotFood()

    init(coldFood: ColdFood, hotFood: HotFood) {
        self.coldFood = coldFood
        self.hotFood = hotFood
    }

    func Operation() -> String {
        var result = String()

        result += coldFood.WashAndRinse()
        result += coldFood.Wrap()
        result += coldFood.Freeze()

        result += hotFood.UnWrap()
        result += hotFood.Clean()
        result += hotFood.Cook()

        return(result)
    }
}

@main
class MyFacade {
    static func main() {
        let coldFood = ColdFood()
        let hotFood = HotFood()

        let restaurant = Restaurant(coldFood: coldFood, hotFood: hotFood)
        print(restaurant.Operation())
    }
}