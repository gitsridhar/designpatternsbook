package main

func main() {
	steelPan := NewSteelPan("Premium")
	potatoFry := NewPotatoFry(steelPan)

	println(potatoFry.Prepare())
}
