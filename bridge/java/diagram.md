```mermaid
classDiagram
  direction LR
  class MyBridge {
  }

  class Food {
  }

  class Pan {
  }

  class PotatoFry {
  }

  class SteelPan {
  }

  Food <|-- PotatoFry
  Pan <|.. SteelPan
  note "startup code: main()"
```
