package main

func main() {
	// This is just a placeholder main function.
	saltandpepper := new(SaltAndPepper)
	dinner := new(Serving)
	appetizer := new(Serving)
	soup := new(Soup)
	fruitsalad := new(FruitSalad)
	maindish := new(MainDish)

	if appetizer.IsComposite() {
		appetizer.AddDish(&soup.Dish)
		appetizer.AddDish(&fruitsalad.Dish)
	}

	maincourse := new(Serving)
	if maincourse.IsComposite() {
		maincourse.AddDish(&maindish.Dish)
		maincourse.AddDish(&saltandpepper.Dish)
	}

	dinner.AddDish(&appetizer.Dish)
	dinner.AddDish(&maincourse.Dish)
	preparations := dinner.PrepareAll()
	for _, prep := range preparations {
		println(prep)
	}
}
