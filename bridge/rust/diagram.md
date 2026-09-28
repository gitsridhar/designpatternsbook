```mermaid
classDiagram
  direction LR
  class CopperPan {
  }

  class Pan {
  }

  class Food {
  }

  class PotatoFry {
  }

  class SteelPan {
  }

  Food <|.. Food
  Pan <|.. Pan
  note "startup code: fn main()"
```
