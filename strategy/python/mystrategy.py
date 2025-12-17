from abc import ABC, abstractmethod

class StrategyInterface(ABC):
    def performOperation(self, a, b) -> str:
        pass
    
class OpenPanStrategy(StrategyInterface):
    def performOperation(self, a, b) -> str:
        return "Open Pan Strategy " + (a + b)

class ClosedPanStrategy(StrategyInterface):
    def performOperation(self, a, b) -> str:
        return "Closed Pan Strategy " + (a * b)

class Strategy():
    def executeStrategy(self, a, b, strategyinterface) -> str:
        return strategyinterface.performOperation(a, b)
    
class CriticalStrategy(Strategy):
    def executeStrategy(self, a, b, strategyinterface) -> str:
        print("Critica Strategy")
        super().executeStrategy(a, b, strategyinterface)
        
class NonCriticalStrategy(Strategy):
    def executeStrategy(self, a, b, strategyinterface) -> str:
        print("Non-Critical Strategy")
        super().executeStrategy(a, b, strategyinterface)

class FoodPreparation():
    def __init__(self):
        self.strategyinterface = None
    
    def setStrategy(self, strategyinterface):
        self.strategyinterface = strategyinterface
        
    def prepareFood(self):
        if not self.strategyinterface == None:
            a, b = 1, 2
            print(self.strategyinterface.performOperation(a, b))
        else:
            print("Strategy interface not set yet")
            
def main():
    foodPreparation = FoodPreparation()
    foodPreparation.setStrategy(CriticalStrategy())
    foodPreparation.prepareFood()
    
    foodPreparation.setStrategy(NonCriticalStrategy())
    foodPreparation.prepareFood()

if __name__ == "__main__":
    main()
    