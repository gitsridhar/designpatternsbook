package observer.java;

public class Waiter extends Observer {

    private Chef chef;
    public Waiter(Chef chef) {
        this.chef = chef;
        this.chef.attach(this);
    }

    public void stopObserving() {
        chef.detach(this);
    }

    @Override
    public void update(String message) {
        System.out.println("Waiter received message: " + message);
    }
}