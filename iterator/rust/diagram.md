```mermaid
classDiagram
  direction LR
  class Dinner {
  }

  class WeekendDinner {
  }

  class Dish {
  }

  class Eating {
  }

  class RestaurantEating {
  }

  Dinner <|.. Dinner
  Eating <|.. Eating
  note "startup code: fn main()"
```
