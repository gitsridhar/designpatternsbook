
class FoodProcessor:
    def process(self, food_item):
        return f"Processing {food_item} in myadapter."


class Chopper:
    def chop(self, food_item):
        return f"Chopping {food_item} in myadapter."

class ChopperAdapter(FoodProcessor, Chopper):
    def process(self, food_item):
        return self.chop(food_item)

if __name__ == "__main__":
    fp = FoodProcessor()
    fp.process("carrot")
    
    chopper = Chopper()
    chopper.chop("onion")
    
    chopper_adapter = ChopperAdapter()
    chopper_adapter.process("carrot")
    chopper_adapter.chop("onion")
