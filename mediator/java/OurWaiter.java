package mediator.java;

public class OurWaiter extends Waiter {
   private SoupChef soupChef;
   private SandwitchChef sandwitchChef;

    public OurWaiter(SoupChef soupChef, SandwitchChef sandwitchChef) {
         this.soupChef = soupChef;
         this.sandwitchChef = sandwitchChef;
    }
    
    @Override
    public void informChef(Chef chef, String message) {
        System.out.println("OurWaiter: " + message);
    }
}
