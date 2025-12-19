class Pizza:
    def prepare(self):
        self.make_dough()
        self.add_sauce()
        self.add_toppings()
        self.bake()

    def make_dough(self):
        print("Making dough")

    def add_sauce(self):
        print("Adding sauce")

    def add_toppings(self):
        raise NotImplementedError("Subclasses must implement add_toppings method")

    def bake(self):
        print("Baking pizza")
        
    def slice(self):
        print("Slicing pizza")
        
    def box(self):
        print("Boxing pizza")

class PepperoniPizza(Pizza):
    def add_toppings(self):
        print("Adding pepperoni toppings")

class CheesePizza(Pizza):
    def add_toppings(self):
        print("Adding cheese toppings")

def main():
    pepperoni_pizza = PepperoniPizza()
    pepperoni_pizza.prepare()
    pepperoni_pizza.slice()
    pepperoni_pizza.box()

    cheese_pizza = CheesePizza()
    cheese_pizza.prepare()
    cheese_pizza.slice()
    cheese_pizza.box()

if __name__ == "__main__":
    main()
