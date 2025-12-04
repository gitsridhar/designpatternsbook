package main

type HotFood struct {
	name string
}

func NewHotFood(name string) *HotFood {
	return &HotFood{name: name}
}

func (h *HotFood) Unwrap() string {
	return "Unwrapping hot food: " + h.name
}

func (h *HotFood) Clean() string {
	return "Cleaning hot food: " + h.name
}

func (h *HotFood) Cook() string {
	return "Cooking hot food: " + h.name
}

func (h *HotFood) Prepare() string {
	return "Preparing hot food: " + h.name
}
func (h *HotFood) Serve() string {
	return "Serving hot food: " + h.name
}
