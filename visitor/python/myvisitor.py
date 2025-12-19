class Visitor:
    def drink(self, restauant):
        pass

class Visitor1(Visitor):
    def drink(self, restaurant):
        
        restaurant.serve_drink(self)
        restaurant.take_payment(self)
        
        return "Visitor1 is drinking at " + restaurant.name
    
class Visitor2(Visitor):
    def drink(self, restaurant):
        
        restaurant.serve_drink(self)
        restaurant.take_payment(self)
        
        return "Visitor2 is drinking at " + restaurant.name

class Restaurant:
    def __init__(self, name):
        self.name = name

    def accept(self, visitor):
        pass

    def serve_drink(self, visitor):
        pass

    def take_payment(self, visitor):
        pass

class RestaurantA(Restaurant):
    def serve_drink(self, visitor):
        print(f"RestaurantA serves a drink to {visitor.__class__.__name__}")

    def take_payment(self, visitor):
        print(f"RestaurantA takes payment from {visitor.__class__.__name__}")
    
    def accept(self, visitor):
        return visitor.drink(self)
        
class RestaurantB(Restaurant):
    def serve_drink(self, visitor):
        print(f"RestaurantB serves a drink to {visitor.__class__.__name__}")

    def take_payment(self, visitor):
        print(f"RestaurantB takes payment from {visitor.__class__.__name__}")
        
    def accept(self, visitor):
        return visitor.drink(self)


def main():
    visitor1 = Visitor1()
    visitor2 = Visitor2()
    
    restaurant_a = RestaurantA("RestaurantA")
    restaurant_b = RestaurantB("RestaurantB")
    
    print(restaurant_a.accept(visitor1))
    print(restaurant_a.serve_drink(visitor1))
    print(restaurant_a.take_payment(visitor1))
    
    print(restaurant_a.accept(visitor2))
    print(restaurant_a.serve_drink(visitor2))
    print(restaurant_a.take_payment(visitor2))
    
    print(restaurant_b.accept(visitor1))
    print(restaurant_b.serve_drink(visitor1))
    print(restaurant_b.take_payment(visitor1))
    
    print(restaurant_b.accept(visitor2))
    print(restaurant_b.serve_drink(visitor2))
    print(restaurant_b.take_payment(visitor2)) 


if __name__ == "__main__":
    main()