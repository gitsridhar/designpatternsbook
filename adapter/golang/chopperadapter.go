package main

type ChopperAdapter struct {
	chopper *Chopper
}

func newChopperAdapter(brand string) *ChopperAdapter {
	return &ChopperAdapter{
		chopper: &Chopper{brand: brand},
	}
}

func (ca *ChopperAdapter) process(food string) string {
	return ca.chopper.chop(food)
}
