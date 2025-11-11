from abc import ABC, abstractmethod

class Fruit(ABC):
    def consume(self) -> str:
        pass
    
class Apple(Fruit):
    def consume(self) -> str:
        return  "Apple: Cut and Eat"
    
class Banana(Fruit):
    def consume(self) -> str:
        return "Banana: Peel and Eat"
    
class Vegetable(ABC):
    def cookAndConsume(self) -> str:
        pass
    def serve(self) -> str:
        pass
    
class Potato(Vegetable):
    def cookAndConsume(self) -> str:
        return "Potato: Fry and Eat"
    def serve(self) -> str:
        return "Potato: In a bag"
    
class Beans(Vegetable):
    def cookAndConsume(self) -> str:
        return "Beans: Boil and Eat"
    def serve(self) -> str:
        return "Beans: In a bowl"

class MyFactory(ABC):
    def consumeFruit(self) -> str:
        pass
    def consumeVegetable(self) -> str:
        pass

class MyFactory1(MyFactory):
    def consumeFruit(self) -> str:
        fruit = Apple()
        return fruit.consume()
    def consumeVegetable(self) -> str:
        vegetable = Potato()
        return vegetable.cookAndConsume()
    
class MyFactory2(MyFactory):
    def consumeFruit(self) -> str:
        fruit = Banana()
        return fruit.consume()
    def consumeVegetable(self) -> str:
        vegetable = Beans()
        return vegetable.cookAndConsume()

def main():
    fact1 = MyFactory1()
    print(fact1.consumeFruit())
    print(fact1.consumeVegetable())
    
    fact2 = MyFactory2()
    print(fact2.consumeFruit())
    print(fact2.consumeVegetable())
    
if __name__ == "__main__":
    main()