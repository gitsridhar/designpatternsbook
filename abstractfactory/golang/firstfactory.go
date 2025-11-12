package main

type FirstFactory struct {
	IMyFactory
}

func (f *FirstFactory) consumeFruit() IFruit {
	return &Apple{
		name: "Seattle Apple",
	}
}

func (f *FirstFactory) consumeVegetable() IVegetable {
	return &Potato{
		name: "Idaho Potato",
	}
}
