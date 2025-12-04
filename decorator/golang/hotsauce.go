package main

type HotSauce struct {
	sauce Sauce
}

func NewHotSauce(sauce Sauce) *HotSauce {
	return &HotSauce{
		sauce,
	}
}

func (h *HotSauce) dip() {
	println("Adding hot sauce to the food and dipping it")
	h.sauce.dip()
}
