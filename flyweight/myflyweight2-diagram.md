```mermaid
classDiagram
  direction LR

  class Fruit

  class FruitFactory

  class Apple

  class Restaurant {
    +EatApple()
  }

  Fruit <|-- Apple

  note "Entry point: main()"
```
