package main

type FoodFactory struct {
	foodTypes map[string]FoodType
}

func NewFoodFactory() *FoodFactory {
	return &FoodFactory{
		foodTypes: make(map[string]FoodType),
	}
}

func (ff *FoodFactory) GetFoodType(cusine string, category string) FoodType {
	foodTypeName := cusine + "-" + category
	if foodType, exists := ff.foodTypes[foodTypeName]; exists {
		return foodType
	}
	var foodType FoodType = NewFoodType(cusine, category)
	ff.foodTypes[foodTypeName] = foodType
	return foodType
}
