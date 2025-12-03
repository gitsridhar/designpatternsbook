package main

type FoodProcessor struct {
	brand string
}

func (fp *FoodProcessor) process(food string) string {
	return "Processing " + food + " with " + fp.brand + " food processor"
}
