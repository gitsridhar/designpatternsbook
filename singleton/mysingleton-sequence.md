```mermaid
sequenceDiagram
  actor App
  participant Singleton
  App->>App: start main()
  App->>Singleton: create and use instance
```
