```mermaid
classDiagram
  direction LR
  class NewFoodProcessor {
  }

  class Chopper {
  }

  class FoodProcessor {
  }

  class MyAdapter {
  }

  Chopper <|.. NewFoodProcessor
  note "startup code: main()"
```
