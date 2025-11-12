package main

import "fmt"

func main() {
	firstfact := FirstFactory{}
	fmt.Println(firstfact.consumeFruit())
	fmt.Println(firstfact.consumeVegetable())

	secondfact := SecondFactory{}
	fmt.Println(secondfact.consumeFruit())
	fmt.Println(secondfact.consumeVegetable())
}
