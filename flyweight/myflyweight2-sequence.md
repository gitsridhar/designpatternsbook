```mermaid
sequenceDiagram
  actor App
  participant Fruit
  participant FruitFactory
  participant Apple
  participant Restaurant
  App->>App: start main()
  App->>Fruit: create and use instance
  Fruit-->>App: return result
  App->>FruitFactory: trigger behavior
  FruitFactory-->>App: return result
  App->>Apple: trigger behavior
  Apple-->>App: return result
  App->>Restaurant: trigger behavior
```
