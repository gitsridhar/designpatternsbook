package main

type FoodType struct {
	cusine   string
	category string
}

func NewFoodType(cusine, category string) FoodType {
	return FoodType{
		cusine:   cusine,
		category: category,
	}
}

func (ft FoodType) GetType() string {
	return ft.cusine + " - " + ft.category
}

func (ft FoodType) GetCusine() string {
	return ft.cusine
}

func (ft FoodType) GetCategory() string {
	return ft.category
}

func (ft FoodType) String() string {
	return "Cusine: " + ft.cusine + ", Category: " + ft.category
}

func (ft FoodType) Consume() string {
	return "Consuming " + ft.cusine + " of category " + ft.category
}
