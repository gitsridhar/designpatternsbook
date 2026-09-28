```mermaid
classDiagram
  direction LR
  class BasicChef {
  }

  class FryingChef {
  }

  class MyChain {
  }

  class CollectingIngredientsChef {
  }

  class MasterChef {
  }

  class Chef {
  }

  class BoilingChef {
  }

  Chef <|.. BasicChef
  Chef <|.. FryingChef
  Chef <|.. CollectingIngredientsChef
  Chef <|.. MasterChef
  Chef <|.. BoilingChef
  note "startup code: main()"
```
