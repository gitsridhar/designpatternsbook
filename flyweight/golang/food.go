package main

type Food struct {
	name     string
	price    float64
	foodType FoodType
}

func NewFood(name string, price float64, foodType FoodType) *Food {
	return &Food{
		name:     name,
		price:    price,
		foodType: foodType,
	}
}

func (f *Food) GetName() string {
	return f.name
}

func (f *Food) GetPrice() float64 {
	return f.price
}

func (f *Food) GetFoodType() FoodType {
	return f.foodType
}

func (f *Food) Serve() string {
	return "Serving " + f.name + " of type " + f.foodType.GetType()
}
