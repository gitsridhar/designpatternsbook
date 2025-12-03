package main

type PotatoFry struct {
	pan Pan
}

func NewPotatoFry(pan Pan) *PotatoFry {
	return &PotatoFry{pan: pan}
}

func (pf *PotatoFry) Prepare() string {
	return "Preparing potato fry with " + pf.pan.Cook()
}
