package main

func main() {
	// Initialize concrete restaurants
	res1 := &Restaurant1{Name: "The Burger Shack"}
	res2 := &Restaurant2{Name: "The Juice Bar"}

	// Initialize concrete visitors
	v1 := &Visitor1{}
	v2 := &Visitor2{}

	// Execute visitor pattern
	res1.Accept(v1)
	res2.Accept(v2)
}
