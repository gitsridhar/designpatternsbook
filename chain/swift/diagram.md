```mermaid
classDiagram
  direction LR
  class defining {
  }
  class Chef {
  }
  class CollectingIngredientsChef {
  }
  class BoilingChef {
  }
  class FryingChef {
  }
  class BasicChef {
  }
  class MasterChef {
  }
  Chef <|-- CollectingIngredientsChef
  Chef <|-- BoilingChef
  Chef <|-- FryingChef
  Chef <|-- BasicChef
  Chef <|-- MasterChef
  note "Top-level startup statements"
```
