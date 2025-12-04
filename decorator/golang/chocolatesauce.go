package main

type ChocolateSauce struct {
	sauce Sauce
}

func NewChocolateSauce(sauce Sauce) *ChocolateSauce {
	return &ChocolateSauce{
		sauce,
	}
}

func (c *ChocolateSauce) dip() {
	println("Adding chocolate sauce to the food and dipping it")
	c.sauce.dip()
}
