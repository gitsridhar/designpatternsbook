package main

type ColdFood struct {
	name string
}

func NewColdFood(name string) *ColdFood {
	return &ColdFood{name: name}
}

func (c *ColdFood) WashAndRinse() string {
	return "Washing and rinsing cold food: " + c.name
}

func (c *ColdFood) Wrap() string {
	return "Wrapping cold food: " + c.name
}

func (c *ColdFood) Freeze() string {
	return "Freezing cold food: " + c.name
}

func (c *ColdFood) Prepare() string {
	return "Preparing cold food: " + c.name
}
func (c *ColdFood) Serve() string {
	return "Serving cold food: " + c.name
}
