package templatemethod.java;

public class MyTemplateMethod {
    public static void main(String[] args) {
        Pizza cheesePizza = new CheesePizza();
        System.out.println("Making a Cheese Pizza:");
        cheesePizza.templateMethod();

        System.out.println();

        Pizza pepperoniPizza = new PepperoniPizza();
        System.out.println("Making a Pepperoni Pizza:");
        pepperoniPizza.templateMethod();
    }
}
