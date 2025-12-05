    class Dish {
        var parent: Dish? = nil

        init() {
            print("Dish init")
        }

        func setParent(parent: Dish?) {
            self.parent = parent
        }

        func getParent() -> Dish {
            return parent!
        }

        func Add(dish: Dish) {
        }

        func Remove(dish: Dish) {
        }

        func IsComposite() -> Bool {
            return false;
        }

        func prepare() -> String {
            return("")
        }
    }

    class SaltAndPepper : Dish {
        override init() {
            print("init")
            super.init()
        }
        override func prepare() -> String {
            return("Salt and Pepper prepared and served")
        }
    }

    class FruitSalad : Dish {
        override func prepare() -> String {
            return("Fruid salad prepared and served")
        }
    }

    class Soup : Dish {
        override func prepare() -> String {
            return("Soup prepared and served")
        }
    }

    class MainDish : Dish {
        override func prepare() -> String {
            return("Maindish prepared and served")
        }
    }

    class Serving: Dish {
        var dishes: [Dish] = []

        override func Add(dish: Dish) {
            dishes.append(dish)
            dish.setParent(parent:self)
        }

        override func Remove(dish: Dish) {
            if let i = dishes.firstIndex(where: {$0 === dish}) {
                dishes.remove(at: i)
            }
            dish.setParent(parent:nil)
        }

        override func IsComposite() -> Bool {
            return true
        }

        override func prepare() -> String {
            var result = ""
            if IsComposite() {
                for dish in dishes {
                    result += dish.prepare() + "\n"
                }
            }
            return(result)
        }
    }

    @main
    struct MyComposite {
        static func main() {
            let saltandpepper = SaltAndPepper()
            print(saltandpepper.prepare())

            let dinner = Serving()
            let appetizer = Serving()

            let soup = Soup()
            let fruitsalad = FruitSalad()
            let maindish = MainDish()

            if appetizer.IsComposite() {
                appetizer.Add(dish:soup)
                appetizer.Add(dish:fruitsalad)
            }

            let maincourse = Serving()
            maincourse.Add(dish:maindish)

            dinner.Add(dish:saltandpepper)
            dinner.Add(dish:appetizer)
            dinner.Add(dish:maincourse)

            print(dinner.prepare())
        }
    }