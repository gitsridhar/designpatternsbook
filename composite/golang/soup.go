package main

type Soup struct {
	Dish
}

func (s *Soup) Prepare() string {
	return "Preparing a warm and comforting soup."
}
