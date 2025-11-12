package main

type Apple struct {
	Fruit
}

func consume() IFruit {
	return &Apple{
		Fruit: Fruit{
			name: "Washington Apple",
		},
	}
}
