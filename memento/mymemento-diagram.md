```mermaid
classDiagram
  direction LR

  class Dish

  class Waiter {
    +Dish()
    +restore()
  }

  class Chef {
    +backup()
  }

  note "Entry point: main()"
```
