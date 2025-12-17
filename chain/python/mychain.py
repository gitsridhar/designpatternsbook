from abc import ABC, abstractmethod

class Chef(ABC):
    def __init__(self):
        self.nextchef = None

    @abstractmethod
    def setNextChef(self, chef):
        pass

    @abstractmethod
    def cook(self):
        pass


class BasicChef(Chef):
    def __init__(self):
        super().__init__()

    def setNextChef(self, chef):
        self.nextchef = chef
        return self.nextchef

    def cook(self):
        print("Basic Chef Cooking...")
        if self.nextchef:
            self.nextchef.cook()


class CollectingIngredientsChef(Chef):
    def __init__(self):
        super().__init__()

    def setNextChef(self, chef):
        self.nextchef = chef
        return self.nextchef

    def cook(self):
        print("Collecting Ingredients Chef Cooking...")
        if self.nextchef:
            self.nextchef.cook()


class BoilingChef(Chef):
    def __init__(self):
        super().__init__()

    def setNextChef(self, chef):
        self.nextchef = chef
        return self.nextchef

    def cook(self):
        print("Boiling Chef Cooking...")
        if self.nextchef:
            self.nextchef.cook()
            
class FryingChef(Chef):
    def __init__(self):
        super().__init__()

    def setNextChef(self, chef):
        self.nextchef = chef
        return self.nextchef

    def cook(self):
        print("Frying Chef Cooking...")
        if self.nextchef:
            self.nextchef.cook()
        
class MasterChef(Chef):
    def __init__(self):
        super().__init__()

    def setNextChef(self, chef):
        self.nextchef = chef
        return self.nextchef

    def cook(self):
        print("Master Chef Cooking...")
        if self.nextchef:
            self.nextchef.cook()

def main():
    basicChef = BasicChef()
    basicChef.cook()
    
    collectingIngredientsChef = CollectingIngredientsChef()
    boilingChef = BoilingChef()
    fryingChef = FryingChef()
    masterChef = MasterChef()

    basicChef.setNextChef(collectingIngredientsChef) \
        .setNextChef(boilingChef).setNextChef(fryingChef) \
        .setNextChef(masterChef)
    basicChef.cook()


if __name__ == "__main__":
    main()