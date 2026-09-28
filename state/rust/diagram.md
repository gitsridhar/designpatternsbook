```mermaid
classDiagram
  direction LR
  class OrderPhase {
  }

  class OrderFood {
  }

  class StartOrderPhase {
  }

  class ReadyOrderPhase {
  }

  class EndOrderPhase {
  }

  OrderPhase <|.. OrderPhase
  note "startup code: fn main()"
```
