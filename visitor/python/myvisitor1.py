class RestaurantVisitor:
    def visit_restaurant(self, restaurant):
        print(f"Visiting restaurant: {restaurant.name}")
        for menu_item in restaurant.menu_items:
            self.visit_menu_item(menu_item)

    def visit_menu_item(self, menu_item):
        print(f"  Menu Item: {menu_item.name}, Price: {menu_item.price}")
class MenuItem:
    def __init__(self, name, price):
        self.name = name
        self.price = price 
class Restaurant:
    def __init__(self, name, menu_items):
        self.name = name
        self.menu_items = menu_items
if __name__ == "__main__":
    menu_items = [
        MenuItem("Pasta", 12.99),
        MenuItem("Pizza", 15.49),
        MenuItem("Salad", 9.99)
    ]
    restaurant = Restaurant("Italian Bistro", menu_items)
    visitor = RestaurantVisitor()
    visitor.visit_restaurant(restaurant)    
