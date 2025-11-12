package main

type PrototypeFactory struct {
}

func (pf *PrototypeFactory) createThing(thing string) Prototype {
	if thing == "fruit" {
		fp := FruitPrototype{}
		return fp.clone()
	}

	if thing == "vegetable" {
		vp := VegetablePrototype{}
		return vp.clone()
	}

	return nil
}
