```mermaid
classDiagram
  direction LR
  class FoodPreparation {
  }

  class Strategy {
  }

  class OpenStrategy {
  }

  class ClosedStrategy {
  }

  class StrategyInterface {
  }

  class OpenPanStrategy {
  }

  class ClosedPanStrategy {
  }

  Strategy <|.. Strategy
  StrategyInterface <|.. StrategyInterface
  note "startup code: fn main()"
```
