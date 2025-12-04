package main

type Restaurant struct {
	hotFood  *HotFood
	coldFood *ColdFood
}

func NewRestaurant(hotFoodName, coldFoodName string) *Restaurant {
	return &Restaurant{
		hotFood:  NewHotFood(hotFoodName),
		coldFood: NewColdFood(coldFoodName),
	}
}

func (r *Restaurant) ServeHotFood() []string {
	var results []string
	results = append(results, r.hotFood.Unwrap())
	results = append(results, r.hotFood.Clean())
	results = append(results, r.hotFood.Cook())
	results = append(results, r.hotFood.Prepare())
	results = append(results, r.hotFood.Serve())
	return results
}

func (r *Restaurant) ServeColdFood() []string {
	var results []string
	results = append(results, r.coldFood.WashAndRinse())
	results = append(results, r.coldFood.Wrap())
	results = append(results, r.coldFood.Freeze())
	results = append(results, r.coldFood.Prepare())
	results = append(results, r.coldFood.Serve())
	return results
}
