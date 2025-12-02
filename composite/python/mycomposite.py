class Dish:
    parentdish = None
    
    def setParent(self,parent):
        self.parentdish = parent
        
    def getParent(self):
        return parentdish
    
    def addDish(self,dish):
        pass
    def removeDish(self,dish):
        pass
    
    def isComposite(self) -> bool:
        return False
    
    def prepare(self) -> str:
        pass
    
class SaltAndPepper(Dish):
    def prepare(self) -> str:
        return "Salt and Pepper"

class FruitSalad(Dish):
    def prepare(self) -> str:
        return "Fruit Salad"
    
class Soup(Dish):
    def prepare(self) -> str:
        return "Soup"

class MainDish(Dish):
    def prepare(self) -> str:
        return "Main Dish"
    
class Serving(Dish):
    def __init__(self):
        self.dishes = []
        
    def addDish(self,dish):
        self.dishes.append(dish)
        dish.setParent(self)
        
    def removeDish(self,dish):
        self.dishes.remove(dish)
        dish.setParent(None)
        
    def isComposite(self) -> bool:
        return True
    
    def prepare(self) -> str:
        result = "Serving contains:\n"
        for dish in self.dishes:
            result += "- " + dish.prepare() + "\n"
        return result
    
def main():
    saltandpepper = SaltAndPepper()
    print(saltandpepper.prepare())
    
    dinner = Serving()
    appetizer = Serving()
    soup = Soup()
    fruit = FruitSalad()
    maindish = MainDish()
    
    appetizer.addDish(soup)
    appetizer.addDish(fruit)
    
    maincourse = Serving()
    maincourse.addDish(maindish)
    
    dinner.addDish(appetizer)
    dinner.addDish(maincourse)
    
    if dinner.isComposite():
        dinner.addDish(saltandpepper)

    print(dinner.prepare())

if __name__ == "__main__":
    main()