package main

type SaltAndPepper struct {
	Dish
}

func (sp *SaltAndPepper) Prepare() string {
	return "Adding salt and pepper to taste."
}
