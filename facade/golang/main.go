package main

func main() {
	coldFood := NewColdFood("Salad")
	hotFood := NewHotFood("Steak")

	restaurant := NewRestaurant(hotFood.name, coldFood.name)

	hotFoodResults := restaurant.ServeHotFood()
	for _, result := range hotFoodResults {
		println(result)
	}

	coldFoodResults := restaurant.ServeColdFood()
	for _, result := range coldFoodResults {
		println(result)
	}
}
