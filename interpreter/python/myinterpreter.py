class Item:
    def interpret(self, foodorder):
        pass

class FoodItem(Item):
    def __init__(self, name, type):
        self.name = name
        self.type = type

    def get_name(self):
        return self.name

    def get_type(self):
        return self.type
    
    def interpret(self, foodorder):
        if foodorder.get_type().lower() == "food":
            return f"Food Item: {foodorder.get_name()}"
        return None

class DrinkItem(Item):
    def __init__(self, name, size):
        self.name = name
        self.size = size

    def get_name(self):
        return self.name

    def get_size(self):
        return self.size

    def interpret(self, foodorder):
        if foodorder.get_type().lower() == "drink":
            return f"Drink Item: {foodorder.get_name()}"
        return None
    
class FoodOrder:
    def __init__(self, name, type):
        self.name = name
        self.type = type

    def get_name(self):
        return self.name

    def get_type(self):
        return self.type

class Interpreter:
    def __init__(self):
        self.items = []

    def add_item(self, item):
        self.items.append(item)

    def interpret(self, foodorder):
        for item in self.items:
            result = item.interpret(foodorder)
            if result is not None:
                return result
        return "Item not found in the order."

if __name__ == "__main__":  
    interpreter = Interpreter()
    interpreter.add_item(FoodItem("Burger", "food"))
    interpreter.add_item(DrinkItem("Coke", "large"))

    order1 = FoodOrder("Burger", "food")
    order2 = FoodOrder("Coke", "drink")
    order3 = FoodOrder("Fries", "food")

    print(interpreter.interpret(order1))  # Output: Food Item: Burger
    print(interpreter.interpret(order2))  # Output: Drink Item: Coke
    print(interpreter.interpret(order3))  # Output: Item not found in the order.
