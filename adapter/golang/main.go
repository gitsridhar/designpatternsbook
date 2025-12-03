package main

func main() {
	foodProcessor := FoodProcessor{brand: "KitchenMaster"}
	result1 := foodProcessor.process("vegetables")
	println(result1)

	chopperAdapter := newChopperAdapter("QuickChop")
	result2 := chopperAdapter.process("fruits")
	println(result2)
}
