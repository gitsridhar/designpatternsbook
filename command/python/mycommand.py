from abc import ABC, abstractmethod

class Action(ABC):
    @abstractmethod
    def doit(self):
        pass
    
class Customer():
    def orderFood(self):
        print("Customer ordering food")
    
    def makePayment(self):
        print("Customer making payment")

class CustomerInteraction(Action):
    def __init__(self, customer, interactiontype):
        self.customer = customer
        self.interactiontype = interactiontype

    def doit(self):
        if self.interactiontype == "order":
            self.customer.orderFood()
        elif self.interactiontype == "payment":
            self.customer.makePayment()
        else:
            print("Unknown interaction type")

class Peel(Action):
    def __init__(self, vegetable):
        self.vegetable = vegetable
    
    def doit(self):
        print("Peeling the vegetable")

class Waiter():
    def __init__(self, action1, action2):
        self.action1 = action1
        self.action2 = action2

    def executeActions(self):
        if self.action1:
            self.action1.doit()
        if self.action2:
            self.action2.doit()

def main():
    waiter = Waiter(Peel(vegetable = "potato"),
                    Peel(vegetable = "carrot"))
    
    waiter.executeActions()
    
    customer = Customer()
    customer.orderFood()
    customer.makePayment()
    
    customerinteraction1 = CustomerInteraction(customer, interactiontype = "order")
    customerinteraction2 = CustomerInteraction(customer, interactiontype = "payment")
    
    waiter2 = Waiter(customerinteraction1, customerinteraction2)
    waiter2.executeActions()

if __name__ == "__main__":
    main()