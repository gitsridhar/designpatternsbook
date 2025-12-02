package templatemethod.java;

public class Pizza {
    public void templateMethod() {
        prepareDough();
        addSauce();
        addToppings();
        bake();
        slice();
        box();
    }

    protected void prepareDough() {
        System.out.println("Preparing the dough");
    }

    protected void addSauce() {
        System.out.println("Adding sauce");
    }

    protected void addToppings() {
        System.out.println("Adding toppings");
    }

    protected void bake() {
        System.out.println("Baking the pizza");
    }

    protected void slice() {
        System.out.println("Slicing the pizza");
    }

    protected void box() {
        System.out.println("Boxing the pizza");
    }
}
