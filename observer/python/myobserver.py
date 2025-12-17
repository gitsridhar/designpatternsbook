from abc import ABC, abstractmethod

class Subject(ABC):
    observers = []
    @abstractmethod
    def attach(self, observer):
        pass
    
    @abstractmethod
    def detach(self, observer):
        pass
        
    def notify(self, message):
        for observer in self.observers:
            observer.update(message)
            
class Observer():
    def update(self, message):
        print("Observer received message : " + message)
        
class Chef(Subject):
    def attach(self, observer):
        self.observers.append(observer)

    def detach(self, observer):
        self.observers.remove(observer)

    def prepareDish(self, dishname):
        self.notify("Preparation of dish " + dishname)
        
class Waiter(Observer):
    def __init__(self, chef):
        self.chef = chef
        self.chef.attach(self)
        
    def stopObserving(self):
        self.chef.detach(self)
        
    def update(self, message):
        print("Observer Waiter received message " + message)
        
def main():
    chef = Chef()
    waiter1 = Waiter(chef)
    
    chef.prepareDish(dishname = "TiraMisu")
    
    waiter1.stopObserving()
    
    chef.prepareDish(dishname = "Pasta")

if __name__ == "__main__":
    main()