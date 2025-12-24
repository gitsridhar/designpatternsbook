package main

// OrderFood is the Context
type OrderFood struct {
	State OrderPhase
}

func (o *OrderFood) SetState(state OrderPhase) {
	o.State = state
}

func (o *OrderFood) AddItem() {
	o.State.AddItem()
}

func (o *OrderFood) Confirm() {
	o.State.Confirm()
}
