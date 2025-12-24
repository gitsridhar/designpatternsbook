package main

// Waiter represents another Originator
type Waiter struct {
	ActiveTable int
}

// WaiterMemento stores the Waiter state
type WaiterMemento struct {
	activeTable int
}

func (w *Waiter) Save() *WaiterMemento {
	return &WaiterMemento{activeTable: w.ActiveTable}
}

func (w *Waiter) Restore(m *WaiterMemento) {
	w.ActiveTable = m.activeTable
}
