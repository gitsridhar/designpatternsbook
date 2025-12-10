class FoodType {
    var cuisine = String()
    var category = String()

    init(cuisine:String, category:String) {
        self.cuisine = cuisine
        self.category = category
    }

    func toString() -> String {
        return cuisine + " : " + category
    }

    func consume() {
        print("Food Type : consume " + category + " " + cuisine)
    }
}

class Food {
    var cost = Int()
    var name = String()
    var foodType = FoodType(cuisine:"", category:"")

    init(cost:Int, name:String, foodType:FoodType) {
        self.cost = cost
        self.name = name
        self.foodType = foodType
    }

    func consume() {
        print("Food " + name + " " + name + " " + foodType.toString())
    }
}

class FoodFactory {
    private static var foodTypes : [String: FoodType] = [:]

    static func GetFoodType(cuisine:String, category:String) -> FoodType {
        let key = cuisine + "-" + category
        if let foodType = foodTypes[key] {
            return foodType
        } else {
            let foodType = FoodType(cuisine:cuisine, category:category)
            foodTypes[key] = foodType
            return foodType
        }
    }
}

class Restaurant {
    var foods : [Food] = []

    func AddFood(cost:Int, name:String, cuisine:String, category:String) {
        let foodType = FoodFactory.GetFoodType(cuisine:cuisine, category:category)
        let food = Food(cost:cost, name:name, foodType:foodType)
        foods.append(food)
    }

    func ServeFood() {
        for food in foods {
            food.consume()
        }
    }
}

@main
class MyFlyweight {
    static func main() {
        let restaurant = Restaurant()
        restaurant.AddFood(cost:10, name:"Pasta", cuisine:"Italian", category:"Main Course");
        restaurant.AddFood(cost:8, name:"Bruschetta", cuisine:"Italian", category:"Appetizer");
        restaurant.AddFood(cost:12, name:"Tiramisu", cuisine:"Italian", category:"Dessert");
        restaurant.AddFood(cost:15, name:"Sushi", cuisine:"Japanese", category:"Main Course");
        restaurant.AddFood(cost:7, name:"Miso Soup", cuisine:"Japanese", category:"Appetizer");
        restaurant.AddFood(cost:9, name:"Mochi", cuisine:"Japanese", category:"Dessert");

        restaurant.ServeFood()
    }
}