package mediator.java;

public class SoupChef extends Chef {
    public SoupChef(Waiter waiter) {
        super(waiter);
    }

    public void prepareSoup() {
        System.out.println("SoupChef: Preparing a delicious soup.");
        waiter.informChef(this, "Soup is ready to be served.");
    }

    public void decorateSoup() {
        System.out.println("SoupChef: Decorating the soup with fresh herbs.");
        waiter.informChef(this, "Soup has been decorated and is ready to serve.");
    }
}
