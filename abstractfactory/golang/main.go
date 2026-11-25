package main

import "fmt"

func main() {
	firstfact := FirstFactory{}
	fmt.Println(firstfact.consumeFruit().consume(""))
	fmt.Println(firstfact.consumeVegetable().consume(""))

	secondfact := SecondFactory{}
	fmt.Println(secondfact.consumeFruit().consume(""))
	fmt.Println(secondfact.consumeVegetable().consume(""))
}
