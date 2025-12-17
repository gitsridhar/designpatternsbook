class Dish():
    def __init__(self, state):
        self.state = state
        
    def getState(self):
        return self.state
    
class Chef():
    def __init__(self, waiter):
        self.waiter = waiter
        self.dishes = []
        
    def backup(self):
        self.dishes.append(self.waiter.saveToMemento())
    
    def undo(self):
        if len(self.dishes) > 0:
            dish = self.dishes.pop()
            self.waiter.restoreFromMemento(dish)
            
class Waiter():
    def __init__(self, state):
        self.state = state

    def getState(self):
        return self.state

    def saveToMemento(self):
        dish = Dish(state = self.state)
        return dish
    
    def restoreFromMemento(self, dish):
        self.state = dish.getState()
        
def main():
    waiter = Waiter(state = "InitialState")
    chef = Chef(waiter)
    
    print("Current State : " + waiter.getState())
    chef.backup()
    
    waiter = Waiter(state = "AfterState")
    print("Current State : " + waiter.getState())
    
    chef.undo()
    print("Restored State : " + waiter.getState())

if __name__ == "__main__":
    main()