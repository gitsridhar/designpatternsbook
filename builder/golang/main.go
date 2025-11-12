package main

import "fmt"

func main() {
	fc := FruitConsumer{}
	fmt.Println(fc.consume())

	vc := VegetableConsumer{}
	fmt.Println(vc.consume())
}
