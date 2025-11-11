from abc import ABC, abstractmethod
class Fruit(ABC):
    def consume(self) -> str:
        pass
    
class Apple(Fruit):
    def consume(self) -> str:
        return "Cut and Eat."

class Banana(Fruit):
    def consume(self) -> str:
        return "Peel and Eat."
    
class Consumer(ABC):
    def getFruit(self) -> Fruit:
        pass
    def consumeFruit(self) -> str:
        pass
    
class AppleConsumer(Consumer):
    def getFruit(self) -> Fruit:
        return Apple()
    
    def consumeFruit(self) -> str:
        return self.getFruit().consume()
    
class BananaConsumer(Consumer):
    def getFruit(self) -> Fruit:
        return Banana()
    
    def consumeFruit(self) -> str:
        return self.getFruit().consume()
    
def main():
    appleconsumer = AppleConsumer()
    print(appleconsumer.consumeFruit())
    
    bananaconsumer = BananaConsumer()
    print(bananaconsumer.consumeFruit())
    
if __name__ == "__main__":
    main()