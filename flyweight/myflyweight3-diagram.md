```mermaid
classDiagram
  direction LR

  class TreeType

  class TreeFactory

  class Tree {
    +Tree()
  }

  class Forest {
    +plantTree()
    +displayTrees()
  }

  note "Entry point: main()"
```
