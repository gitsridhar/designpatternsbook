```mermaid
sequenceDiagram
  actor App
  participant ColdFood
  participant HotFood
  participant Restaurant
  App->>App: start main()
  App->>ColdFood: create and use instance
  ColdFood-->>App: return result
  App->>HotFood: trigger behavior
  HotFood-->>App: return result
  App->>Restaurant: trigger behavior
```
