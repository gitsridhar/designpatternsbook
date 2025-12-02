package mediator.java;

public class SandwitchChef extends Chef {
    public SandwitchChef(Waiter waiter) {
        super(waiter);
    }

    public void grillBread() {
        System.out.println("SandwitchChef: Grilling a tasty sandwitch.");
        waiter.informChef(this, "Sandwitch is grilled and ready to be served.");
    }

    public void assemble() {
        System.out.println("SandwitchChef: Decorating the sandwitch with fresh veggies.");
        waiter.informChef(this, "Sandwitch has been decorated and is ready to serve.");
    }
    
}
