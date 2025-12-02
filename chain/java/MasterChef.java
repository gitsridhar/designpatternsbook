package chain.java;

public class MasterChef implements Chef {
    private Chef nextChef;

    @Override
    public Chef setNextChef(Chef chef) {
        this.nextChef = chef;
        return chef;
    }

    @Override
    public void cook(String dish) {
        System.out.println("MasterChef is overseeing the cooking of: " + dish);
        if (nextChef != null) {
            nextChef.cook(dish);
        } else {
            System.out.println("No chef available to cook: " + dish);
        }
    }
}
