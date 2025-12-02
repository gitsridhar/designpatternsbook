class ColdFood:
    def washRinse(self):
        print("Washing and rinsing cold food.")
    
    def wrap(self):
        print("Wrapping cold food.")
        
    def freeze(self):
        print("Freezing cold food.")
        
class HotFood:  
    def preheat(self):
        print("Preheating hot food.")
    
    def cook(self):
        print("Cooking hot food.")
        
    def serve(self):
        print("Serving hot food.")

class RestaurantFacade:
    def __init__(self):
        self.cold_food = ColdFood()
        self.hot_food = HotFood()
    
    def prepareColdDish(self):
        self.cold_food.washRinse()
        self.cold_food.wrap()
        self.cold_food.freeze()
    
    def prepareHotDish(self):
        self.hot_food.preheat()
        self.hot_food.cook()
        self.hot_food.serve()
        
    def prepareFullMeal(self):
        print("Preparing full meal:")
        self.prepareColdDish()
        self.prepareHotDish()
    
def main():
    restaurant = RestaurantFacade()
    restaurant.prepareColdDish()
    print("---")
    restaurant.prepareHotDish()
    print("---")
    restaurant.prepareFullMeal()

if __name__ == "__main__":
    main()