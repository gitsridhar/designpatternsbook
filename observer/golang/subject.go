package main

// Subject defines the interface for adding and notifying observers.
type Subject interface {
	Register(o Observer)
	Deregister(o Observer)
	NotifyAll(orderName string)
}

// BaseSubject provides a reusable implementation of the Subject interface.
type BaseSubject struct {
	observers []Observer
}

func (s *BaseSubject) Register(o Observer) {
	s.observers = append(s.observers, o)
}

func (s *BaseSubject) Deregister(o Observer) {
	for i, observer := range s.observers {
		if observer == o {
			s.observers = append(s.observers[:i], s.observers[i+1:]...)
			break
		}
	}
}

func (s *BaseSubject) NotifyAll(orderName string) {
	for _, observer := range s.observers {
		observer.Update(orderName)
	}
}
