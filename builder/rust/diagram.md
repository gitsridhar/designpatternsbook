```mermaid
classDiagram
  direction LR
  class Consumer {
  }

  class FruitConsumer {
  }

  class VegetableConsumer {
  }

  class Director {
  }

  Consumer <|.. Consumer
  note "startup code: fn main()"
```
