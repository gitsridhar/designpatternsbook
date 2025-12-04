package main

type BurgerProxy struct {
	burger Burger
	name   string
	price  float64
	isVeg  bool
}

func NewBurgerProxy(name string, price float64, isVeg bool) *BurgerProxy {
	return &BurgerProxy{
		name:  name,
		price: price,
		isVeg: isVeg,
	}
}

func (bp *BurgerProxy) ServeBurger() string {
	if bp.burger == nil {
		if bp.IsHealthy() {
			println("Preparing a healthy burger...")
		} else {
			println("Preparing a regular burger...!")
		}

		if bp.isVeg {
			bp.burger = &VegBurger{
				name:  bp.name,
				price: bp.price,
			}
		} else {
			bp.burger = &NonVegBurger{
				name:  bp.name,
				price: bp.price,
			}
		}
	}
	return bp.burger.ServeBurger()
}

func (bp *BurgerProxy) IsHealthy() bool {
	return true
}
