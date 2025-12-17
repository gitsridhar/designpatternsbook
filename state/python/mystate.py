from abc import ABC, abstractmethod

class OrderPhase():
    def setOrderFood(self, orderfood):
        self.orderfood = orderfood
        
    def startOrder():
        pass
    
    def endOrder():
        pass
    
    def deliverOrder():
        pass
    
class OrderFood():
    def __init__(self, orderphase):
        self.orderphase = orderphase
        self.switchOrderPhase(orderphase)
        
    def switchOrderPhase(self, orderphase):
        self.orderphase.setOrderFood(self)
        
    def startOrder(self):
        self.orderphase.startOrder()
        
    def endOrder(self):
        self.orderphase.endOrder()
        
    def deliverOrder(self):
        self.orderphase.deliverOrder()
        
class StartOrderPhase(OrderPhase):
    def startOrder(self):
        print("StartOrderPhase : startOrder")

    def endOrder(self):
        print("StartOrderPhase : endOrder")

    def deliverOrder(self):
        print("StartOrderPhase : deliverOrder")
        self.orderfood.switchOrderPhase(EndOrderPhase())
        
class EndOrderPhase(OrderPhase):
    def startOrder(self):
        print("StartOrderPhase : startOrder")

    def endOrder(self):
        print("StartOrderPhase : endOrder")

    def deliverOrder(self):
        print("EndOrderPhase : deliverOrder")
        self.orderfood.switchOrderPhase(ReadyOrderPhase())

class ReadyOrderPhase(OrderPhase):
    def startOrder(self):
        print("StartOrderPhase : startOrder")

    def endOrder(self):
        print("StartOrderPhase : endOrder")

    def deliverOrder(self):
        print("EndOrderPhase : deliverOrder")

def main():
    orderFood = OrderFood(StartOrderPhase())
    orderFood.startOrder()
    orderFood.deliverOrder()
    orderFood.endOrder()
    orderFood.endOrder()

if __name__ == "__main__":
    main()
