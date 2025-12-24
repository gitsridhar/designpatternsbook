package main

import "fmt"

// GlobalStateSnapshot captures the state of multiple objects
type GlobalStateSnapshot struct {
	dishState   *DishMemento
	waiterState *WaiterMemento
}

type Chef struct {
	Dish    *Dish
	Waiter  *Waiter
	history []*GlobalStateSnapshot
}

func (c *Chef) Backup() {
	fmt.Println("Chef: Saving current kitchen state...")
	snapshot := &GlobalStateSnapshot{
		dishState:   c.Dish.Save(),
		waiterState: c.Waiter.Save(),
	}
	c.history = append(c.history, snapshot)
}

func (c *Chef) Undo() {
	if len(c.history) == 0 {
		fmt.Println("Chef: Nothing to undo.")
		return
	}

	fmt.Println("Chef: Restoring to previous state...")
	// Pop last item from stack
	lastIndex := len(c.history) - 1
	snapshot := c.history[lastIndex]
	c.history = c.history[:lastIndex]

	c.Dish.Restore(snapshot.dishState)
	c.Waiter.Restore(snapshot.waiterState)
}
