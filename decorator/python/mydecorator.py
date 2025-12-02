class Food:
    def dip(self) -> str:
        pass
    
class Strawberry(Food):
    def dip(self) -> str:
        return "Strawberry dipped in chocolate"

class Sauce(Food):
    def __init__(self, food: Food) -> None:
        self._food = food

    def dip(self) -> str:
        return self._food.dip()
        
class ChocolateSauce(Sauce):
    def dip(self) -> str:
        return f"{self._food.dip()} with chocolate sauce"
    
class HotSauce(Sauce):
    def dip(self) -> str:
        return f"{self._food.dip()} with hot sauce"
    
def main():
    strawberry = Strawberry()
    print(strawberry.dip())
    
    chocolate_strawberry = ChocolateSauce(strawberry)
    print(chocolate_strawberry.dip())
    
    hot_chocolate_strawberry = HotSauce(chocolate_strawberry)
    print(hot_chocolate_strawberry.dip())

if __name__ == "__main__":
    main()