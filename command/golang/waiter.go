package main

// Waiter acts as the invoker
type Waiter struct {
	actions []Action
}

func (w *Waiter) AddAction(a Action) {
	w.actions = append(w.actions, a)
}

func (w *Waiter) ExecuteActions() {
	for _, action := range w.actions {
		action.DoIt()
	}
	// Clear actions after execution
	w.actions = nil
}
