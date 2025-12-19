
from abc import ABC, abstractmethod

class StrategyInterface(ABC):
    def performOperation(self, a, b) -> str:
        pass
    
class OpenPanStrategy(StrategyInterface):
    def performOperation(self, a, b) -> str:
        retval = "Open Pan Strategy " + str(a + b)
        return retval

class ClosedPanStrategy(StrategyInterface):
    def performOperation(self, a, b) -> str:
        retval = "Closed Pan Strategy " + str(a * b)
        return retval

class Strategy():
    def executeStrategy(self, a, b, strategyinterface) -> str:
        return strategyinterface.performOperation(a, b)
    
class CriticalStrategy(Strategy):
    def executeStrategy(self, a, b, strategyinterface) -> str:
        print("Critica Strategy")
        return(super().executeStrategy(a, b, strategyinterface))
        
class NonCriticalStrategy(Strategy):
    def executeStrategy(self, a, b, strategyinterface) -> str:
        print("Non-Critical Strategy")
        return(super().executeStrategy(a, b, strategyinterface))

class FoodPreparation():
    def __init__(self):
        self.strategyinterface = None
    
    def setStrategy(self, strategyinterface):
        self.strategyinterface = strategyinterface
        
    def prepareFood(self):
        if not self.strategyinterface == None:
            if self.strategyinterface.__class__ == CriticalStrategy:
                a, b = 1, 2
                print(self.strategyinterface.executeStrategy(a, b, OpenPanStrategy()))
            elif self.strategyinterface.__class__ == NonCriticalStrategy:
                a, b = 3, 4
                print(self.strategyinterface.executeStrategy(a, b, ClosedPanStrategy()))    
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
