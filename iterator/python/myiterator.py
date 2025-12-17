from abc import ABC, abstractmethod

class Dinner(ABC):
    def createDinner(self):
        return None

class Dish():
    def __init__(self, name):
        self.name = name
    
    def getName(self):
        return self.name
    
class Eating():
    def eat(self, dish: Dish):
        print("Eating : " + dish.getName())
        
    def hasNextDish(self):
        return False
    
    def nextDish(self) -> Dish:
        return None
    
class RestaurantEating(Eating):
    def __init__(self, dishes: list):
        self.dishes = dishes
        self.currentindex = 0

    def hasNextDish(self):
        if self.currentindex < len(self.dishes):
            return True
        else:
            return False
    
    def nextDish(self):
        if self.hasNextDish():
            self.currentindex += 1
            return self.dishes[self.currentindex-1]
        else:
            return None

class WeekendDinner(Dinner):
    def __init__(self, dishes: list):
        self.dishes = dishes

    def createDinner(self):
        return RestaurantEating(self.dishes)
    
def main():
    dish1 = Dish(name = "pasta")
    dish2 = Dish(name = "salad")
    
    dishes = [dish1, dish2]
    
    weekendDinner = WeekendDinner(dishes)
    eating = weekendDinner.createDinner()
    
    while(eating.hasNextDish()):
        dish = eating.nextDish()
        eating.eat(dish)

if __name__ == "__main__":
    main()