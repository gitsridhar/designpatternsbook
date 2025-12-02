package observer.java;

public class MyObserver {
    public static void main(String[] args) {
        Chef chef = new Chef();
        Waiter waiter1 = new Waiter(chef);

        chef.prepareDish("Pasta");
        
        waiter1.stopObserving();
        
        chef.prepareDish("Pizza");
    }
}
