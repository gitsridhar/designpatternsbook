from abc import ABC, abstractmethod

class Chef(ABC):
    def __init__(self, waiter):
        self.waiter = waiter

class Waiter():
    def informChef(self, chef: Chef, message):
        print("Informing the chef to prepare the order")
        
class SandwitchChef(Chef):
    def grillBread(self):
        print("SandwitchChef: Grilling a tasty sandwitch.")
        self.waiter.informChef(self, "Sandwitch is grilled and ready to be served.")

    def assemble(self):
        print("SandwitchChef: Decorating the sandwitch with fresh veggies.")
        self.waiter.informChef(self, "Sandwitch has been decorated and is ready to serve.")

class SoupChef(Chef):
    def prepareSoup(self):
        print("SoupChef: Preparing a tasty soup.")
        self.waiter.informChef(self, "Soup is ready to be served.")

    def decorateSoup(self):
        print("SandwitchChef: Decorating the soup with fresh veggies.")
        self.waiter.informChef(self, "Soup has been decorated and is ready to serve.")

class OurWaiter(Waiter):
    def __init__(self, soupchef, sandwitchchef):
        self.soupchef = soupchef
        self.sandwitchchef = sandwitchchef

    def informChef(self, chef, message):
        print("OurWaiter : informchef")
        
def main():
    waiter = OurWaiter(soupchef = None, sandwitchchef = None)
    
    soupchef = SoupChef(waiter = waiter)
    sandwitchchef = SandwitchChef(waiter = waiter)
    
    waiter = OurWaiter(soupchef = soupchef, sandwitchchef = sandwitchchef)
    
    soupchef.prepareSoup()
    soupchef.decorateSoup()

    sandwitchchef.grillBread()
    sandwitchchef.assemble()
    
    waiter.informChef(soupchef, "All orders delivered")
    waiter.informChef(sandwitchchef, "All orders delivered")

if __name__ == "__main__":
    main()