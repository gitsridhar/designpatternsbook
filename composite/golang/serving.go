package main

type Serving struct {
	Dish
	listOfDishes []*Dish
}

func (s *Serving) Prepare() string {
	return "Preparing a complete serving."
}

func (s *Serving) AddDish(dish *Dish) {
	s.listOfDishes = append(s.listOfDishes, dish)
	dish.SetParent(&s.Dish)
}

func (s *Serving) RemoveDish(dish *Dish) {
	for i, d := range s.listOfDishes {
		if d == dish {
			s.listOfDishes = append(s.listOfDishes[:i], s.listOfDishes[i+1:]...)
			break
		}
	}
	dish.SetParent(nil)
}

func (s *Serving) IsComposite() bool {
	return true
}

func (s *Serving) PrepareAll() []string {
	var preparations []string
	for _, dish := range s.listOfDishes {
		preparations = append(preparations, dish.Prepare())
	}
	return preparations
}
