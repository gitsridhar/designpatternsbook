package main

func main() {
	restaurant := Restaurant{}
	restaurant.AddFood(1, "Thin Pizza", "American", "Thin Crust")
	restaurant.AddFood(2, "Thick Pizza", "American", "Thick Crust")
	restaurant.AddFood(3, "Single Burger", "Australian", "Lettuce, Tomato")
	restaurant.AddFood(4, "Double Burger", "Australian", "Cheese, Bacon")

	restaurant.ServeFood()
}
