package templatemethod.java;

public class CheesePizza extends Pizza {
    @Override
    protected void addToppings() {
        System.out.println("Adding cheese toppings");
    }
}
