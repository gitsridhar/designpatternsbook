package main

type Chopper struct {
	brand string
}

func (c *Chopper) chop(ingredient string) string {
	return "Chopping " + ingredient + " with " + c.brand + " chopper"
}
