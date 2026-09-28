```mermaid
classDiagram
  direction LR
  class Client {
  }

  class AppleConsumer {
  }

  class Consumer {
  }

  class Director {
  }

  class Fruit {
  }

  class Apple {
  }

  Consumer <|.. AppleConsumer
  Fruit <|.. Apple
  note "startup code: main()"
```
