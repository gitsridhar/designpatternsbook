package composite.java;

public class MyComposite {
    public static void main(String[] args) {
        Dish shaker = new SaltAndPepper();

        Dish dinner = new Serving();
        Dish appetizer = new Serving();
        Dish soup = new Soup();
        Dish fruitsalad = new FruitSalad();
        Dish maindish = new MainDish();

        if (appetizer.isComposite()) {
            System.out.println("Appetizer is a composite dish.");
    
            appetizer.addDish(soup);
            appetizer.addDish(fruitsalad);
        }

        Dish maincourse = new Serving();
        maincourse.addDish(maindish);

        dinner.addDish(shaker);
        dinner.addDish(appetizer);
        dinner.addDish(maincourse);

        System.out.println("Dinner consists of:");
        System.out.println(dinner.prepare());
    }
}
