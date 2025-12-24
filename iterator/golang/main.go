package main

import "fmt"

func main() {
	// Initialize the aggregate (WeekendDinner)
	weekendMenu := &WeekendDinner{}
	weekendMenu.AddDish("Bruschetta")
	weekendMenu.AddDish("Lasagna")
	weekendMenu.AddDish("Tiramisu")

	// Create the iterator (Eating)
	iterator := weekendMenu.CreateDinner()

	fmt.Println("Starting the weekend dinner course:")

	// Traverse using the iterator
	for iterator.HasNextDish() {
		iterator.Eat()
	}
}
