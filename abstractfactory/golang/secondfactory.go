package main

type SecondFactory struct {
	IMyFactory
}

func (f *SecondFactory) consumeFruit() IFruit {
	return &Banana{
		name: "Florida Banana",
	}
}

func (f *SecondFactory) consumeVegetable() IVegetable {
	return &Beans{
		name: "Carolina Beans",
	}
}
