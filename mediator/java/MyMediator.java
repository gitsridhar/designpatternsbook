package mediator.java;

public class MyMediator {
    public static void main(String[] args) {
        OurWaiter waiter = new OurWaiter(null, null);
        SoupChef soupChef = new SoupChef(waiter);
        SandwitchChef sandwitchChef = new SandwitchChef(waiter);
        
        // Setting chefs in waiter
        waiter = new OurWaiter(soupChef, sandwitchChef);
        
        // Chefs preparing food
        soupChef.prepareSoup();
        soupChef.decorateSoup();
        sandwitchChef.grillBread();
        sandwitchChef.assemble();

        waiter.informChef(soupChef, "All orders are delivered.");
        waiter.informChef(sandwitchChef, "All orders are delivered.");
    }
}
