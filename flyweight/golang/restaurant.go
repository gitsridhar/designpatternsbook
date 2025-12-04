package main

type Restaurant struct {
	foods []*Food
}

func (r *Restaurant) AddFood(cost float64, name string,
	cusine string, category string) {

	foodFactory := NewFoodFactory()
	foodType := foodFactory.GetFoodType(cusine, category)
	food := &Food{
		price:    cost,
		name:     name,
		foodType: foodType,
	}

	r.foods = append(r.foods, food)
}

func (r *Restaurant) ServeFood() {
	for _, food := range r.foods {
		println(food.Serve())
	}
}
