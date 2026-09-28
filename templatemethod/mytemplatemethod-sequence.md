```mermaid
sequenceDiagram
  actor App
  participant Pizza
  participant CheesePizza
  participant PepperoniPizza
  App->>App: start main()
  App->>Pizza: create and use instance
  Pizza-->>App: return result
  App->>CheesePizza: trigger behavior
  CheesePizza-->>App: return result
  App->>PepperoniPizza: trigger behavior
```
