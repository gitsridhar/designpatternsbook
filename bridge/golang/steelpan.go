package main

type SteelPan struct {
	brand string
}

func NewSteelPan(brand string) SteelPan {
	return SteelPan{brand: brand}
}

func (sp SteelPan) Cook() string {
	return "cooking in a " + sp.brand + " steel pan"
}
