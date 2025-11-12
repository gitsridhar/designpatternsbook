class KitchenMeta(type):
    _instances = {}

    def __call__(cls, *args, **kwargs):
        if cls not in cls._instances:
            cls._instances[cls] = super().__call__(*args, **kwargs)
        return cls._instances[cls]

class MyKitchen(metaclass=KitchenMeta):
    def __init__(self, data):
        self.data = data

# Usage
obj1 = MyKitchen("Data A")
obj2 = MyKitchen("Data B")

print(obj1 is obj2) # Output: True
print(obj1.data) # Output: Data A