class Burger:
    def __init__(self, size, cheese=False, pepperoni=False, 
                 lettuce=False, tomato=False):
        self.size = size
        self.cheese = cheese
        self.pepperoni = pepperoni
        self.lettuce = lettuce
        self.tomato = tomato

    def __str__(self):
        toppings = []
        if self.cheese:
            toppings.append("cheese")
        if self.pepperoni:
            toppings.append("pepperoni")
        if self.lettuce:
            toppings.append("lettuce")
        if self.tomato:
            toppings.append("tomato")
        toppings_str = ", ".join(toppings) if toppings else "no toppings"
        return f"{self.size} burger with {toppings_str}"
    
    def prepare(self):
        print(f"Preparing a {self.size} burger.")
        if self.cheese:
            print("Adding cheese.")
        if self.pepperoni:
            print("Adding pepperoni.")
        if self.lettuce:
            print("Adding lettuce.")
        if self.tomato:
            print("Adding tomato.")
        print("Burger is ready!")

class VegBurger(Burger):
    def __init__(self, size, cheese=False, lettuce=False, tomato=False):
        self.burger = Burger(size, cheese=cheese, pepperoni=False, 
                             lettuce=lettuce, tomato=tomato)

    def __str__(self):
        return str(self.burger)
    
    def prepare(self):
        print("Preparing a vegetarian burger.")
        
        
class VegBurgerProxy (Burger):
    def __init__(self, size, cheese=False, lettuce=False, tomato=False):
        self.veg_burger = VegBurger(size, cheese=cheese, lettuce=lettuce, tomato=tomato)

    def __str__(self):
        return str(self.veg_burger)
    
    def tastesGood(self):
        print("Proxy: Checking if the vegetarian burger tastes good.")
        # Simulate taste check
        print("Proxy: The vegetarian burger tastes great!")
        return True
    
    def isReady(self):
        print("Proxy: Checking if the vegetarian burger is ready.")
        # Simulate readiness check
        print("Proxy: The vegetarian burger is ready!")
        return True
    
    def prepare(self):
        print("Proxy: Starting preparation of vegetarian burger.")
        self.veg_burger.prepare()
        if self.isReady() and self.tastesGood():
            print("Proxy: Vegetarian burger preparation complete.")
    
def main():
    
    veg_burger = VegBurger(size="Medium", cheese=True, lettuce=True, tomato=False)
    veg_burger.prepare()

    veg_burger_proxy = VegBurgerProxy(size="Large", cheese=True, lettuce=True, tomato=True)
    veg_burger_proxy.prepare()
    
if __name__ == "__main__":
    main()