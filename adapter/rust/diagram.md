```mermaid
classDiagram
  direction LR
  class Chopper {
  }

  class NewFoodProcessor {
  }

  class Processor {
  }

  class FoodProcessor {
  }

  Processor <|.. Processor
  note "startup code: fn main()"
```
