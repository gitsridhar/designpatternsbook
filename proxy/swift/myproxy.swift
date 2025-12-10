protocol Burger {
    func Prepare() -> String
}

struct VegBurger : Burger {
    func Prepare() -> String {
        return("VegBurger : prepare")
    }
}

struct BurgerProxy : Burger {
    var burger : Burger

    init(burger : Burger) {
        self.burger = burger
    }

    func Prepare() -> String {
        if TastesGood() && IsHealthy() {
            print(burger.Prepare())
            return "BurgerProxy : prepare"
        } else {
            return "BurgerProxy : Bad Burger"
        }
    }

    func TastesGood() -> Bool {
        return true
    }

    func IsHealthy() -> Bool {
        return true
    }
}

@main
class MyProxy {
    static func main() {
        let vegburger = VegBurger()
        print(vegburger.Prepare())

        let someburger = BurgerProxy(burger: vegburger)
        print(someburger.Prepare())
    }
}