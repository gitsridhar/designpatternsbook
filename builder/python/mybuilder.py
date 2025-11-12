from abc import ABC, abstractmethod

class Fruit(ABC):
    def __init__(self):
        self.utensils = []
    def getUtensils(self):
        for utensil in self.utensils:
            print(utensil)
    
class Consumer(ABC):
    def peeler(self):
        pass
    def knife(self):
        pass
    
class FruitConsumer(Consumer):
    def __init__(self):
        self.fruit = Fruit()
    def peeler(self):
        self.fruit.utensils.append("Peeler")
    def knife(self):
        self.fruit.utensils.append("Knife")

def main():
    fc = FruitConsumer()
    fc.peeler()
    fc.knife()
    print(fc.fruit.getUtensils())
    
if __name__ == "__main__":
    main()