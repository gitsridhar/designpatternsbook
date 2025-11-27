package proxy.java;

public class BurgerProxy implements Burger {
    private Burger burger;

    public BurgerProxy(Burger burger) {
        this.burger = burger;
    }

    private boolean tasteGood() {
        // Simulate a taste test
        return true; // Assume the taste test is always passed for simplicity
    }

    private boolean isHealthy() {
        // Simulate a health check
        return true; // Assume it's always healthy for simplicity
    }

    @Override
    public void prepare() {
        System.out.println("Starting preparation through Proxy...");
        if (tasteGood()) {
            System.out.println("Taste test succeeded! Prepare the burger.");

            if (isHealthy()) {
                System.out.println("Taste test succeeded and burger is healthy! Prepare.");
                burger.prepare();
                return;
            }
            return;
        }
    }
}
