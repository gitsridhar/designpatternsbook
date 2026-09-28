```mermaid
sequenceDiagram
  actor App
  participant Burger
  participant VegBurger
  participant BurgerProxy
  App->>App: start main()
  App->>Burger: create and use instance
  Burger-->>App: return result
  App->>VegBurger: trigger behavior
  VegBurger-->>App: return result
  App->>BurgerProxy: trigger behavior
```
