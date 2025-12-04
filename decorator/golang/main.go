package main

func main() {
	strawberrywithhotsauce := NewHotSauce(Sauce{&Strawberry{}})
	strawberrywithhotsauce.dip()

	strawberryWithChocolateAndHotSauce := NewChocolateSauce(Sauce{&Strawberry{}})
	strawberryWithChocolateAndHotSauce.dip()
}
