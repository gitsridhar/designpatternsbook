```mermaid
sequenceDiagram
  actor App
  participant Chef
  participant BasicChef
  participant CollectIngredientsChef
  participant BoilingChef
  participant FryingChef
  participant MasterChef
  App->>App: start main()
  App->>Chef: create and use instance
  Chef-->>App: return result
  App->>BasicChef: trigger behavior
  BasicChef-->>App: return result
  App->>CollectIngredientsChef: trigger behavior
  CollectIngredientsChef-->>App: return result
  App->>BoilingChef: trigger behavior
  BoilingChef-->>App: return result
  App->>FryingChef: trigger behavior
  FryingChef-->>App: return result
  App->>MasterChef: trigger behavior
```
