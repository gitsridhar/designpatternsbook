```mermaid
classDiagram
  direction LR

  class FoodProcessor

  class Chopper

  class ChopperAdapter

  FoodProcessor <|-- ChopperAdapter

  note "Entry point: main()"
```
