package main

type Sauce struct {
	Food Food
}

func NewSauce(food Food) *Sauce {
	return &Sauce{
		Food: food,
	}
}

func (s *Sauce) dip() {
	println("Adding sauce to the food and dipping it")
	s.Food.dip()
}
